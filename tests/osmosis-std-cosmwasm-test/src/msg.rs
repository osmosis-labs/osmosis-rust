use cosmwasm_schema::{cw_serde, QueryResponses};
pub use osmosis_std::types::osmosis::epochs::v1beta1::QueryEpochsInfoResponse;
pub use osmosis_std::types::osmosis::gamm::v1beta1::QueryNumPoolsResponse;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryNumPoolsResponse)]
    QueryNumPools {},

    #[returns(QueryEpochsInfoResponse)]
    QueryEpochsInfo {},
}
