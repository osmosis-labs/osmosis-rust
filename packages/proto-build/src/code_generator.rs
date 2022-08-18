use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;

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
    deps: Vec<CosmosProject>
}

impl CodeGenerator {
    pub fn new(
        out_dir: PathBuf,
        // TODO: remove tmp_build_dir from constructor in favor of generated tmp dir
        tmp_build_dir: PathBuf,
        project: CosmosProject,
        deps: Vec<CosmosProject>
    ) -> Self {
        let descriptor_file = tmp_build_dir.join("descriptors.bin");

        let tonic_build_config = tonic_build::configure()
            .build_client(false)
            .build_server(false)
            .out_dir(tmp_build_dir.clone())
            .extern_path(".google.protobuf.Timestamp", "crate::shim::Timestamp")
            .extern_path(".google.protobuf.Duration", "crate::shim::Duration")
            .extern_path(".google.protobuf.Any", "crate::shim::Any")
            .file_descriptor_set_path(&descriptor_file);

        Self {
            project,
            tonic_build_config,
            root: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            out_dir,
            tmp_build_dir,
            deps
        }
    }

    pub fn generate(&self) {
        if self.tmp_build_dir.exists() {
            remove_dir_all(self.tmp_build_dir.clone()).unwrap();
        }
        create_dir_all(self.tmp_namespaced_dir()).unwrap();
        output_version_file(&self.project.name, &self.project.version, &self.tmp_namespaced_dir());
    }

    // TODO: create config tonic

    fn tmp_namespaced_dir(&self) -> PathBuf {
        self.tmp_build_dir.join(&self.project.name)
    }
}

fn output_version_file(namespace: &str, versions: &str, out_dir: &PathBuf) {
    let path = out_dir.join(format!("{}_COMMIT", namespace));
    std::fs::write(path, versions).unwrap();
}
