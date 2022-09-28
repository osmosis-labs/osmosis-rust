use std::fmt::Debug;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, to_vec, Binary, ContractResult, Deps, DepsMut, Empty, Env, MessageInfo, Reply,
    Response, StdError, StdResult, SystemResult,
};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::epochs::v1beta1::{
    QueryEpochsInfoRequest, QueryEpochsInfoResponse,
};
use osmosis_std::types::osmosis::gamm::v1beta1::{QueryNumPoolsRequest, QueryNumPoolsResponse};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:osmosis-std-cosmwasm-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryNumPools {} => {
            query_and_debug::<QueryNumPoolsResponse>(&deps, QueryNumPoolsRequest {})
        }
        QueryMsg::QueryEpochsInfo {} => {
            query_and_debug::<QueryEpochsInfoResponse>(&deps, QueryEpochsInfoRequest {})
        } // Find matched incoming message variant and query them your custom logic
          // and then construct your query response with the type usually defined
          // `msg.rs` alongside with the query message itself.
          //
          // use `cosmwasm_std::to_binary` to serialize query response to json binary.
    }
}

fn query_and_debug<T>(
    deps: &Deps,
    q: impl Into<cosmwasm_std::QueryRequest<Empty>>,
) -> StdResult<Binary>
where
    T: Serialize + DeserializeOwned + Debug,
{
    to_binary(&{
        let request: cosmwasm_std::QueryRequest<Empty> = q.into();
        let raw = to_vec(&request).map_err(|serialize_err| {
            StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
        })?;
        let res: T = match deps.querier.raw_query(&raw) {
            SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
                "Querier system error: {}",
                system_err
            ))),
            SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(
                format!("Querier contract error: {}", contract_err),
            )),
            SystemResult::Ok(ContractResult::Ok(value)) => {
                // deps.api
                //     .debug(std::str::from_utf8(value.as_slice()).unwrap());
                cosmwasm_std::from_binary(&value)
            }
        }?;
        res
    })
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
