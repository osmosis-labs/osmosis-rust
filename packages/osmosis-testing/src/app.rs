use crate::{
    account::{Account, SigningAccount},
    bindings::{
        CommitTx, CwExecute, CwGetCodeInfo, CwGetContractInfo, CwInstantiate, CwQuery, CwStoreCode,
        GAMMCreateBalancerPool, GAMMGetTotalPoolLiquidity, GetAllBalances, InitAccount,
        InitTestEnv,
    },
    redefine_as_go_string,
};

use cosmrs::{
    cosmwasm::MsgInstantiateContract,
    crypto::secp256k1::SigningKey,
    proto::{
        cosmwasm::wasm::{
            self,
            v1::{CodeInfo, ContractInfo, MsgExecuteContract},
        },
        tendermint::abci::{RequestDeliverTx, ResponseDeliverTx},
    },
};
use cosmwasm_std::Coin;
use osmosis_std::types::osmosis::gamm::{
    poolmodels::balancer::v1beta1::MsgCreateBalancerPool,
    v1beta1::{PoolAsset, PoolParams},
};
use prost::Message;
use serde::{de::DeserializeOwned, Serialize};
use std::{ffi::CString, io, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    id: u64,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: unsafe { InitTestEnv() },
        }
    }

    /// This function initialize account with initial balance of any coins.
    pub fn init_account(&self, coins: &[Coin]) -> SigningAccount {
        let mut coins = coins.to_vec();

        // invalid coins if denom are unsorted
        coins.sort_by(|a, b| a.denom.cmp(&b.denom));

        let coins_json = serde_json::to_string(&coins).unwrap();
        redefine_as_go_string!(coins_json);

        let base64_priv = unsafe {
            let addr = InitAccount(self.id, coins_json);
            CString::from_raw(addr)
        }
        .to_str()
        .expect("invalid utf8")
        .to_string();

        let secp256k1_priv = base64::decode(base64_priv).expect("base64 decode failed");
        SigningKey::from_bytes(&secp256k1_priv)
            .expect("invalid signing key")
            .into()
    }

    /// Get all balances of an account
    pub fn get_all_balances(&self, bech32_addr: &str) -> Vec<Coin> {
        redefine_as_go_string!(bech32_addr);

        let bal = unsafe {
            let bal = GetAllBalances(self.id, bech32_addr);
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
        redefine_as_go_string!(base64_wasm, bech32_addr);

        unsafe { CwStoreCode(self.id, bech32_addr, base64_wasm) }
    }

    pub fn store_code_from_path(&self, bech32_addr: &str, wasm_path: PathBuf) -> io::Result<u64> {
        let wasm = std::fs::read(wasm_path)?;
        Ok(self.store_code(bech32_addr, &wasm))
    }

    /// Instantiate contract
    pub fn instantiate_contract<A1, A2, M>(
        &self,
        sender: &A1,
        code_id: u64,
        msg: &M,
        funds: &[Coin],
        admin: Option<&A2>,
        label: Option<String>,
    ) -> String
    where
        A1: Account,
        A2: Account,
        M: ?Sized + Serialize,
    {
        let msg = MsgInstantiateContract {
            sender: sender.account_id(),
            admin: admin.map(|a| a.account_id()),
            code_id,
            label,
            msg: serde_json::to_vec(msg).expect("json serialization failed"),
            funds: funds
                .iter()
                .map(|c| -> cosmrs::Coin {
                    cosmrs::proto::cosmos::base::v1beta1::Coin {
                        denom: c.denom.clone(),
                        amount: c.amount.into(),
                    }
                    .try_into()
                    .unwrap()
                })
                .collect(),
        };

        let msg: wasm::v1::MsgInstantiateContract = msg.into();

        let mut buf = Vec::new();
        wasm::v1::MsgInstantiateContract::encode(&msg, &mut buf)
            .expect("Message encoding must be infallible");

        let base64_msg = base64::encode(buf);
        redefine_as_go_string!(base64_msg);

        unsafe {
            let addr = CwInstantiate(self.id, base64_msg);
            CString::from_raw(addr)
                .to_str()
                .expect("bech32 address must be valid UTF-8")
                .to_string()
        }
    }

    /// Execute contract
    pub fn execute_contract<M>(
        &self,
        sender: &str,
        contract: &str,
        msg: &M,
        funds: &[Coin],
    ) -> String
    where
        M: ?Sized + Serialize,
    {
        let msg = MsgExecuteContract {
            sender: sender.into(),
            contract: contract.into(),
            msg: serde_json::to_vec(msg).expect("json serialization failed"),
            funds: funds
                .iter()
                .map(|c| cosmrs::proto::cosmos::base::v1beta1::Coin {
                    denom: c.denom.clone(),
                    amount: c.amount.into(),
                })
                .collect(),
        };

        let msg: wasm::v1::MsgExecuteContract = msg;

        let mut buf = Vec::new();
        wasm::v1::MsgExecuteContract::encode(&msg, &mut buf)
            .expect("Message encoding must be infallible");

        let base64_msg = base64::encode(buf);
        redefine_as_go_string!(base64_msg);

        unsafe {
            let res = CwExecute(self.id, base64_msg);
            CString::from_raw(res)
                .to_str()
                .expect("bech32 address must be valid UTF-8")
                .to_string()
        }
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

    /// Get contract_info by contract address
    pub fn get_contract_info(&self, contract_address: &str) -> Option<ContractInfo> {
        redefine_as_go_string!(contract_address);
        unsafe {
            let contract_info = CwGetContractInfo(self.id, contract_address);

            if contract_info.is_null() {
                None
            } else {
                let contract_info_c = CString::from_raw(contract_info);
                Some(ContractInfo::decode(contract_info_c.as_bytes()).unwrap())
            }
        }
    }

    /// Query contract
    pub fn query_contract<Q, R>(&self, contract_address: &str, query_msg: &Q) -> R
    where
        Q: ?Sized + Serialize,
        R: ?Sized + DeserializeOwned,
    {
        let query_msg = serde_json::to_string(query_msg).unwrap();
        redefine_as_go_string!(contract_address, query_msg);
        unsafe {
            let query_response = CwQuery(self.id, contract_address, query_msg);

            let query_response = CString::from_raw(query_response);
            let query_response = query_response.to_str().unwrap_unchecked();

            serde_json::from_str(query_response).unwrap()
        }
    }

    /// Create basic pool
    pub fn create_basic_pool(&self, sender: &str, initial_liquidity: &[Coin]) -> u64 {
        let msg = MsgCreateBalancerPool {
            sender: sender.to_string(),
            pool_params: Some(PoolParams {
                swap_fee: "10000000000000000".to_string(),
                exit_fee: "10000000000000000".to_string(),
                smooth_weight_change_params: None,
            }),
            pool_assets: initial_liquidity
                .iter()
                .map(|c| PoolAsset {
                    token: Some(c.clone().into()),
                    weight: "1000000".to_string(),
                })
                .collect(),
            future_pool_governor: "".to_string(),
        };

        let mut buf = Vec::new();
        MsgCreateBalancerPool::encode(&msg, &mut buf).expect("Message encoding must be infallible");

        let base64_msg = base64::encode(buf);
        redefine_as_go_string!(base64_msg);

        unsafe { GAMMCreateBalancerPool(self.id, base64_msg) }
    }

    pub fn get_total_pool_liquidity(&self, pool_id: u64) -> Vec<Coin> {
        unsafe {
            let liq = GAMMGetTotalPoolLiquidity(self.id, pool_id);
            let liq = CString::from_raw(liq);

            serde_json::from_slice(liq.as_bytes()).unwrap()
        }
    }

    // (WIP)
    pub fn commit_tx(&self, req: RequestDeliverTx) -> ResponseDeliverTx {
        let mut buf = Vec::new();
        RequestDeliverTx::encode(&req, &mut buf).expect("Message encoding must be infallible");

        let base64_req = base64::encode(buf);
        redefine_as_go_string!(base64_req);
        unsafe {
            let res = CommitTx(self.id, base64_req);
            let res_c = CString::from_raw(res);
            ResponseDeliverTx::decode(res_c.as_bytes()).unwrap()
        }
    }

    // fn deliver_and_commit(msgs, signing_account)
    // https://github.com/osmosis-labs/beaker/blob/ce0ecd8a0d3eb10acd5a048f52da5d68653d1754/packages/cli/src/support/cosmos.rs#L224-L241
}

#[cfg(test)]
mod tests {
    use crate::{account::Account, app::App};
    use cosmrs::{
        proto::tendermint::abci::RequestDeliverTx,
        tx::{self, Fee, SignerInfo},
    };
    use cosmwasm_std::{coins, Coin};
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
    use prost::Message;
    use rayon::prelude::*;
    use std::path::PathBuf;

    fn cw1_whitelist_wasm_path() -> PathBuf {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir
            .join("test_artifacts")
            .join("cw1_whitelist.wasm")
    }

    #[test]
    fn test_parrallel_env_access_should_not_cause_concurrent_map_write_issue() {
        (1..25).into_par_iter().for_each(|_| {
            let app = App::new();
            app.init_account(&[]);
        });
    }

    #[test]
    fn test_init_account() {
        let app = App::new();

        // alice
        let alice_balance = coins(100_000_000_000, "uosmo");
        let alice = app.init_account(&alice_balance);
        assert_eq!(app.get_all_balances(&alice.address()), alice_balance);

        // bob
        let bob_balance = [
            Coin::new(100_000_000_000, "uatom"),
            Coin::new(999_999_999_999, "uion"),
        ];
        let bob = app.init_account(&bob_balance);
        assert_eq!(app.get_all_balances(&bob.address()), bob_balance);
    }

    #[test]
    fn test_store_contract() {
        let app = App::new();
        let contract_owner = app.init_account(&[]);

        let code_id = app
            .store_code_from_path(&contract_owner.address(), cw1_whitelist_wasm_path())
            .unwrap();

        assert_eq!(code_id, 1);

        let code_info = app.get_code_info(&code_id);
        assert_eq!(code_info.unwrap().creator, contract_owner.address());

        let code_info = app.get_code_info(&999);
        assert_eq!(code_info, None);
    }
    #[test]
    fn test_store_and_init_contract() {
        let app = App::new();
        let contract_owner = app.init_account(&[]);

        let code_id = app
            .store_code_from_path(&contract_owner.address(), cw1_whitelist_wasm_path())
            .unwrap();

        let contract_addr = app.instantiate_contract(
            &contract_owner,
            code_id,
            &cw1_whitelist::msg::InstantiateMsg {
                admins: vec![],
                mutable: true,
            },
            &[],
            Some(&contract_owner),
            None,
        );

        let contract_info = app.get_contract_info(&contract_addr).unwrap();
        assert_eq!(contract_info.code_id, code_id);
        assert_eq!(contract_info.creator, contract_owner.address());
    }

    #[test]
    fn test_execute_contract_and_query_contract() {
        let app = App::new();
        let contract_owner = app.init_account(&[]);
        let alice = app.init_account(&[Coin::new(100_000_000_000, "uosmo")]);

        let code_id = app
            .store_code_from_path(&contract_owner.address(), cw1_whitelist_wasm_path())
            .unwrap();

        let contract_addr = app.instantiate_contract(
            &contract_owner,
            code_id,
            &cw1_whitelist::msg::InstantiateMsg {
                admins: vec![contract_owner.address()],
                mutable: true,
            },
            &[],
            Some(&contract_owner),
            None,
        );

        let admins = vec![alice.address()];
        app.execute_contract(
            &contract_owner.address(),
            &contract_addr,
            &cw1_whitelist::msg::ExecuteMsg::<cosmwasm_std::Empty>::UpdateAdmins {
                admins: admins.clone(),
            },
            &[],
        );

        let res: cw1_whitelist::msg::AdminListResponse = app.query_contract(
            &contract_addr,
            &cw1_whitelist::msg::QueryMsg::<cosmwasm_std::Empty>::AdminList {},
        );

        assert_eq!(
            res,
            cw1_whitelist::msg::AdminListResponse {
                admins,
                mutable: true
            }
        );
    }

    #[test]
    fn test_create_basic_pool() {
        let app = App::new();
        let alice = app.init_account(&[
            Coin::new(100_000_000_000, "uion"),
            Coin::new(100_000_000_000, "uatom"),
            Coin::new(100_000_000_000, "uosmo"),
        ]);

        let liq = [Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
        let pool_id = app.create_basic_pool(&alice.address(), &liq);

        assert_eq!(app.get_total_pool_liquidity(pool_id), liq.to_vec());
    }

    #[test]
    // (WIP) deliverState's context is not initialized properly
    fn test_commit_tx() {
        let app = App::new();
        let acc = app.init_account(&[Coin::new(1000000000000000, "stake")]);

        dbg!(acc.address());

        let mut buf = Vec::new();
        let msg = MsgCreateDenom {
            sender: acc.address(),
            subdenom: "newdenom".to_string(),
        };

        MsgCreateDenom::encode(&msg, &mut buf).unwrap();

        let msg = cosmrs::Any {
            type_url: MsgCreateDenom::TYPE_URL.to_string(),
            value: buf,
        };

        // - init tx body
        let tx_body = tx::Body::new([msg], "", 0u32);

        // - [test] set big fee
        let fee = Fee::from_amount_and_gas(
            cosmrs::Coin {
                amount: "1000000000".parse().unwrap(),
                denom: "stake".parse().unwrap(), // NOTE: using "uosmo" causes invalid fee token
            },
            10000000,
        );

        // - set auth_info
        let auth_info = SignerInfo::single_direct(Some(acc.public_key()), 0).auth_info(fee);

        // - sign
        let sign_doc =
            tx::SignDoc::new(&tx_body, &auth_info, &("osmosis-1".parse().unwrap()), 9).unwrap();
        let tx_raw = sign_doc.sign(&acc.signing_key()).unwrap();

        let tx = tx_raw.to_bytes().unwrap();

        let res = app.commit_tx(RequestDeliverTx { tx });

        dbg!(res);
        // NOTE: this resulted in:
        // res = ResponseDeliverTx {
        //     code: 4,
        //     data: [],
        //     log: "signature verification failed; please verify account number (8) and chain-id (): unauthorized",
        //     info: "",
        //     gas_wanted: 10000000,
        //     gas_used: 58703,
        //     events: [],
        //     codespace: "sdk",
        // }
    }
}
