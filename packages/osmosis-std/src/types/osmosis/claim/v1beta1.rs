use osmosis_std_derive::CosmwasmExt;
/// Params defines the claim module's parameters.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.Params")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub airdrop_start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub duration_until_decay: ::core::option::Option<crate::shim::Duration>,
    #[prost(message, optional, tag = "3")]
    pub duration_of_decay: ::core::option::Option<crate::shim::Duration>,
    /// denom of claimable asset
    #[prost(string, tag = "4")]
    pub claim_denom: ::prost::alloc::string::String,
}
/// A Claim Records is the metadata of claim data per address
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.ClaimRecord")]
pub struct ClaimRecord {
    /// address of claim user
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// total initial claimable amount for the user
    #[prost(message, repeated, tag = "2")]
    pub initial_claimable_amount:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// true if action is completed
    /// index of bool in array refers to action enum #
    #[prost(bool, repeated, packed = "false", tag = "3")]
    pub action_completed: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    AddLiquidity = 0,
    Swap = 1,
    Vote = 2,
    DelegateStake = 3,
}
/// GenesisState defines the claim module's genesis state.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.GenesisState")]
pub struct GenesisState {
    /// balance of the claim module's account
    #[prost(message, optional, tag = "1")]
    pub module_account_balance:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
    /// list of claim records, one for every airdrop recipient
    #[prost(message, repeated, tag = "3")]
    pub claim_records: ::prost::alloc::vec::Vec<ClaimRecord>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryModuleAccountBalanceRequest")]
#[proto_query(
    path = "/osmosis.claim.v1beta1.Query/ModuleAccountBalance",
    response_type = QueryModuleAccountBalanceResponse
)]
pub struct QueryModuleAccountBalanceRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryModuleAccountBalanceResponse")]
pub struct QueryModuleAccountBalanceResponse {
    /// params defines the parameters of the module.
    #[prost(message, repeated, tag = "1")]
    pub module_account_balance:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/osmosis.claim.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryClaimRecordRequest")]
#[proto_query(
    path = "/osmosis.claim.v1beta1.Query/ClaimRecord",
    response_type = QueryClaimRecordResponse
)]
pub struct QueryClaimRecordRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryClaimRecordResponse")]
pub struct QueryClaimRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub claim_record: ::core::option::Option<ClaimRecord>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryClaimableForActionRequest")]
#[proto_query(
    path = "/osmosis.claim.v1beta1.Query/ClaimableForAction",
    response_type = QueryClaimableForActionResponse
)]
pub struct QueryClaimableForActionRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "Action", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub action: i32,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryClaimableForActionResponse")]
pub struct QueryClaimableForActionResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryTotalClaimableRequest")]
#[proto_query(
    path = "/osmosis.claim.v1beta1.Query/TotalClaimable",
    response_type = QueryTotalClaimableResponse
)]
pub struct QueryTotalClaimableRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.claim.v1beta1.QueryTotalClaimableResponse")]
pub struct QueryTotalClaimableResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
pub struct ClaimQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ClaimQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn module_account_balance(
        &self,
    ) -> Result<QueryModuleAccountBalanceResponse, cosmwasm_std::StdError> {
        QueryModuleAccountBalanceRequest {}.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn claim_record(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryClaimRecordResponse, cosmwasm_std::StdError> {
        QueryClaimRecordRequest { address }.query(self.querier)
    }
    pub fn claimable_for_action(
        &self,
        address: ::prost::alloc::string::String,
        action: i32,
    ) -> Result<QueryClaimableForActionResponse, cosmwasm_std::StdError> {
        QueryClaimableForActionRequest { address, action }.query(self.querier)
    }
    pub fn total_claimable(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryTotalClaimableResponse, cosmwasm_std::StdError> {
        QueryTotalClaimableRequest { address }.query(self.querier)
    }
}
