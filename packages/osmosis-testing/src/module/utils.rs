use cosmrs::proto::{
    cosmos::bank::v1beta1::MsgSend,
    cosmwasm::wasm::v1::{
        MsgClearAdmin, MsgExecuteContract, MsgInstantiateContract, MsgMigrateContract,
        MsgUpdateAdmin,
    },
};
use cosmwasm_std::{BankMsg, Coin, StdResult, WasmMsg};
use prost::Message;

use crate::{Account, EncodeError, RunnerError, SigningAccount};

pub fn coins_to_proto(coins: &[Coin]) -> Vec<cosmrs::proto::cosmos::base::v1beta1::Coin> {
    coins
        .iter()
        .map(|c| cosmrs::proto::cosmos::base::v1beta1::Coin {
            denom: c.denom.parse().unwrap(),
            amount: format!("{}", c.amount.u128()),
        })
        .collect()
}

pub fn proto_coins_to_coins(coins: &[cosmrs::proto::cosmos::base::v1beta1::Coin]) -> Vec<Coin> {
    coins
        .iter()
        .map(|c| Coin {
            denom: c.denom.to_string(),
            amount: c.amount.parse().unwrap(),
        })
        .collect()
}

pub fn osmosis_coins_to_coins(
    coins: &[osmosis_std::types::cosmos::base::v1beta1::Coin],
) -> Vec<Coin> {
    coins
        .into_iter()
        .map(|c| c.clone().try_into())
        .collect::<StdResult<_>>()
        .unwrap()
}

pub fn msg_to_any<T: Message>(type_url: &str, msg: &T) -> Result<cosmrs::Any, RunnerError> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)
        .map_err(EncodeError::ProtoEncodeError)?;

    Ok(cosmrs::Any {
        type_url: type_url.to_owned(),
        value: buf,
    })
}

pub fn bank_msg_to_any(msg: &BankMsg, signer: &SigningAccount) -> Result<cosmrs::Any, RunnerError> {
    match msg {
        BankMsg::Send { to_address, amount } => {
            let type_url = "/cosmos.bank.v1beta1.MsgSend";
            let msg = MsgSend {
                from_address: signer.address().to_string(),
                to_address: to_address.to_string(),
                amount: coins_to_proto(&amount),
            };
            msg_to_any(type_url, &msg)
        }
        _ => {
            todo!() // TODO: Can't find BurnMsg...?
        }
    }
}

pub fn wasm_msg_to_any(msg: &WasmMsg, signer: &SigningAccount) -> Result<cosmrs::Any, RunnerError> {
    match msg {
        WasmMsg::Execute {
            contract_addr,
            msg,
            funds,
        } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgExecuteContract",
            &MsgExecuteContract {
                contract: contract_addr.clone(),
                funds: coins_to_proto(&funds),
                sender: signer.address(),
                msg: msg.to_vec(),
            },
        ),
        WasmMsg::Instantiate {
            admin,
            code_id,
            msg,
            funds,
            label,
        } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgInstantiateContract",
            &MsgInstantiateContract {
                sender: signer.address(),
                admin: admin.clone().unwrap_or_default(),
                code_id: *code_id,
                label: label.clone(),
                msg: msg.to_vec(),
                funds: coins_to_proto(&funds),
            },
        ),
        WasmMsg::Migrate {
            contract_addr,
            new_code_id,
            msg,
        } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgMigrateContract",
            &MsgMigrateContract {
                contract: contract_addr.clone(),
                sender: signer.address(),
                code_id: *new_code_id,
                msg: msg.to_vec(),
            },
        ),
        WasmMsg::UpdateAdmin {
            contract_addr,
            admin,
        } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgUpdateAdmin",
            &MsgUpdateAdmin {
                contract: contract_addr.clone(),
                sender: signer.address(),
                new_admin: admin.clone(),
            },
        ),
        WasmMsg::ClearAdmin { contract_addr } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgClearAdmin",
            &MsgClearAdmin {
                contract: contract_addr.clone(),
                sender: signer.address(),
            },
        ),
        _ => Err(RunnerError::ExecuteError {
            msg: "Unsupported WasmMsg".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use cosmrs::proto::{
        cosmos::bank::v1beta1::{MsgSendResponse, QueryBalanceRequest},
        cosmwasm::wasm::v1::{MsgExecuteContractResponse, MsgInstantiateContractResponse},
    };
    use cosmwasm_std::{to_binary, BankMsg, Coin, CosmosMsg, Empty, Event, WasmMsg};
    use cw1_whitelist::msg::{ExecuteMsg, InstantiateMsg};
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgCreateDenomResponse,
    };

    use crate::{Account, Bank, Module, OsmosisTestApp, Runner, Wasm};

    #[test]
    fn test_cosmos_msg() {
        let app = OsmosisTestApp::new();
        let signer = app
            .init_account(&[Coin::new(10000000000, "uosmo")])
            .unwrap();

        let bank = Bank::new(&app);

        // BankMsg::Send
        let to = app.init_account(&[]).unwrap();
        let coin = Coin::new(100, "uosmo");
        let send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: to.address(),
            amount: vec![coin],
        });
        app.execute_cosmos_msgs::<MsgSendResponse>(&[send_msg], &signer)
            .unwrap();
        let balance = bank
            .query_balance(&QueryBalanceRequest {
                address: to.address().to_string(),
                denom: "uosmo".to_string(),
            })
            .unwrap()
            .balance;
        assert_eq!(balance.clone().unwrap().amount, "100".to_string());
        assert_eq!(balance.unwrap().denom, "uosmo".to_string());

        // WasmMsg, first upload a contract
        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, &signer)
            .unwrap()
            .data
            .code_id;
        assert_eq!(code_id, 1);

        // Wasm::Instantiate
        let instantiate_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id,
            msg: to_binary(&InstantiateMsg {
                admins: vec![signer.address()],
                mutable: true,
            })
            .unwrap(),
            funds: vec![],
            label: "test".to_string(),
            admin: None,
        });
        let init_res = app
            .execute_cosmos_msgs::<MsgInstantiateContractResponse>(&[instantiate_msg], &signer)
            .unwrap();
        let contract_address = init_res.data.address;
        assert_ne!(contract_address, "".to_string());

        // Wasm::Execute
        let execute_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_address.clone(),
            msg: to_binary(&ExecuteMsg::<Empty>::Freeze {}).unwrap(),
            funds: vec![],
        });
        let execute_res = app
            .execute_cosmos_msgs::<MsgExecuteContractResponse>(&[execute_msg], &signer)
            .unwrap();
        let events = execute_res.events;

        let wasm_events: Vec<Event> = events.into_iter().filter(|x| x.ty == "wasm").collect();
        for event in wasm_events.iter() {
            assert_eq!(event.attributes[0].key, "_contract_address");
            assert_eq!(event.attributes[0].value, contract_address);
            assert_eq!(event.attributes[1].key, "action");
            assert_eq!(event.attributes[1].value, "freeze");
        }

        // Stargate
        let denom = "test".to_string();
        let create_denom_msg: CosmosMsg = MsgCreateDenom {
            sender: signer.address(),
            subdenom: denom.clone(),
        }
        .into();
        let create_denom_res = app
            .execute_cosmos_msgs::<MsgCreateDenomResponse>(&[create_denom_msg], &signer)
            .unwrap();
        assert_eq!(
            create_denom_res.data.new_token_denom,
            format!("factory/{}/{}", signer.address(), denom.clone())
        );
    }
}
