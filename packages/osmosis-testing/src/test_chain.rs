use crate::bindings::{CwGetCodeInfo, CwStoreCode, GetAllBalances, InitAccount, InitTestEnv};
use cosmos_sdk_proto::cosmwasm::wasm::v1::CodeInfo;
use cosmos_sdk_proto::prost::Message;
use cosmwasm_std::Coin;
use std::ffi::CString;

pub struct TestChain {
    id: u64,
}

impl TestChain {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: unsafe { InitTestEnv() },
        }
    }

    /// This function initialize account with initial balance of any coins.
    /// It returns bech32 address of the account initialized.
    pub fn init_account(&self, coins: &[Coin]) -> String {
        let coins_json = serde_json::to_string(&coins).unwrap();
        let coins_json_c = &CString::new(coins_json).unwrap();

        unsafe {
            let addr = InitAccount(self.id, coins_json_c.into());
            CString::from_raw(addr)
        }
        .to_str()
        .expect("invalid utf8")
        .to_string()
    }

    /// Get all balances of an account
    pub fn get_all_balances(&self, bech32_addr: &str) -> Vec<Coin> {
        let addr_c = &CString::new(bech32_addr).unwrap();
        let bal = unsafe {
            let bal = GetAllBalances(self.id, addr_c.into());
            CString::from_raw(bal)
        }
        .to_str()
        .expect("invalid utf8")
        .to_string();

        serde_json::from_str(&bal).expect("invalid Vec<Coin> json")
    }

    /// Store code to state machine, returns code id
    pub fn store_code(&self, bech32_addr: &str, wasm: &[u8]) -> u64 {
        let base64_wasm = base64::encode(wasm);
        let base64_wasm_c = &CString::new(base64_wasm).unwrap();
        let bech32_addr_c = &CString::new(bech32_addr).unwrap();

        unsafe { CwStoreCode(self.id, bech32_addr_c.into(), base64_wasm_c.into()) }
    }

    /// Get code_info by code_id
    pub fn get_code_info(&self, code_id: &u64) -> Option<CodeInfo> {
        unsafe {
            let code_info = CwGetCodeInfo(self.id, code_id.to_owned());

            if code_info.is_null() {
                None
            } else {
                let code_info_c = CString::from_raw(code_info);
                Some(CodeInfo::decode(code_info_c.as_bytes()).unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::test_chain::TestChain;
    use cosmwasm_std::{coins, Coin};

    #[test]
    fn test_init_account() {
        let chain = TestChain::new();

        // alice
        let alice_balance = coins(100_000_000_000, "uosmo");
        let alice = chain.init_account(&alice_balance);
        assert_eq!(chain.get_all_balances(&alice), alice_balance);

        // bob
        let bob_balance = [
            Coin::new(100_000_000_000, "uatom"),
            Coin::new(999_999_999_999, "uion"),
        ];
        let bob = chain.init_account(&bob_balance);
        assert_eq!(chain.get_all_balances(&bob), bob_balance);
    }

    #[test]
    fn test_store_and_init_simple_contract() {
        let env = TestChain::new();
        let contract_owner = env.init_account(&[]);

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // TODO: use cw-plus/swap_router as dev deps instead and build as wasm in build.rs to OUT_DIR
        let wasm_path = manifest_dir.join(
            "../../examples/cosmwasm/target/wasm32-unknown-unknown/release/osmosis_stargate.wasm",
        );

        // TODO: refactor this to `fn store_code_from_path`
        let wasm = std::fs::read(wasm_path).unwrap();
        let code_id = env.store_code(&contract_owner, &wasm);

        assert_eq!(code_id, 1);

        let code_info = env.get_code_info(&code_id);
        assert_eq!(code_info.unwrap().creator, contract_owner);

        let code_info = env.get_code_info(&999);
        assert_eq!(code_info, None);
    }
}
