#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochInfo {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(int64, tag = "4")]
    pub current_epoch: i64,
    #[prost(message, optional, tag = "5")]
    pub current_epoch_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "6")]
    pub epoch_counting_started: bool,
    #[prost(int64, tag = "8")]
    pub current_epoch_start_height: i64,
}
impl crate::cosmwasm::ToCosmosMsg for EpochInfo {
    const TYPE_URL: &'static str = "/osmosis.epochs.v1beta1.EpochInfo";
}
/// GenesisState defines the epochs module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
impl crate::cosmwasm::ToCosmosMsg for GenesisState {
    const TYPE_URL: &'static str = "/osmosis.epochs.v1beta1.GenesisState";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochsInfoRequest {}
impl crate::cosmwasm::ToCosmosMsg for QueryEpochsInfoRequest {
    const TYPE_URL: &'static str = "/osmosis.epochs.v1beta1.QueryEpochsInfoRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochsInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
impl crate::cosmwasm::ToCosmosMsg for QueryEpochsInfoResponse {
    const TYPE_URL: &'static str = "/osmosis.epochs.v1beta1.QueryEpochsInfoResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochRequest {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for QueryCurrentEpochRequest {
    const TYPE_URL: &'static str = "/osmosis.epochs.v1beta1.QueryCurrentEpochRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochResponse {
    #[prost(int64, tag = "1")]
    pub current_epoch: i64,
}
impl crate::cosmwasm::ToCosmosMsg for QueryCurrentEpochResponse {
    const TYPE_URL: &'static str = "/osmosis.epochs.v1beta1.QueryCurrentEpochResponse";
}
