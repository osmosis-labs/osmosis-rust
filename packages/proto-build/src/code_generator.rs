use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use log::info;
use prost::Message;
use prost_types::FileDescriptorSet;
use walkdir::WalkDir;

use crate::{mod_gen, transform};

const DESCRIPTOR_FILE: &str = "descriptor.bin";

#[derive(Clone, Debug)]
pub struct CosmosProject {
    pub name: String,
    pub version: String,
    pub project_dir: String,

    /// determines which modules to include from the project
    /// empty vector means include all modules
    pub include_mods: Vec<String>,
}

pub struct CodeGenerator {
    project: CosmosProject,
    tonic_build_config: tonic_build::Builder,
    root: PathBuf,
    out_dir: PathBuf,
    tmp_build_dir: PathBuf,
    deps: Vec<CosmosProject>,
}

impl CodeGenerator {
    pub fn new(
        out_dir: PathBuf,
        // TODO: remove tmp_build_dir from constructor in favor of generated tmp dir
        tmp_build_dir: PathBuf,
        project: CosmosProject,
        deps: Vec<CosmosProject>,
    ) -> Self {
        let tonic_build_config = tonic_build::configure()
            .build_client(false)
            .build_server(false)
            .extern_path(".google.protobuf.Timestamp", "crate::shim::Timestamp")
            .extern_path(".google.protobuf.Duration", "crate::shim::Duration")
            .extern_path(".google.protobuf.Any", "crate::shim::Any");

        Self {
            project,
            tonic_build_config,
            root: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            out_dir,
            tmp_build_dir,
            deps,
        }
    }

    pub fn generate(&self) {
        self.prepare_dir();
        self.compile_proto();

        info!(
            "🧪 [{}] Embellishing modules to expose nice API for library user...",
            self.project.name
        );

        self.transform();
        self.generate_mod_file();
        self.fmt();

        info!(
            "✨  [{}] Library is successfully generated!",
            self.project.name
        );
    }

    fn prepare_dir(&self) {
        if self.tmp_build_dir.exists() {
            remove_dir_all(self.tmp_build_dir.clone()).unwrap();
        }
        create_dir_all(self.tmp_namespaced_dir()).unwrap();
        output_version_file(
            &self.project.name,
            &self.project.version,
            &self.tmp_namespaced_dir(),
        );
    }

    fn generate_mod_file(&self) {
        mod_gen::generate_mod_file(&self.absolute_out_dir());
    }

    fn transform(&self) {
        transform::copy_and_transform_all(
            &self.tmp_namespaced_dir(),
            &self.absolute_out_dir(),
            &self.file_descriptor_set(),
        );
    }

    fn absolute_out_dir(&self) -> PathBuf {
        self.root.join(&self.out_dir)
    }

    fn fmt(&self) {
        let manifest_path = find_cargo_toml(&self.absolute_out_dir());
        let exit_status = Command::new("cargo")
            .arg("fmt")
            .arg("--manifest-path")
            .arg(manifest_path.to_string_lossy().to_string())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        if !exit_status.success() {
            panic!(
                "unable to format with: cargo fmt --manifest-path {}",
                manifest_path.to_string_lossy()
            );
        }
    }

    fn compile_proto(&self) {
        let include_paths = ["proto", "third_party/proto"];

        let all_related_projects = vec![self.deps.clone(), vec![self.project.clone()]].concat();

        // construct absolute paths to be included from all related projects
        let proto_includes_paths: Vec<PathBuf> = all_related_projects
            .iter()
            .flat_map(|project| {
                {
                    include_paths
                        .iter()
                        .map(|path| self.root.join(&project.project_dir).join(path))
                }
            })
            .map(PathBuf::from)
            .collect();

        let proto_paths = all_related_projects
            .iter()
            .map(|p| {
                let paths = fs::read_dir(
                    self.root
                        .join(&p.project_dir)
                        .join(format!("proto/{}", p.name)),
                )
                .unwrap()
                .map(|d| d.unwrap().path().to_string_lossy().to_string())
                .collect::<Vec<String>>();

                // if include_mods is not empty, include only those, else include everything
                if !p.include_mods.is_empty() {
                    let include_mods = p.include_mods.to_vec();
                    paths
                        .into_iter()
                        .filter(|p| include_mods.iter().any(|m| p.contains(m)))
                        .collect()
                } else {
                    paths
                }
            })
            .collect::<Vec<Vec<String>>>()
            .concat();

        // List available proto files
        let mut protos: Vec<PathBuf> = vec![];
        collect_protos(&proto_paths, &mut protos);

        info!(
            "🧪 [{}] Compiling types from protobuf definitions...",
            self.project.name
        );

        let descriptor_file = self.tmp_namespaced_dir().join(DESCRIPTOR_FILE);
        self.tonic_build_config
            .clone()
            .out_dir(self.tmp_namespaced_dir())
            .file_descriptor_set_path(&descriptor_file)
            .compile(&protos, &proto_includes_paths)
            .unwrap();

        info!(
            "✨  [{}] Types from protobuf definitions is compiled successfully!",
            self.project.name
        );
    }

    pub fn file_descriptor_set(&self) -> FileDescriptorSet {
        let descriptor_file = self.tmp_namespaced_dir().join(DESCRIPTOR_FILE);
        let descriptor_bytes = &fs::read(descriptor_file).unwrap()[..];

        FileDescriptorSet::decode(descriptor_bytes).unwrap()
    }

    // TODO: create config tonic

    fn tmp_namespaced_dir(&self) -> PathBuf {
        self.tmp_build_dir.join(&self.project.name)
    }
}

fn output_version_file(project_name: &str, versions: &str, out_dir: &Path) {
    let path = out_dir.join(format!("{}_COMMIT", project_name.to_uppercase()));
    fs::write(path, versions).unwrap();
}

/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}

fn find_cargo_toml(path: &Path) -> PathBuf {
    if path.join("Cargo.toml").exists() {
        path.to_path_buf().join("Cargo.toml")
    } else {
        find_cargo_toml(path.parent().expect("Cargo.toml not found"))
    }
}
