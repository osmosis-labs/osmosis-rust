use log::info;
use prost::Message;
use prost_types::{FileDescriptorSet, ServiceDescriptorProto};
use std::collections::HashMap;
use std::fs;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use walkdir::WalkDir;

const DESCRIPTOR_FILE: &'static str = "descriptor.bin";

pub struct CosmosProject {
    pub name: String,
    pub version: String,
    pub project_dir: String,
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
        if self.tmp_build_dir.exists() {
            remove_dir_all(self.tmp_build_dir.clone()).unwrap();
        }
        create_dir_all(self.tmp_namespaced_dir()).unwrap();
        output_version_file(
            &self.project.name,
            &self.project.version,
            &self.tmp_namespaced_dir(),
        );
        self.compile_proto();
    }

    fn compile_proto(&self) {
        let include_paths = ["proto", "third_party/proto"];

        let deps_dirs = self
            .deps
            .iter()
            .map(|dep| self.root.join(&dep.project_dir))
            .collect();
        let project_dir = self.root.join(&self.project.project_dir);

        let proto_includes_path = vec![deps_dirs, vec![project_dir.clone()]].concat();
        let proto_includes_paths = proto_includes_path
            .iter()
            .flat_map(|dir| include_paths.iter().map(|path| dir.join(path)));

        // List available paths for dependencies
        let includes: Vec<PathBuf> = proto_includes_paths.map(PathBuf::from).collect();

        let proto_paths = fs::read_dir(project_dir.join(format!("proto/{}", self.project.name)))
            .unwrap()
            .map(|d| d.unwrap().path().to_string_lossy().to_string())
            .collect::<Vec<String>>();

        // List available proto files
        let mut protos: Vec<PathBuf> = vec![];
        collect_protos(&proto_paths, &mut protos);

        info!("🧪 Compiling Osmosis' types from protobuf definitions...");

        let descriptor_file = self.tmp_namespaced_dir().join(DESCRIPTOR_FILE);
        self.tonic_build_config
            .clone()
            .out_dir(self.tmp_namespaced_dir())
            .file_descriptor_set_path(&descriptor_file)
            .compile(&protos, &includes)
            .unwrap();

        info!("✨  Osmosis' types from protobuf definitions is compiled successfully!");
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

fn output_version_file(project_name: &str, versions: &str, out_dir: &PathBuf) {
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

pub fn query_services(descriptor: FileDescriptorSet) -> HashMap<String, ServiceDescriptorProto> {
    descriptor
        .file
        .into_iter()
        .filter_map(|f| {
            let service = f
                .service
                .into_iter()
                .find(|s| s.name == Some("Query".to_string()));

            if let Some(service) = service {
                Some((
                    f.package.expect("Missing package name in file descriptor"),
                    service,
                ))
            } else {
                None
            }
        })
        .collect()
}
