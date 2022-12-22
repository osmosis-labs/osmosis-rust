use osmosis_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/osmosis.epochs.v1beta1.EpochInfo")]
pub struct EpochInfo {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub current_epoch: i64,
    #[prost(message, optional, tag = "5")]
    pub current_epoch_start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bool, tag = "6")]
    pub epoch_counting_started: bool,
    #[prost(int64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub current_epoch_start_height: i64,
}
/// GenesisState defines the epochs module's genesis state.
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
#[proto_message(type_url = "/osmosis.epochs.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
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
#[proto_message(type_url = "/osmosis.epochs.v1beta1.QueryEpochsInfoRequest")]
#[proto_query(
    path = "/osmosis.epochs.v1beta1.Query/EpochInfos",
    response_type = QueryEpochsInfoResponse
)]
pub struct QueryEpochsInfoRequest {}
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
#[proto_message(type_url = "/osmosis.epochs.v1beta1.QueryEpochsInfoResponse")]
pub struct QueryEpochsInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
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
#[proto_message(type_url = "/osmosis.epochs.v1beta1.QueryCurrentEpochRequest")]
#[proto_query(
    path = "/osmosis.epochs.v1beta1.Query/CurrentEpoch",
    response_type = QueryCurrentEpochResponse
)]
pub struct QueryCurrentEpochRequest {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.epochs.v1beta1.QueryCurrentEpochResponse")]
pub struct QueryCurrentEpochResponse {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub current_epoch: i64,
}
pub struct EpochsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> EpochsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn epoch_infos(&self) -> Result<QueryEpochsInfoResponse, cosmwasm_std::StdError> {
        QueryEpochsInfoRequest {}.query(self.querier)
    }
    pub fn current_epoch(
        &self,
        identifier: ::prost::alloc::string::String,
    ) -> Result<QueryCurrentEpochResponse, cosmwasm_std::StdError> {
        QueryCurrentEpochRequest { identifier }.query(self.querier)
    }
}
