#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gauge {
    /// unique ID of a Gauge
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// flag to show if it's perpetual or multi-epoch
    /// distribution incentives by third party
    #[prost(bool, tag = "2")]
    pub is_perpetual: bool,
    /// Rewards are distributed to lockups that are are returned by at least one of
    /// these queries
    #[prost(message, optional, tag = "3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// total amount of Coins that has been in the gauge.
    /// can distribute multiple coins
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// number of epochs distribution will be done
    #[prost(uint64, tag = "6")]
    pub num_epochs_paid_over: u64,
    /// number of epochs distributed already
    #[prost(uint64, tag = "7")]
    pub filled_epochs: u64,
    /// already distributed coins
    #[prost(message, repeated, tag = "8")]
    pub distributed_coins: ::prost::alloc::vec::Vec<
        cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for Gauge {
    const TYPE_URL: &'static str = "/osmosis.incentives.Gauge";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockableDurationsInfo {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg for LockableDurationsInfo {
    const TYPE_URL: &'static str = "/osmosis.incentives.LockableDurationsInfo";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGauge {
    /// flag to show if it's perpetual or multi-epoch
    /// distribution incentives by third party
    #[prost(bool, tag = "1")]
    pub is_perpetual: bool,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// distribute condition of a lock which meet one of these conditions
    #[prost(message, optional, tag = "3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// can distribute multiple coins
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// number of epochs distribution will be done
    #[prost(uint64, tag = "6")]
    pub num_epochs_paid_over: u64,
}
impl crate::cosmwasm::ToCosmosMsg for MsgCreateGauge {
    const TYPE_URL: &'static str = "/osmosis.incentives.MsgCreateGauge";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGaugeResponse {}
impl crate::cosmwasm::ToCosmosMsg for MsgCreateGaugeResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.MsgCreateGaugeResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGauge {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub gauge_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub rewards: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for MsgAddToGauge {
    const TYPE_URL: &'static str = "/osmosis.incentives.MsgAddToGauge";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGaugeResponse {}
impl crate::cosmwasm::ToCosmosMsg for MsgAddToGaugeResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.MsgAddToGaugeResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsRequest {}
impl crate::cosmwasm::ToCosmosMsg for ModuleToDistributeCoinsRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.ModuleToDistributeCoinsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for ModuleToDistributeCoinsResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.ModuleToDistributeCoinsResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDistributedCoinsRequest {}
impl crate::cosmwasm::ToCosmosMsg for ModuleDistributedCoinsRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.ModuleDistributedCoinsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDistributedCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for ModuleDistributedCoinsResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.ModuleDistributedCoinsResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl crate::cosmwasm::ToCosmosMsg for GaugeByIdRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.GaugeByIdRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub gauge: ::core::option::Option<Gauge>,
}
impl crate::cosmwasm::ToCosmosMsg for GaugeByIdResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.GaugeByIdResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for GaugesRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.GaugesRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for GaugesResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.GaugesResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for ActiveGaugesRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.ActiveGaugesRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for ActiveGaugesResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.ActiveGaugesResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for ActiveGaugesPerDenomRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.ActiveGaugesPerDenomRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for ActiveGaugesPerDenomResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.ActiveGaugesPerDenomResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for UpcomingGaugesRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.UpcomingGaugesRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for UpcomingGaugesResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.UpcomingGaugesResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for UpcomingGaugesPerDenomRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.UpcomingGaugesPerDenomRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub upcoming_gauges: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse,
    >,
}
impl crate::cosmwasm::ToCosmosMsg for UpcomingGaugesPerDenomResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.UpcomingGaugesPerDenomResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag = "2")]
    pub lock_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(int64, tag = "3")]
    pub end_epoch: i64,
}
impl crate::cosmwasm::ToCosmosMsg for RewardsEstRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.RewardsEstRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for RewardsEstResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.RewardsEstResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsRequest {}
impl crate::cosmwasm::ToCosmosMsg for QueryLockableDurationsRequest {
    const TYPE_URL: &'static str = "/osmosis.incentives.QueryLockableDurationsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg for QueryLockableDurationsResponse {
    const TYPE_URL: &'static str = "/osmosis.incentives.QueryLockableDurationsResponse";
}
/// Params holds parameters for the incentives module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// distribution epoch identifier
    #[prost(string, tag = "1")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for Params {
    const TYPE_URL: &'static str = "/osmosis.incentives.Params";
}
/// GenesisState defines the incentives module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, repeated, tag = "3")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_types::Duration>,
    #[prost(uint64, tag = "4")]
    pub last_gauge_id: u64,
}
impl crate::cosmwasm::ToCosmosMsg for GenesisState {
    const TYPE_URL: &'static str = "/osmosis.incentives.GenesisState";
}
