use cosmrs::proto::{
    cosmos::bank::v1beta1::MsgSend,
    cosmwasm::wasm::v1::{
        MsgClearAdmin, MsgExecuteContract, MsgInstantiateContract, MsgMigrateContract,
        MsgUpdateAdmin,
    },
};
use cosmwasm_std::{BankMsg, Coin, CosmosMsg, WasmMsg};
use prost::Message;

use crate::{runner::Runner, Account, RunnerError, RunnerExecuteResult, SigningAccount};

fn coins_to_proto(coins: &[Coin]) -> Vec<cosmrs::proto::cosmos::base::v1beta1::Coin> {
    coins
        .iter()
        .map(|c| cosmrs::proto::cosmos::base::v1beta1::Coin {
            denom: c.denom.parse().unwrap(),
            amount: format!("{}", c.amount.u128()),
        })
        .collect()
}

pub fn execute_cosmos_msg<R, S>(
    runner: R,
    msg: CosmosMsg,
    signer: SigningAccount,
) -> RunnerExecuteResult<S>
where
    R: for<'a> Runner<'a>,
    S: Message + Default,
{
    match msg {
        CosmosMsg::Bank(msg) => match msg {
            BankMsg::Send { to_address, amount } => runner.execute(
                MsgSend {
                    amount: coins_to_proto(&amount),
                    from_address: signer.address(),
                    to_address,
                },
                "/cosmos.bank.v1beta1.MsgSend",
                &signer,
            ),
            _ => {
                todo!() // TODO: Can't find BurnMsg...?
            }
        },
        CosmosMsg::Custom(_) => todo!(),
        CosmosMsg::Staking(_) => todo!(),
        CosmosMsg::Distribution(_) => todo!(),
        CosmosMsg::Stargate { type_url, value } => runner.execute_multiple_raw(
            vec![cosmrs::Any {
                type_url,
                value: value.0,
            }],
            &signer,
        ),
        // runner.execute(value, type_url.as_str(), &signer),
        CosmosMsg::Ibc(_) => todo!(),
        CosmosMsg::Wasm(msg) => execute_wasm_msg::<R, S>(runner, msg, signer),
        CosmosMsg::Gov(_) => todo!(),
        _ => todo!(),
    }
}

pub fn execute_wasm_msg<R, S>(
    runner: R,
    msg: WasmMsg,
    signer: SigningAccount,
) -> RunnerExecuteResult<S>
where
    R: for<'a> Runner<'a>,
    S: Message + Default,
{
    match msg {
        WasmMsg::Execute {
            contract_addr,
            msg,
            funds,
        } => {
            let msg = MsgExecuteContract {
                contract: contract_addr,
                funds: coins_to_proto(&funds),
                sender: signer.address(),
                msg: msg.to_vec(),
            };
            let type_url = "/cosmwasm.wasm.v1.MsgExecuteContract";
            runner.execute(msg, type_url, &signer)
        }
        WasmMsg::Instantiate {
            admin,
            code_id,
            msg,
            funds,
            label,
        } => runner.execute(
            MsgInstantiateContract {
                sender: signer.address(),
                admin: admin.unwrap_or_default(),
                code_id,
                label,
                msg: msg.to_vec(),
                funds: coins_to_proto(&funds),
            },
            "/cosmwasm.wasm.v1.MsgInstantiateContract",
            &signer,
        ),
        WasmMsg::Migrate {
            contract_addr,
            new_code_id,
            msg,
        } => runner.execute(
            MsgMigrateContract {
                contract: contract_addr,
                sender: signer.address(),
                code_id: new_code_id,
                msg: msg.to_vec(),
            },
            "/cosmwasm.wasm.v1.MsgMigrateContract",
            &signer,
        ),
        WasmMsg::UpdateAdmin {
            contract_addr,
            admin,
        } => runner.execute(
            MsgUpdateAdmin {
                contract: contract_addr,
                sender: signer.address(),
                new_admin: admin,
            },
            "/cosmwasm.wasm.v1.MsgUpdateAdmin",
            &signer,
        ),
        WasmMsg::ClearAdmin { contract_addr } => runner.execute(
            MsgClearAdmin {
                contract: contract_addr,
                sender: signer.address(),
            },
            "/cosmwasm.wasm.v1.MsgClearAdmin",
            &signer,
        ),
        _ => Err(RunnerError::ExecuteError {
            msg: "Unsupported WasmMsg".to_string(),
        })?,
    }
}
