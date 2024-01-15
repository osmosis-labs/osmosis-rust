//! Build Osmosis proto files. This build script clones the CosmosSDK and Osmosis version
//! specified in the COSMOS_SDK_REV and OSMOSIS_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{env, path::PathBuf};

use proto_build::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.47.5-osmo";

/// The osmosis commit or tag to be cloned and used to build the proto files
const OSMOSIS_REV: &str = "v21.0.0";

/// The wasmd commit or tag to be cloned and used to build the proto files
const WASMD_REV: &str = "v0.45.0-osmo";

/// The cometbft commit or tag to be cloned and used to build the proto files
const COMETBFT_REV: &str = "v0.37.2";

/// The ibc-go commit or tag to be cloned and used to build the proto files
const IBC_GO_REV: &str = "v7.3.1";

/// The ics23 commit or tag to be cloned and used to build the proto files
///
/// cosmos-sdk deps for `osmo/v0.47` is `v0.9.0` but that has no buf.yml,
/// so we are using this version instead which will work but
/// [prehash_key_before_comparison](https://github.com/cosmos/ics23/commit/cea74ba58ffbf87154701cd5959184acedf09cd6#diff-fe43695465b668ae6b79cc97ff2103fbb665f8440c42bc4f85a1942380a3fae4)
/// will be missing
const ICS23_REV: &str = "rust/v0.10.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../osmosis-std/src/types/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk/";
/// Directory where the osmosis submodule is located
const OSMOSIS_DIR: &str = "../../dependencies/osmosis/";
/// Directory where the wasmd submodule is located
const WASMD_DIR: &str = "../../dependencies/wasmd/";
/// Directory where the cometbft submodule is located
const COMETBFT_DIR: &str = "../../dependencies/cometbft/";
/// Directory where the ibc-go submodule is located
const IBC_GO_DIR: &str = "../../dependencies/ibc-go/";
/// Directory where the ics23 submodule is located
const ICS23_DIR: &str = "../../dependencies/ics23/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn generate() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodule(COSMOS_SDK_DIR, COSMOS_SDK_REV);
        git::update_submodule(OSMOSIS_DIR, OSMOSIS_REV);
        git::update_submodule(WASMD_DIR, WASMD_REV);
        git::update_submodule(COMETBFT_DIR, COMETBFT_REV);
        git::update_submodule(IBC_GO_DIR, IBC_GO_REV);
        git::update_submodule(ICS23_DIR, ICS23_REV);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let osmosis_project = CosmosProject {
        name: "osmosis".to_string(),
        version: OSMOSIS_REV.to_string(),
        project_dir: OSMOSIS_DIR.to_string(),
        exclude_mods: vec![],
    };
    let wasmd_project = CosmosProject {
        name: "wasmd".to_string(),
        version: WASMD_REV.to_string(),
        project_dir: WASMD_DIR.to_string(),
        exclude_mods: vec![],
    };
    let cometbft_project = CosmosProject {
        name: "tendermint".to_string(),
        version: COMETBFT_REV.to_string(),
        project_dir: COMETBFT_DIR.to_string(),
        exclude_mods: vec![],
    };

    let ibc_project = CosmosProject {
        name: "ibc".to_string(),
        version: IBC_GO_REV.to_string(),
        project_dir: IBC_GO_DIR.to_string(),
        exclude_mods: vec![],
    };

    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: COSMOS_SDK_REV.to_string(),
        project_dir: COSMOS_SDK_DIR.to_string(),
        exclude_mods: vec!["reflection".to_string(), "autocli".to_string()],
    };

    let ics23_project = CosmosProject {
        name: "ics23".to_string(),
        version: ICS23_REV.to_string(),
        project_dir: ICS23_DIR.to_string(),
        exclude_mods: vec![],
    };

    let osmosis_code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        osmosis_project,
        vec![
            ibc_project,
            cometbft_project,
            cosmos_project,
            wasmd_project,
            ics23_project,
        ],
    );

    osmosis_code_generator.generate();
}

fn main() {
    pretty_env_logger::init();
    generate();
}
