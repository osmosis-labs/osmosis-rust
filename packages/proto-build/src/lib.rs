//! Build Osmosis proto files. This build script clones the CosmosSDK and Osmosis version
//! specified in the COSMOS_SDK_REV and OSMOSIS_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{env, path::PathBuf};

use crate::code_generator::{CodeGenerator, CosmosProject};
use syn::__private::quote::{format_ident, quote};

mod code_generator;
mod git;
mod mod_gen;
mod transform;
mod transformers;

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "sdk-v13.0.0-rc1";

/// The osmosis commit or tag to be cloned and used to build the proto files
const OSMOSIS_REV: &str = "v13.0.0-rc2";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../osmosis-std/src/types/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk/";
/// Directory where the osmosis submodule is located
const OSMOSIS_DIR: &str = "../../dependencies/osmosis/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodules();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let osmosis_project = CosmosProject {
        name: "osmosis".to_string(),
        version: OSMOSIS_REV.to_string(),
        project_dir: OSMOSIS_DIR.to_string(),
    };
    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: COSMOS_SDK_REV.to_string(),
        project_dir: COSMOS_SDK_DIR.to_string(),
    };

    let osmosis_code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        osmosis_project,
        vec![cosmos_project],
    );

    osmosis_code_generator.generate();
}
