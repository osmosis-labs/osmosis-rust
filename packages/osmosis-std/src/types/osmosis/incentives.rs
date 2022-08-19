use osmosis_std_derive::CosmwasmExt;
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.Gauge")]
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
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// number of epochs distribution will be done
    #[prost(uint64, tag = "6")]
    pub num_epochs_paid_over: u64,
    /// number of epochs distributed already
    #[prost(uint64, tag = "7")]
    pub filled_epochs: u64,
    /// already distributed coins
    #[prost(message, repeated, tag = "8")]
    pub distributed_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.LockableDurationsInfo")]
pub struct LockableDurationsInfo {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<crate::shim::Duration>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgCreateGauge")]
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
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// number of epochs distribution will be done
    #[prost(uint64, tag = "6")]
    pub num_epochs_paid_over: u64,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgCreateGaugeResponse")]
pub struct MsgCreateGaugeResponse {}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgAddToGauge")]
pub struct MsgAddToGauge {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub gauge_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.MsgAddToGaugeResponse")]
pub struct MsgAddToGaugeResponse {}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ModuleToDistributeCoinsRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/ModuleToDistributeCoins",
    response_type = ModuleToDistributeCoinsResponse
)]
pub struct ModuleToDistributeCoinsRequest {}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ModuleToDistributeCoinsResponse")]
pub struct ModuleToDistributeCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ModuleDistributedCoinsRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/ModuleDistributedCoins",
    response_type = ModuleDistributedCoinsResponse
)]
pub struct ModuleDistributedCoinsRequest {}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ModuleDistributedCoinsResponse")]
pub struct ModuleDistributedCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GaugeByIDRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/GaugeByID",
    response_type = GaugeByIdResponse
)]
pub struct GaugeByIdRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GaugeByIDResponse")]
pub struct GaugeByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub gauge: ::core::option::Option<Gauge>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GaugesRequest")]
#[proto_query(path = "/osmosis.incentives.Query/Gauges", response_type = GaugesResponse)]
pub struct GaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GaugesResponse")]
pub struct GaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ActiveGaugesRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/ActiveGauges",
    response_type = ActiveGaugesResponse
)]
pub struct ActiveGaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ActiveGaugesResponse")]
pub struct ActiveGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ActiveGaugesPerDenomRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/ActiveGaugesPerDenom",
    response_type = ActiveGaugesPerDenomResponse
)]
pub struct ActiveGaugesPerDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.ActiveGaugesPerDenomResponse")]
pub struct ActiveGaugesPerDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.UpcomingGaugesRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/UpcomingGauges",
    response_type = UpcomingGaugesResponse
)]
pub struct UpcomingGaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.UpcomingGaugesResponse")]
pub struct UpcomingGaugesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.UpcomingGaugesPerDenomRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/UpcomingGaugesPerDenom",
    response_type = UpcomingGaugesPerDenomResponse
)]
pub struct UpcomingGaugesPerDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.UpcomingGaugesPerDenomResponse")]
pub struct UpcomingGaugesPerDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub upcoming_gauges: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.RewardsEstRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/RewardsEst",
    response_type = RewardsEstResponse
)]
pub struct RewardsEstRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag = "2")]
    pub lock_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(int64, tag = "3")]
    pub end_epoch: i64,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.RewardsEstResponse")]
pub struct RewardsEstResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryLockableDurationsRequest")]
#[proto_query(
    path = "/osmosis.incentives.Query/LockableDurations",
    response_type = QueryLockableDurationsResponse
)]
pub struct QueryLockableDurationsRequest {}
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.QueryLockableDurationsResponse")]
pub struct QueryLockableDurationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<crate::shim::Duration>,
}
/// Params holds parameters for the incentives module
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.Params")]
pub struct Params {
    /// distribution epoch identifier
    #[prost(string, tag = "1")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
}
/// GenesisState defines the incentives module's genesis state.
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.incentives.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, repeated, tag = "3")]
    pub lockable_durations: ::prost::alloc::vec::Vec<crate::shim::Duration>,
    #[prost(uint64, tag = "4")]
    pub last_gauge_id: u64,
}
pub struct IncentivesQuerier<'a> {
    querier: cosmwasm_std::QuerierWrapper<'a, cosmwasm_std::Empty>,
}
impl<'a> IncentivesQuerier<'a> {
    pub fn new(querier: cosmwasm_std::QuerierWrapper<'a, cosmwasm_std::Empty>) -> Self {
        Self { querier }
    }
    pub fn module_to_distribute_coins(
        &self,
    ) -> Result<ModuleToDistributeCoinsResponse, cosmwasm_std::StdError> {
        ModuleToDistributeCoinsRequest {}.query(self.querier)
    }
    pub fn module_distributed_coins(
        &self,
    ) -> Result<ModuleDistributedCoinsResponse, cosmwasm_std::StdError> {
        ModuleDistributedCoinsRequest {}.query(self.querier)
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
}
