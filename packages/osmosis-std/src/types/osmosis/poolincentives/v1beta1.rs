use osmosis_std_derive::CosmwasmExt;
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.Params")]
pub struct Params {
    /// minted_denom is the denomination of the coin expected to be minted by the
    /// minting module. Pool-incentives module doesn’t actually mint the coin
    /// itself, but rather manages the distribution of coins that matches the
    /// defined minted_denom.
    #[prost(string, tag = "1")]
    pub minted_denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.LockableDurationsInfo")]
pub struct LockableDurationsInfo {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.DistrInfo")]
pub struct DistrInfo {
    #[prost(string, tag = "1")]
    pub total_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.DistrRecord")]
pub struct DistrRecord {
    #[prost(uint64, tag = "1")]
    pub gauge_id: u64,
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
/// ReplacePoolIncentivesProposal is a gov Content type for updating the pool
/// incentives. If a ReplacePoolIncentivesProposal passes, the proposal’s records
/// override the existing DistrRecords set in the module. Each record has a
/// specified gauge id and weight, and the incentives are distributed to each
/// gauge according to weight/total_weight. The incentives are put in the fee
/// pool and it is allocated to gauges and community pool by the DistrRecords
/// configuration. Note that gaugeId=0 represents the community pool.
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.ReplacePoolIncentivesProposal")]
pub struct ReplacePoolIncentivesProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
/// For example: if the existing DistrRecords were:
/// [(Gauge 0, 5), (Gauge 1, 6), (Gauge 2, 6)]
/// An UpdatePoolIncentivesProposal includes
/// [(Gauge 1, 0), (Gauge 2, 4), (Gauge 3, 10)]
/// This would delete Gauge 1, Edit Gauge 2, and Add Gauge 3
/// The result DistrRecords in state would be:
/// [(Gauge 0, 5), (Gauge 2, 4), (Gauge 3, 10)]
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.UpdatePoolIncentivesProposal")]
pub struct UpdatePoolIncentivesProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryGaugeIdsRequest")]
pub struct QueryGaugeIdsRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryGaugeIdsResponse")]
pub struct QueryGaugeIdsResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauge_ids_with_duration:
        ::prost::alloc::vec::Vec<query_gauge_ids_response::GaugeIdWithDuration>,
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
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryDistrInfoRequest")]
pub struct QueryDistrInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryDistrInfoResponse")]
pub struct QueryDistrInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub distr_info: ::core::option::Option<DistrInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryParamsRequest")]
pub struct QueryParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryLockableDurationsRequest")]
pub struct QueryLockableDurationsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryLockableDurationsResponse")]
pub struct QueryLockableDurationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryIncentivizedPoolsRequest")]
pub struct QueryIncentivizedPoolsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.IncentivizedPool")]
pub struct IncentivizedPool {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "2")]
    pub lockable_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint64, tag = "3")]
    pub gauge_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryIncentivizedPoolsResponse")]
pub struct QueryIncentivizedPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub incentivized_pools: ::prost::alloc::vec::Vec<IncentivizedPool>,
}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryExternalIncentiveGaugesRequest")]
pub struct QueryExternalIncentiveGaugesRequest {}
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.QueryExternalIncentiveGaugesResponse")]
pub struct QueryExternalIncentiveGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<super::super::incentives::Gauge>,
}
/// GenesisState defines the pool incentives module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message, CosmwasmExt)]
#[proto(type_url = "/osmosis.poolincentives.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
    #[prost(message, optional, tag = "3")]
    pub distr_info: ::core::option::Option<DistrInfo>,
}
