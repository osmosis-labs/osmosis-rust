use osmosis_std_derive::CosmwasmExt;
/// Gauge is an object that stores and distributes yields to recipients who
/// satisfy certain conditions. Currently gauges support conditions around the
/// duration for which a given denom is locked.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.Gauge")]
pub struct Gauge {
    /// id is the unique ID of a Gauge
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// is_perpetual is a flag to show if it's a perpetual or non-perpetual gauge
    /// Non-perpetual gauges distribute their tokens equally per epoch while the
    /// gauge is in the active period. Perpetual gauges distribute all their tokens
    /// at a single time and only distribute their tokens again once the gauge is
    /// refilled, Intended for use with incentives that get refilled daily.
    #[prost(bool, tag = "2")]
    pub is_perpetual: bool,
    /// distribute_to is where the gauge rewards are distributed to.
    /// This is queried via lock duration or by timestamp
    #[prost(message, optional, tag = "3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// coins is the total amount of coins that have been in the gauge
    /// Can distribute multiple coin denoms
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// start_time is the distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// num_epochs_paid_over is the number of total epochs distribution will be
    /// completed over
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_epochs_paid_over: u64,
    /// filled_epochs is the number of epochs distribution has been completed on
    /// already
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub filled_epochs: u64,
    /// distributed_coins are coins that have been distributed already
    #[prost(message, repeated, tag = "8")]
    pub distributed_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.LockableDurationsInfo")]
pub struct LockableDurationsInfo {
    /// List of incentivised durations that gauges will pay out to
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<crate::shim::Duration>,
}
/// Params holds parameters for the incentives module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.Params")]
pub struct Params {
    /// distr_epoch_identifier is what epoch type distribution will be triggered by
    /// (day, week, etc.)
    #[prost(string, tag = "1")]
    #[serde(alias = "distr_epochIDentifier")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
    /// group_creation_fee is the fee required to create a new group
    /// It is only charged to all addresses other than incentive module account
    /// or addresses in the unrestricted_creator_whitelist
    #[prost(message, repeated, tag = "2")]
    pub group_creation_fee: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// unrestricted_creator_whitelist is a list of addresses that are
    /// allowed to bypass restrictions on permissionless Group
    /// creation. In the future, we might expand these to creating gauges
    /// as well.
    /// The goal of this is to allow a subdao to manage incentives efficiently
    /// without being stopped by 5 day governance process or a high fee.
    /// At the same time, it prevents spam by having a fee for all
    /// other users.
    #[prost(string, repeated, tag = "3")]
    pub unrestricted_creator_whitelist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Note that while both InternalGaugeInfo and InternalGaugeRecord could
/// technically be replaced by DistrInfo and DistrRecord from the pool-incentives
/// module, we create separate types here to keep our abstractions clean and
/// readable (pool-incentives distribution abstractions are used in a very
/// specific way that does not directly relate to gauge logic). This also helps
/// us sidestep a refactor to avoid an import cycle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.InternalGaugeInfo")]
pub struct InternalGaugeInfo {
    #[prost(string, tag = "1")]
    pub total_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub gauge_records: ::prost::alloc::vec::Vec<InternalGaugeRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.InternalGaugeRecord")]
pub struct InternalGaugeRecord {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "gaugeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gauge_id: u64,
    /// CurrentWeight is the current weight of this gauge being distributed to for
    /// this epoch. For instance, for volume splitting policy, this stores the
    /// volume generated in the last epoch of the linked pool.
    #[prost(string, tag = "2")]
    pub current_weight: ::prost::alloc::string::String,
    /// CumulativeWeight serves as a snapshot of the accumulator being tracked
    /// based on splitting policy. For instance, for volume splitting policy, this
    /// stores the cumulative volume for the linked pool at time of last update.
    #[prost(string, tag = "3")]
    pub cumulative_weight: ::prost::alloc::string::String,
}
/// Group is an object that stores a 1:1 mapped gauge ID, a list of pool gauge
/// info, and a splitting policy. These are grouped into a single abstraction to
/// allow for distribution of group incentives to internal gauges according to
/// the specified splitting policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.Group")]
pub struct Group {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "group_gaugeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_gauge_id: u64,
    #[prost(message, optional, tag = "2")]
    pub internal_gauge_info: ::core::option::Option<InternalGaugeInfo>,
    #[prost(enumeration = "SplittingPolicy", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub splitting_policy: i32,
}
/// CreateGroup is called via governance to create a new group.
/// It takes an array of pool IDs to split the incentives across.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.CreateGroup")]
pub struct CreateGroup {
    #[prost(uint64, repeated, tag = "1")]
    #[serde(alias = "poolIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub pool_ids: ::prost::alloc::vec::Vec<u64>,
}
/// GroupsWithGauge is a helper struct that stores a group and its
/// associated gauge.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GroupsWithGauge")]
pub struct GroupsWithGauge {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<Group>,
    #[prost(message, optional, tag = "2")]
    pub gauge: ::core::option::Option<Gauge>,
}
/// SplittingPolicy determines the way we want to split incentives in groupGauges
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum SplittingPolicy {
    ByVolume = 0,
}
impl SplittingPolicy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SplittingPolicy::ByVolume => "ByVolume",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ByVolume" => Some(Self::ByVolume),
            _ => None,
        }
    }
}
/// GenesisState defines the incentives module's various parameters when first
/// initialized
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GenesisState")]
pub struct GenesisState {
    /// params are all the parameters of the module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// gauges are all gauges (not including group gauges) that should exist at
    /// genesis
    #[prost(message, repeated, tag = "2")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
    /// lockable_durations are all lockup durations that gauges can be locked for
    /// in order to recieve incentives
    #[prost(message, repeated, tag = "3")]
    pub lockable_durations: ::prost::alloc::vec::Vec<crate::shim::Duration>,
    /// last_gauge_id is what the gauge number will increment from when creating
    /// the next gauge after genesis
    #[prost(uint64, tag = "4")]
    #[serde(alias = "last_gaugeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_gauge_id: u64,
    /// gauges are all group gauges that should exist at genesis
    #[prost(message, repeated, tag = "5")]
    pub group_gauges: ::prost::alloc::vec::Vec<Gauge>,
    /// groups are all the groups that should exist at genesis
    #[prost(message, repeated, tag = "6")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
}
/// CreateGroupsProposal is a type for creating one or more groups via
/// governance. This is useful for creating groups without having to pay
/// creation fees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.CreateGroupsProposal")]
pub struct CreateGroupsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub create_groups: ::prost::alloc::vec::Vec<CreateGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ModuleToDistributeCoinsRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/ModuleToDistributeCoins",
    response_type = ModuleToDistributeCoinsResponse
)]
pub struct ModuleToDistributeCoinsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ModuleToDistributeCoinsResponse")]
pub struct ModuleToDistributeCoinsResponse {
    /// Coins that have yet to be distributed
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GaugeByIDRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/GaugeByID",
    response_type = GaugeByIdResponse
)]
pub struct GaugeByIdRequest {
    /// Gague ID being queried
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GaugeByIDResponse")]
pub struct GaugeByIdResponse {
    /// Gauge that corresponds to provided gague ID
    #[prost(message, optional, tag = "1")]
    pub gauge: ::core::option::Option<Gauge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GaugesRequest")]
#[proto_query(path = "/osmosis.incentives.Query/Gauges", response_type = GaugesResponse)]
pub struct GaugesRequest {
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GaugesResponse")]
pub struct GaugesResponse {
    /// Upcoming and active gauges
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ActiveGaugesRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/ActiveGauges",
    response_type = ActiveGaugesResponse
)]
pub struct ActiveGaugesRequest {
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ActiveGaugesResponse")]
pub struct ActiveGaugesResponse {
    /// Active gagues only
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ActiveGaugesPerDenomRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/ActiveGaugesPerDenom",
    response_type = ActiveGaugesPerDenomResponse
)]
pub struct ActiveGaugesPerDenomRequest {
    /// Desired denom when querying active gagues
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ActiveGaugesPerDenomResponse")]
pub struct ActiveGaugesPerDenomResponse {
    /// Active gagues that match denom in query
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.UpcomingGaugesRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/UpcomingGauges",
    response_type = UpcomingGaugesResponse
)]
pub struct UpcomingGaugesRequest {
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.UpcomingGaugesResponse")]
pub struct UpcomingGaugesResponse {
    /// Gauges whose distribution is upcoming
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.UpcomingGaugesPerDenomRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/UpcomingGaugesPerDenom",
    response_type = UpcomingGaugesPerDenomResponse
)]
pub struct UpcomingGaugesPerDenomRequest {
    /// Filter for upcoming gagues that match specific denom
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// Pagination defines pagination for the request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.UpcomingGaugesPerDenomResponse")]
pub struct UpcomingGaugesPerDenomResponse {
    /// Upcoming gagues that match denom in query
    #[prost(message, repeated, tag = "1")]
    pub upcoming_gauges: ::prost::alloc::vec::Vec<Gauge>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.RewardsEstRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/RewardsEst",
    response_type = RewardsEstResponse
)]
pub struct RewardsEstRequest {
    /// Address that is being queried for future estimated rewards
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// Lock IDs included in future reward estimation
    #[prost(uint64, repeated, tag = "2")]
    #[serde(alias = "lockIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub lock_ids: ::prost::alloc::vec::Vec<u64>,
    /// Upper time limit of reward estimation
    /// Lower limit is current epoch
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub end_epoch: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.RewardsEstResponse")]
pub struct RewardsEstResponse {
    /// Estimated coin rewards that will be recieved at provided address
    /// from specified locks between current time and end epoch
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryLockableDurationsRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/LockableDurations",
    response_type = QueryLockableDurationsResponse
)]
pub struct QueryLockableDurationsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryLockableDurationsResponse")]
pub struct QueryLockableDurationsResponse {
    /// Time durations that users can lock coins for in order to recieve rewards
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<crate::shim::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryAllGroupsRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/AllGroups",
    response_type = QueryAllGroupsResponse
)]
pub struct QueryAllGroupsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryAllGroupsResponse")]
pub struct QueryAllGroupsResponse {
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryAllGroupsGaugesRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/AllGroupsGauges",
    response_type = QueryAllGroupsGaugesResponse
)]
pub struct QueryAllGroupsGaugesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryAllGroupsGaugesResponse")]
pub struct QueryAllGroupsGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryAllGroupsWithGaugeRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/AllGroupsWithGauge",
    response_type = QueryAllGroupsWithGaugeResponse
)]
pub struct QueryAllGroupsWithGaugeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryAllGroupsWithGaugeResponse")]
pub struct QueryAllGroupsWithGaugeResponse {
    #[prost(message, repeated, tag = "1")]
    pub groups_with_gauge: ::prost::alloc::vec::Vec<GroupsWithGauge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryGroupByGroupGaugeIDRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/GroupByGroupGaugeID",
    response_type = QueryGroupByGroupGaugeIdResponse
)]
pub struct QueryGroupByGroupGaugeIdRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryGroupByGroupGaugeIDResponse")]
pub struct QueryGroupByGroupGaugeIdResponse {
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<Group>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryCurrentWeightByGroupGaugeIDRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/CurrentWeightByGroupGaugeID",
    response_type = QueryCurrentWeightByGroupGaugeIdResponse
)]
pub struct QueryCurrentWeightByGroupGaugeIdRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "group_gaugeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_gauge_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryCurrentWeightByGroupGaugeIDResponse")]
pub struct QueryCurrentWeightByGroupGaugeIdResponse {
    #[prost(message, repeated, tag = "1")]
    pub gauge_weight: ::prost::alloc::vec::Vec<GaugeWeight>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GaugeWeight")]
pub struct GaugeWeight {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "gaugeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gauge_id: u64,
    #[prost(string, tag = "2")]
    pub weight_ratio: ::prost::alloc::string::String,
}
/// MsgCreateGauge creates a gague to distribute rewards to users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgCreateGauge")]
pub struct MsgCreateGauge {
    /// is_perpetual shows if it's a perpetual or non-perpetual gauge
    /// Non-perpetual gauges distribute their tokens equally per epoch while the
    /// gauge is in the active period. Perpetual gauges distribute all their tokens
    /// at a single time and only distribute their tokens again once the gauge is
    /// refilled
    #[prost(bool, tag = "1")]
    pub is_perpetual: bool,
    /// owner is the address of gauge creator
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// distribute_to show which lock the gauge should distribute to by time
    /// duration or by timestamp
    #[prost(message, optional, tag = "3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// coins are coin(s) to be distributed by the gauge
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// start_time is the distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// num_epochs_paid_over is the number of epochs distribution will be completed
    /// over
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_epochs_paid_over: u64,
    /// pool_id is the ID of the pool that the gauge is meant to be associated
    /// with. if pool_id is set, then the "QueryCondition.LockQueryType" must be
    /// "NoLock" with all other fields of the "QueryCondition.LockQueryType" struct
    /// unset, including "QueryCondition.Denom". However, note that, internally,
    /// the empty string in "QueryCondition.Denom" ends up being overwritten with
    /// incentivestypes.NoLockExternalGaugeDenom(<pool-id>) so that the gauges
    /// associated with a pool can be queried by this prefix if needed.
    #[prost(uint64, tag = "7")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgCreateGaugeResponse")]
pub struct MsgCreateGaugeResponse {}
/// MsgAddToGauge adds coins to a previously created gauge
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgAddToGauge")]
pub struct MsgAddToGauge {
    /// owner is the gauge owner's address
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// gauge_id is the ID of gauge that rewards are getting added to
    #[prost(uint64, tag = "2")]
    #[serde(alias = "gaugeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gauge_id: u64,
    /// rewards are the coin(s) to add to gauge
    #[prost(message, repeated, tag = "3")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgAddToGaugeResponse")]
pub struct MsgAddToGaugeResponse {}
/// MsgCreateGroup creates a group to distribute rewards to a group of pools
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgCreateGroup")]
pub struct MsgCreateGroup {
    /// coins are the provided coins that the group will distribute
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// num_epochs_paid_over is the number of epochs distribution will be completed
    /// in. 0 means it's perpetual
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_epochs_paid_over: u64,
    /// owner is the group owner's address
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    /// pool_ids are the IDs of pools that the group is comprised of
    #[prost(uint64, repeated, tag = "4")]
    #[serde(alias = "poolIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub pool_ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgCreateGroupResponse")]
pub struct MsgCreateGroupResponse {
    /// group_id is the ID of the group that is created from this msg
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
}
pub struct IncentivesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> IncentivesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn module_to_distribute_coins(
        &self,
    ) -> Result<ModuleToDistributeCoinsResponse, cosmwasm_std::StdError> {
        ModuleToDistributeCoinsRequest {}.query(self.querier)
    }
    pub fn gauge_by_id(&self, id: u64) -> Result<GaugeByIdResponse, cosmwasm_std::StdError> {
        GaugeByIdRequest { id }.query(self.querier)
    }
    pub fn gauges(
        &self,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<GaugesResponse, cosmwasm_std::StdError> {
        GaugesRequest { pagination }.query(self.querier)
    }
    pub fn active_gauges(
        &self,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<ActiveGaugesResponse, cosmwasm_std::StdError> {
        ActiveGaugesRequest { pagination }.query(self.querier)
    }
    pub fn active_gauges_per_denom(
        &self,
        denom: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<ActiveGaugesPerDenomResponse, cosmwasm_std::StdError> {
        ActiveGaugesPerDenomRequest { denom, pagination }.query(self.querier)
    }
    pub fn upcoming_gauges(
        &self,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<UpcomingGaugesResponse, cosmwasm_std::StdError> {
        UpcomingGaugesRequest { pagination }.query(self.querier)
    }
    pub fn upcoming_gauges_per_denom(
        &self,
        denom: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<UpcomingGaugesPerDenomResponse, cosmwasm_std::StdError> {
        UpcomingGaugesPerDenomRequest { denom, pagination }.query(self.querier)
    }
    pub fn rewards_est(
        &self,
        owner: ::prost::alloc::string::String,
        lock_ids: ::prost::alloc::vec::Vec<u64>,
        end_epoch: i64,
    ) -> Result<RewardsEstResponse, cosmwasm_std::StdError> {
        RewardsEstRequest {
            owner,
            lock_ids,
            end_epoch,
        }
        .query(self.querier)
    }
    pub fn lockable_durations(
        &self,
    ) -> Result<QueryLockableDurationsResponse, cosmwasm_std::StdError> {
        QueryLockableDurationsRequest {}.query(self.querier)
    }
    pub fn all_groups(&self) -> Result<QueryAllGroupsResponse, cosmwasm_std::StdError> {
        QueryAllGroupsRequest {}.query(self.querier)
    }
    pub fn all_groups_gauges(
        &self,
    ) -> Result<QueryAllGroupsGaugesResponse, cosmwasm_std::StdError> {
        QueryAllGroupsGaugesRequest {}.query(self.querier)
    }
    pub fn all_groups_with_gauge(
        &self,
    ) -> Result<QueryAllGroupsWithGaugeResponse, cosmwasm_std::StdError> {
        QueryAllGroupsWithGaugeRequest {}.query(self.querier)
    }
    pub fn group_by_group_gauge_id(
        &self,
        id: u64,
    ) -> Result<QueryGroupByGroupGaugeIdResponse, cosmwasm_std::StdError> {
        QueryGroupByGroupGaugeIdRequest { id }.query(self.querier)
    }
    pub fn current_weight_by_group_gauge_id(
        &self,
        group_gauge_id: u64,
    ) -> Result<QueryCurrentWeightByGroupGaugeIdResponse, cosmwasm_std::StdError> {
        QueryCurrentWeightByGroupGaugeIdRequest { group_gauge_id }.query(self.querier)
    }
}
