//! Build Osmosis proto files. This build script clones the CosmosSDK and Osmosis version
//! specified in the COSMOS_SDK_REV and OSMOSIS_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process::Command,
};

use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use log::{debug, info};
use prost_types::ServiceDescriptorProto;
use regex::Regex;
use syn::{
    __private::quote::{format_ident, quote}, Fields, File, Ident, Item, ItemMod, parse_quote, Type,
};
use syn::__private::TokenStream2;
use walkdir::WalkDir;

use crate::code_generator::{CodeGenerator, CosmosProject, query_services};

mod code_generator;
mod git;
mod transform;
mod mod_gen;

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.45.4";

/// The osmosis commit or tag to be cloned and used to build the proto files
const OSMOSIS_REV: &str = "v11.0.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const PROTO_DIR: &str = "../osmosis-std/src/types/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk";
/// Directory where the osmosis submodule is located
const OSMOSIS_DIR: &str = "../../dependencies/osmosis";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodules();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = PROTO_DIR.parse().unwrap();

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
        proto_dir.clone(),
        tmp_build_dir.clone(),
        osmosis_project,
        vec![cosmos_project],
    );

    osmosis_code_generator.generate();

    let temp_osmosis_dir = tmp_build_dir.join("osmosis");

    info!("🧪  Embellishing modules to expose nice API for library user...");

    transform::copy_and_transform_generated_files(
        &temp_osmosis_dir,
        &proto_dir,
        query_services(osmosis_code_generator.file_descriptor_set()),
    );
    mod_gen::generate_mod_file(&proto_dir);

    // format osmosis std
    env::set_current_dir(Path::new("../osmosis-std")).unwrap();
    Command::new("cargo")
        .arg("fmt")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    info!("✨  `osmosis-std` is successfully generated!");
}

// pass file descriptor in and have fn to get query service from it
// with file descriptor we can have real detail about type paths
// now, type that contains `ID` will not be serialized properly
