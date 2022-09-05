use std::ffi::CString;

use cosmrs::{
    crypto::secp256k1::SigningKey,
    proto::tendermint::abci::{RequestDeliverTx, RequestQuery, ResponseDeliverTx, ResponseQuery},
    tx::{self, Fee, SignerInfo},
};

use cosmwasm_std::{Coin, Uint128};
use prost::Message;

use crate::{
    account::{Account, SigningAccount},
    bindings::{
        AccountNumber, AccountSequence, BeginBlock, EndBlock, Execute, InitAccount, InitTestEnv,
        Simulate,
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
    fn query(&self, req: RequestQuery) -> ResponseQuery;
}

#[derive(Debug, Clone, PartialEq)]
pub struct App {
    id: u64,
    gas_price: Coin,
    gas_adjustment: f64,
}

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
    pub fn set_gas_price(mut self, gas_price: Coin) -> Self {
        self.gas_price = gas_price;
        self
    }

    pub fn set_gas_adjustment(mut self, gas_adjustment: f64) -> Self {
        self.gas_adjustment = gas_adjustment;
        self
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

    fn query(&self, req: RequestQuery) -> ResponseQuery {
        todo!()
    }
}

mod tests {
    use super::*;
    use cosmrs::proto::tendermint::abci::EventAttribute;
    use cosmwasm_std::coins;
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

    use crate::account::Account;

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
    fn test_execute_1() {
        test_execute()
    }

    #[test]
    fn test_execute_2() {
        test_execute()
    }
}
