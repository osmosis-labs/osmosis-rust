use cosmrs::proto::{
    cosmwasm::wasm::v1::{AccessConfig, MsgStoreCode},
    tendermint::abci::ResponseDeliverTx,
};

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
