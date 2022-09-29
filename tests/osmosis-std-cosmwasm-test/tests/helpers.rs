use cosmwasm_std::Coin;
use osmosis_std_cosmwasm_test::msg::InstantiateMsg;
use osmosis_testing::{Gamm, Module, OsmosisTestApp, SigningAccount, Wasm};
use std::path::PathBuf;

pub fn with_env_setup(
    run: impl Fn(&OsmosisTestApp, Wasm<OsmosisTestApp>, SigningAccount, u64, String),
) {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);
    let signer = app
        .init_account(&[
            Coin::new(100_000_000_000, "uosmo"),
            Coin::new(100_000_000_000, "uion"),
        ])
        .unwrap();

    let code_id = wasm
        .store_code(&get_wasm_byte_code(), None, &signer)
        .unwrap()
        .data
        .code_id;
    let contract_addr = wasm
        .instantiate(code_id, &InstantiateMsg {}, None, None, &[], &signer)
        .unwrap()
        .data
        .address;
    run(&app, wasm, signer, code_id, contract_addr)
}

pub fn setup_pools(app: &OsmosisTestApp, signer: &SigningAccount) -> Vec<u64> {
    let gamm = Gamm::new(app);
    let pool_id = gamm
        .create_basic_pool(
            &[Coin::new(1_000, "uosmo"), Coin::new(1_000, "uion")],
            signer,
        )
        .unwrap()
        .data
        .pool_id;

    vec![pool_id]
}
pub fn get_wasm_byte_code() -> Vec<u8> {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    std::fs::read(
        manifest_path
            .join("..")
            .join("..")
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("release")
            .join("osmosis_std_cosmwasm_test.wasm"),
    )
    .unwrap()
}
