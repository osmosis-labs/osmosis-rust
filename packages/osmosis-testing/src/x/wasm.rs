use cosmrs::proto::{
    cosmwasm::wasm::v1::{
        AccessConfig, MsgExecuteContract, MsgInstantiateContract, MsgStoreCode,
        QuerySmartContractStateRequest, QuerySmartContractStateResponse,
    },
    tendermint::abci::ResponseDeliverTx,
};
use cosmwasm_std::Coin;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    account::{Account, SigningAccount},
    runner::Runner,
};

pub struct Wasm<'a, R: Runner> {
    runner: &'a R,
}

impl<'a, R: Runner> super::Module<'a, R> for Wasm<'a, R> {
    fn new(runner: &'a R) -> Self {
        Wasm { runner }
    }
}

impl<'a, R> super::AsModule<'a, Wasm<'a, R>, R> for R
where
    R: Runner,
{
    fn as_module(&'a self) -> Wasm<'a, R> {
        Wasm { runner: self }
    }
}

impl<'a, R> Wasm<'a, R>
where
    R: Runner,
{
    pub fn store_code(
        &self,
        wasm_byte_code: &[u8],
        instantiate_permission: Option<AccessConfig>,
        signer: &SigningAccount,
    ) -> u64 {
        let res = self.runner.execute(
            MsgStoreCode {
                sender: signer.address(),
                wasm_byte_code: wasm_byte_code.to_vec(),
                instantiate_permission,
            },
            "/cosmwasm.wasm.v1.MsgStoreCode",
            signer,
        );

        // TODO: create more robust mech to get the response
        find_attr(res, "store_code", "code_id")
            .unwrap()
            .parse()
            .unwrap()
    }

    pub fn instantiate<M>(
        &self,
        code_id: u64,
        msg: &M,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> String
    where
        M: ?Sized + Serialize,
    {
        let res = self.runner.execute(
            MsgInstantiateContract {
                sender: signer.address(),
                admin: "".to_string(),
                code_id,
                label: "default".to_string(),
                msg: serde_json::to_vec(msg).expect("json serialization failed"),
                funds: funds
                    .iter()
                    .map(|c| cosmrs::proto::cosmos::base::v1beta1::Coin {
                        denom: c.denom.parse().unwrap(),
                        amount: format!("{}", c.amount.u128()),
                    })
                    .collect(),
            },
            "/cosmwasm.wasm.v1.MsgInstantiateContract",
            signer,
        );

        // TODO: create more robust mech to get the response
        find_attr(res, "instantiate", "_contract_address")
            .unwrap()
            .parse()
            .unwrap()
    }

    pub fn execute<M>(&self, contract: &str, msg: &M, funds: &[Coin], signer: &SigningAccount)
    where
        M: ?Sized + Serialize,
    {
        self.runner.execute(
            MsgExecuteContract {
                sender: signer.address(),
                msg: serde_json::to_vec(msg).expect("json serialization failed"),
                funds: funds
                    .iter()
                    .map(|c| cosmrs::proto::cosmos::base::v1beta1::Coin {
                        denom: c.denom.parse().unwrap(),
                        amount: format!("{}", c.amount.u128()),
                    })
                    .collect(),
                contract: contract.to_owned(),
            },
            "/cosmwasm.wasm.v1.MsgExecuteContract",
            signer,
        );
    }

    pub fn query<M, Res>(&self, contract: &str, msg: &M) -> Res
    where
        M: ?Sized + Serialize,
        Res: ?Sized + DeserializeOwned,
    {
        let res = self
            .runner
            .query::<QuerySmartContractStateRequest, QuerySmartContractStateResponse>(
                "/cosmwasm.wasm.v1.Query/SmartContractState",
                &QuerySmartContractStateRequest {
                    address: contract.to_owned(),
                    query_data: serde_json::to_vec(msg).expect("json serialization failed"),
                },
            );

        serde_json::from_slice(&res.data).unwrap()
    }
}

fn find_attr(res: ResponseDeliverTx, event: &str, attr: &str) -> Option<String> {
    let e = res.events.into_iter().find(|e| e.r#type == event);

    let attr_bytes = e?
        .attributes
        .into_iter()
        .find(|a| a.key == attr.as_bytes())?
        .value;

    Some(std::str::from_utf8(&attr_bytes).unwrap().to_owned())
}
