use std::ffi::CString;

use cosmrs::{
    crypto::secp256k1::SigningKey,
    proto::tendermint::abci::{RequestDeliverTx, ResponseDeliverTx},
    tx::{self, Fee, SignerInfo},
};

use cosmwasm_std::{Coin, Uint128};
use prost::Message;

use crate::x::AsModule;
use crate::{
    account::{Account, SigningAccount},
    bindings::{
        AccountNumber, AccountSequence, BeginBlock, EndBlock, Execute, InitAccount, InitTestEnv,
        Query, Simulate,
    },
    redefine_as_go_string,
};

const FEE_DENOM: &str = "uosmo";
const CHAIN_ID: &str = "osmosis-1";

pub trait Runner {
    // TODO: use wraped response instead
    fn execute<M>(&self, msg: M, type_url: &str, signer: &SigningAccount) -> ResponseDeliverTx
    where
        M: ::prost::Message;
    fn query<Q, R>(&self, path: &str, query: &Q) -> R
    where
        Q: ::prost::Message,
        R: ::prost::Message + Default;
}

#[derive(Debug, Clone, PartialEq)]
pub struct App {
    id: u64,
    gas_price: Coin,
    gas_adjustment: f64,
}

impl<'a> AsModule<'a> for App {}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: unsafe { InitTestEnv() },
            gas_price: Coin {
                denom: FEE_DENOM.to_string(),
                amount: Uint128::new(0),
            },
            gas_adjustment: 1.2,
        }
    }
    pub fn gas_price(mut self, gas_price: Coin) -> Self {
        self.gas_price = gas_price;
        self
    }

    pub fn gas_adjustment(mut self, gas_adjustment: f64) -> Self {
        self.gas_adjustment = gas_adjustment;
        self
    }

    /// Initialize account with initial balance of any coins.
    /// This function mints new coins and send to newly created account
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
    /// Convinience function to create multiple accounts with the same
    /// Initial coins balance
    pub fn init_accounts(&self, coins: &[Coin], count: u64) -> Vec<SigningAccount> {
        (0..count)
            .into_iter()
            .map(|_| self.init_account(coins))
            .collect()
    }

    fn create_signed_tx<I>(&self, msgs: I, signer: &SigningAccount, fee: Fee) -> Vec<u8>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        let tx_body = tx::Body::new(msgs, "", 0u32);
        let addr = signer.address();
        redefine_as_go_string!(addr);

        let seq = unsafe { AccountSequence(self.id, addr) };

        let account_number = unsafe { AccountNumber(self.id, addr) };
        let signer_info = SignerInfo::single_direct(Some(signer.public_key()), seq);
        let auth_info = signer_info.auth_info(fee);
        let sign_doc = tx::SignDoc::new(
            &tx_body,
            &auth_info,
            &(CHAIN_ID.parse().unwrap()),
            account_number,
        )
        .unwrap();
        let tx_raw = sign_doc.sign(signer.signing_key()).unwrap();

        tx_raw.to_bytes().unwrap()
    }

    fn simulate_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
    ) -> cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        let zero_fee = Fee::from_amount_and_gas(
            cosmrs::Coin {
                denom: FEE_DENOM.parse().unwrap(),
                amount: 0u8.into(),
            },
            0u64,
        );

        let tx = self.create_signed_tx(msgs, signer, zero_fee);
        let base64_tx_bytes = base64::encode(&tx);
        redefine_as_go_string!(base64_tx_bytes);

        unsafe {
            let res = Simulate(self.id, base64_tx_bytes);

            cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo::decode(
                CString::from_raw(res).as_bytes(),
            )
            .unwrap()
        }
    }
    fn estimate_fee<I>(&self, msgs: I, signer: &SigningAccount) -> Fee
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        let gas_info = self.simulate_tx(msgs, signer);
        let gas_limit = ((gas_info.gas_used as f64) * (self.gas_adjustment)).ceil() as u64;

        let amount = cosmrs::Coin {
            denom: FEE_DENOM.parse().unwrap(),
            amount: (((gas_limit as f64) * (self.gas_price.amount.u128() as f64)).ceil() as u64)
                .into(),
        };

        Fee::from_amount_and_gas(amount, gas_limit)
    }
}

impl Runner for App {
    fn execute<M>(&self, msg: M, type_url: &str, signer: &SigningAccount) -> ResponseDeliverTx
    where
        M: ::prost::Message,
    {
        unsafe { BeginBlock(self.id) };

        let mut buf = Vec::new();
        M::encode(&msg, &mut buf)
            .expect("Using vec as buffer has theoretically unlimited capacity");

        let msg = cosmrs::Any {
            type_url: type_url.to_string(),
            value: buf,
        };

        let fee = self.estimate_fee([msg.clone()], signer);
        let tx = self.create_signed_tx([msg], signer, fee);

        let mut buf = Vec::new();
        RequestDeliverTx::encode(&RequestDeliverTx { tx }, &mut buf)
            .expect("Message encoding must be infallible");

        let base64_req = base64::encode(buf);
        redefine_as_go_string!(base64_req);
        let res = unsafe {
            let res = Execute(self.id, base64_req);
            let res_c = CString::from_raw(res);
            ResponseDeliverTx::decode(res_c.as_bytes()).unwrap()
        };

        unsafe { EndBlock(self.id) };

        res
    }

    fn query<Q, R>(&self, path: &str, q: &Q) -> R
    where
        Q: ::prost::Message,
        R: ::prost::Message + Default,
    {
        let mut buf = Vec::new();
        Q::encode(q, &mut buf).expect("Using vec as buffer has theoretically unlimited capacity");

        let base64_query_msg_bytes = base64::encode(buf);
        redefine_as_go_string!(path);
        redefine_as_go_string!(base64_query_msg_bytes);

        unsafe {
            let res = Query(self.id, path, base64_query_msg_bytes);
            let v = CString::from_raw(res);
            R::decode(v.as_bytes()).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmrs::proto::tendermint::abci::EventAttribute;
    use cosmwasm_std::coins;
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, QueryParamsRequest, QueryParamsResponse,
    };

    use crate::account::Account;
    use crate::x::gamm::Gamm;
    use crate::x::wasm::Wasm;

    #[test]
    fn test_init_accounts() {
        let app = App::new();
        let accounts = app.init_accounts(&coins(100_000_000_000, "uosmo"), 3);

        assert!(accounts.get(0).is_some());
        assert!(accounts.get(1).is_some());
        assert!(accounts.get(2).is_some());
        assert!(accounts.get(3).is_none());
    }

    #[test]
    fn test_execute() {
        let app = App::new();

        let acc = app.init_account(&coins(100_000_000_000, "uosmo"));
        let addr = acc.address();

        let msg = MsgCreateDenom {
            sender: acc.address(),
            subdenom: "newdenom".to_string(),
        };

        let res = app.execute(msg, MsgCreateDenom::TYPE_URL, &acc);

        let create_denom_attrs = &res
            .events
            .iter()
            .find(|e| e.r#type == "create_denom")
            .unwrap()
            .attributes;

        // TODO: make assertion based on string representation
        assert_eq!(
            create_denom_attrs,
            &vec![
                EventAttribute {
                    key: "creator".into(),
                    value: addr.clone().into(),
                    index: true
                },
                EventAttribute {
                    key: "new_token_denom".into(),
                    value: format!("factory/{}/{}", addr, "newdenom").into(),
                    index: true
                },
            ]
        );

        // execute on more time to excercise account sequence
        let msg = MsgCreateDenom {
            sender: acc.address(),
            subdenom: "newerdenom".to_string(),
        };

        let res = app.execute(msg, MsgCreateDenom::TYPE_URL, &acc);

        let create_denom_attrs = &res
            .events
            .iter()
            .find(|e| e.r#type == "create_denom")
            .unwrap()
            .attributes;

        // TODO: make assertion based on string representation
        assert_eq!(
            create_denom_attrs,
            &vec![
                EventAttribute {
                    key: "creator".into(),
                    value: addr.clone().into(),
                    index: true
                },
                EventAttribute {
                    key: "new_token_denom".into(),
                    value: format!("factory/{}/{}", addr, "newerdenom").into(),
                    index: true
                },
            ]
        );
    }

    #[test]
    fn test_query() {
        let app = App::new();

        let denom_creation_fee = app
            .query::<QueryParamsRequest, QueryParamsResponse>(
                "/osmosis.tokenfactory.v1beta1.Query/Params",
                &QueryParamsRequest {},
            )
            .params
            .unwrap()
            .denom_creation_fee;

        assert_eq!(denom_creation_fee, [Coin::new(10000000, "uosmo").into()])
    }

    #[test]
    fn test_multiple_as_module() {
        let app = App::new();
        let alice = app.init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ]);

        let gamm = app.as_module::<Gamm<_>>();

        let pool_liquidity = vec![Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
        gamm.create_basic_pool(&pool_liquidity, &alice);

        let pool = gamm.query_pool(1);

        assert_eq!(
            pool_liquidity
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<osmosis_std::types::cosmos::base::v1beta1::Coin>>(),
            pool.pool_assets
                .into_iter()
                .map(|a| a.token.unwrap())
                .collect::<Vec<osmosis_std::types::cosmos::base::v1beta1::Coin>>(),
        );

        let wasm = app.as_module::<Wasm<_>>();
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm.store_code(&wasm_byte_code, None, &alice);

        assert_eq!(code_id, 1);
    }

    #[test]
    fn test_wasm_execute_and_query() {
        use cw1_whitelist::msg::*;

        let app = App::new();
        let accs = app.init_accounts(
            &[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ],
            2,
        );
        let admin = &accs[0];
        let new_admin = &accs[1];

        let wasm: Wasm<_> = app.as_module();
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm.store_code(&wasm_byte_code, None, &admin);
        assert_eq!(code_id, 1);

        // initialize admins and check if the state is correct
        let init_admins = vec![admin.address()];
        let contract_addr = wasm.instantiate(
            code_id,
            &InstantiateMsg {
                admins: init_admins.clone(),
                mutable: true,
            },
            &[],
            admin,
        );
        let admin_list =
            wasm.query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {});
        assert_eq!(admin_list.admins, init_admins);
        assert!(admin_list.mutable);

        // update admin and check again
        let new_admins = vec![new_admin.address()];
        wasm.execute::<ExecuteMsg>(
            &contract_addr,
            &ExecuteMsg::UpdateAdmins {
                admins: new_admins.clone(),
            },
            &[],
            admin,
        );
        let admin_list =
            wasm.query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {});

        assert_eq!(admin_list.admins, new_admins);
        assert!(admin_list.mutable);
    }
}
