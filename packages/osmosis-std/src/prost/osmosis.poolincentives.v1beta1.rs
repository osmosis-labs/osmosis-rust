#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// minted_denom is the denomination of the coin expected to be minted by the
    /// minting module. Pool-incentives module doesn’t actually mint the coin
    /// itself, but rather manages the distribution of coins that matches the
    /// defined minted_denom.
    #[prost(string, tag = "1")]
    pub minted_denom: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for Params {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.Params";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockableDurationsInfo {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg for LockableDurationsInfo {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.LockableDurationsInfo";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistrInfo {
    #[prost(string, tag = "1")]
    pub total_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
impl crate::cosmwasm::ToCosmosMsg for DistrInfo {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.DistrInfo";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistrRecord {
    #[prost(uint64, tag = "1")]
    pub gauge_id: u64,
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for DistrRecord {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.DistrRecord";
}
/// ReplacePoolIncentivesProposal is a gov Content type for updating the pool
/// incentives. If a ReplacePoolIncentivesProposal passes, the proposal’s records
/// override the existing DistrRecords set in the module. Each record has a
/// specified gauge id and weight, and the incentives are distributed to each
/// gauge according to weight/total_weight. The incentives are put in the fee
/// pool and it is allocated to gauges and community pool by the DistrRecords
/// configuration. Note that gaugeId=0 represents the community pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplacePoolIncentivesProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
impl crate::cosmwasm::ToCosmosMsg for ReplacePoolIncentivesProposal {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.ReplacePoolIncentivesProposal";
}
/// For example: if the existing DistrRecords were:
/// [(Gauge 0, 5), (Gauge 1, 6), (Gauge 2, 6)]
/// An UpdatePoolIncentivesProposal includes
/// [(Gauge 1, 0), (Gauge 2, 4), (Gauge 3, 10)]
/// This would delete Gauge 1, Edit Gauge 2, and Add Gauge 3
/// The result DistrRecords in state would be:
/// [(Gauge 0, 5), (Gauge 2, 4), (Gauge 3, 10)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePoolIncentivesProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
impl crate::cosmwasm::ToCosmosMsg for UpdatePoolIncentivesProposal {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.UpdatePoolIncentivesProposal";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugeIdsRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
impl crate::cosmwasm::ToCosmosMsg for QueryGaugeIdsRequest {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryGaugeIdsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGaugeIdsResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauge_ids_with_duration: ::prost::alloc::vec::Vec<
        query_gauge_ids_response::GaugeIdWithDuration,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for QueryGaugeIdsResponse {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryGaugeIdsResponse";
}
/// Nested message and enum types in `QueryGaugeIdsResponse`.
pub mod query_gauge_ids_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GaugeIdWithDuration {
        #[prost(uint64, tag = "1")]
        pub gauge_id: u64,
        #[prost(message, optional, tag = "2")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDistrInfoRequest {}
impl crate::cosmwasm::ToCosmosMsg for QueryDistrInfoRequest {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryDistrInfoRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDistrInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub distr_info: ::core::option::Option<DistrInfo>,
}
impl crate::cosmwasm::ToCosmosMsg for QueryDistrInfoResponse {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryDistrInfoResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl crate::cosmwasm::ToCosmosMsg for QueryParamsRequest {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryParamsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl crate::cosmwasm::ToCosmosMsg for QueryParamsResponse {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryParamsResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsRequest {}
impl crate::cosmwasm::ToCosmosMsg for QueryLockableDurationsRequest {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryLockableDurationsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg for QueryLockableDurationsResponse {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryLockableDurationsResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPoolsRequest {}
impl crate::cosmwasm::ToCosmosMsg for QueryIncentivizedPoolsRequest {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryIncentivizedPoolsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncentivizedPool {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "2")]
    pub lockable_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint64, tag = "3")]
    pub gauge_id: u64,
}
impl crate::cosmwasm::ToCosmosMsg for IncentivizedPool {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.IncentivizedPool";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub incentivized_pools: ::prost::alloc::vec::Vec<IncentivizedPool>,
}
impl crate::cosmwasm::ToCosmosMsg for QueryIncentivizedPoolsResponse {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryIncentivizedPoolsResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalIncentiveGaugesRequest {}
impl crate::cosmwasm::ToCosmosMsg for QueryExternalIncentiveGaugesRequest {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryExternalIncentiveGaugesRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExternalIncentiveGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<super::super::incentives::Gauge>,
}
impl crate::cosmwasm::ToCosmosMsg for QueryExternalIncentiveGaugesResponse {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.QueryExternalIncentiveGaugesResponse";
}
/// GenesisState defines the pool incentives module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
    #[prost(message, optional, tag = "3")]
    pub distr_info: ::core::option::Option<DistrInfo>,
}
impl crate::cosmwasm::ToCosmosMsg for GenesisState {
    const TYPE_URL: &'static str = "/osmosis.poolincentives.v1beta1.GenesisState";
}
