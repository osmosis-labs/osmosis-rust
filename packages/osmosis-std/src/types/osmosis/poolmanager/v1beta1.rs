use osmosis_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.SwapAmountInRoute")]
pub struct SwapAmountInRoute {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_out_denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.SwapAmountOutRoute")]
pub struct SwapAmountOutRoute {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.SwapAmountInSplitRoute")]
pub struct SwapAmountInSplitRoute {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(string, tag = "2")]
    pub token_in_amount: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.SwapAmountOutSplitRoute")]
pub struct SwapAmountOutSplitRoute {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "2")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ModuleRouter defines a route encapsulating pool type.
/// It is used as the value of a mapping from pool id to the pool type,
/// allowing the pool manager to know which module to route swaps to given the
/// pool id.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.ModuleRoute")]
pub struct ModuleRoute {
    /// pool_type specifies the type of the pool
    #[prost(enumeration = "PoolType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_type: i32,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
/// PoolType is an enumeration of all supported pool types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum PoolType {
    /// Balancer is the standard xy=k curve. Its pool model is defined in x/gamm.
    Balancer = 0,
    /// Stableswap is the Solidly cfmm stable swap curve. Its pool model is defined
    /// in x/gamm.
    Stableswap = 1,
    /// Concentrated is the pool model specific to concentrated liquidity. It is
    /// defined in x/concentrated-liquidity.
    Concentrated = 2,
    /// CosmWasm is the pool model specific to CosmWasm. It is defined in
    /// x/cosmwasmpool.
    CosmWasm = 3,
}
impl PoolType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PoolType::Balancer => "Balancer",
            PoolType::Stableswap => "Stableswap",
            PoolType::Concentrated => "Concentrated",
            PoolType::CosmWasm => "CosmWasm",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Balancer" => Some(Self::Balancer),
            "Stableswap" => Some(Self::Stableswap),
            "Concentrated" => Some(Self::Concentrated),
            "CosmWasm" => Some(Self::CosmWasm),
            _ => None,
        }
    }
}
/// ===================== MsgSwapExactAmountIn
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSwapExactAmountIn")]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSwapExactAmountInResponse")]
pub struct MsgSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSplitRouteSwapExactAmountIn
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSplitRouteSwapExactAmountIn")]
pub struct MsgSplitRouteSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInSplitRoute>,
    #[prost(string, tag = "3")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSplitRouteSwapExactAmountInResponse")]
pub struct MsgSplitRouteSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSwapExactAmountOut
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSwapExactAmountOut")]
pub struct MsgSwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "3")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSwapExactAmountOutResponse")]
pub struct MsgSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSplitRouteSwapExactAmountOut
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSplitRouteSwapExactAmountOut")]
pub struct MsgSplitRouteSwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutSplitRoute>,
    #[prost(string, tag = "3")]
    pub token_out_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_in_max_amount: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSplitRouteSwapExactAmountOutResponse")]
pub struct MsgSplitRouteSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSetDenomPairTakerFee
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSetDenomPairTakerFee")]
pub struct MsgSetDenomPairTakerFee {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub denom_pair_taker_fee: ::prost::alloc::vec::Vec<DenomPairTakerFee>,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSetDenomPairTakerFeeResponse")]
pub struct MsgSetDenomPairTakerFeeResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.DenomPairTakerFee")]
pub struct DenomPairTakerFee {
    /// denom0 and denom1 get automatically lexigographically sorted
    /// when being stored, so the order of input here does not matter.
    #[prost(string, tag = "1")]
    pub denom0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom1: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub taker_fee: ::prost::alloc::string::String,
}
/// Params holds parameters for the poolmanager module
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.Params")]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub pool_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// taker_fee_params is the container of taker fee parameters.
    #[prost(message, optional, tag = "2")]
    pub taker_fee_params: ::core::option::Option<TakerFeeParams>,
    /// authorized_quote_denoms is a list of quote denoms that can be used as
    /// token1 when creating a concentrated pool. We limit the quote assets to a
    /// small set for the purposes of having convinient price increments stemming
    /// from tick to price conversion. These increments are in a human readable
    /// magnitude only for token1 as a quote. For limit orders in the future, this
    /// will be a desirable property in terms of UX as to allow users to set limit
    /// orders at prices in terms of token1 (quote asset) that are easy to reason
    /// about.
    #[prost(string, repeated, tag = "3")]
    pub authorized_quote_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GenesisState defines the poolmanager module's genesis state.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.GenesisState")]
pub struct GenesisState {
    /// the next_pool_id
    #[prost(uint64, tag = "1")]
    #[serde(alias = "next_poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_pool_id: u64,
    /// params is the container of poolmanager parameters.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
    /// pool_routes is the container of the mappings from pool id to pool type.
    #[prost(message, repeated, tag = "3")]
    pub pool_routes: ::prost::alloc::vec::Vec<ModuleRoute>,
    /// KVStore state
    #[prost(message, optional, tag = "4")]
    pub taker_fees_tracker: ::core::option::Option<TakerFeesTracker>,
    #[prost(message, repeated, tag = "5")]
    pub pool_volumes: ::prost::alloc::vec::Vec<PoolVolume>,
    #[prost(message, repeated, tag = "6")]
    pub denom_pair_taker_fee_store: ::prost::alloc::vec::Vec<DenomPairTakerFee>,
}
/// TakerFeeParams consolidates the taker fee parameters for the poolmanager.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TakerFeeParams")]
pub struct TakerFeeParams {
    /// default_taker_fee is the fee used when creating a new pool that doesn't
    /// fall under a custom pool taker fee or stableswap taker fee category.
    #[prost(string, tag = "1")]
    pub default_taker_fee: ::prost::alloc::string::String,
    /// osmo_taker_fee_distribution defines the distribution of taker fees
    /// generated in OSMO. As of this writing, it has two catagories:
    /// - staking_rewards: the percent of the taker fee that gets distributed to
    ///    stakers.
    /// - community_pool: the percent of the taker fee that gets sent to the
    ///    community pool.
    #[prost(message, optional, tag = "2")]
    pub osmo_taker_fee_distribution: ::core::option::Option<TakerFeeDistributionPercentage>,
    /// non_osmo_taker_fee_distribution defines the distribution of taker fees
    /// generated in non-OSMO. As of this writing, it has two categories:
    /// - staking_rewards: the percent of the taker fee that gets swapped to OSMO
    ///    and then distirbuted to stakers.
    /// - community_pool: the percent of the taker fee that gets sent to the
    ///    community pool. Note: If the non-OSMO asset is an authorized_quote_denom,
    ///    that denom is sent directly to the community pool. Otherwise, it is
    ///    swapped to the community_pool_denom_to_swap_non_whitelisted_assets_to and
    ///    then sent to the community pool as that denom.
    #[prost(message, optional, tag = "3")]
    pub non_osmo_taker_fee_distribution: ::core::option::Option<TakerFeeDistributionPercentage>,
    /// admin_addresses is a list of addresses that are allowed to set and remove
    /// custom taker fees for denom pairs. Governance also has the ability to set
    /// and remove custom taker fees for denom pairs, but with the normal
    /// governance delay.
    #[prost(string, repeated, tag = "4")]
    pub admin_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// community_pool_denom_to_swap_non_whitelisted_assets_to is the denom that
    /// non-whitelisted taker fees will be swapped to before being sent to
    /// the community pool.
    #[prost(string, tag = "5")]
    pub community_pool_denom_to_swap_non_whitelisted_assets_to: ::prost::alloc::string::String,
    /// reduced_fee_whitelist is a list of addresses that are
    /// allowed to pay a reduce taker fee when performing a swap
    /// (i.e. swap without paying the taker fee).
    /// It is intended to be used for integrators who meet qualifying factors
    /// that are approved by governance.
    /// Initially, the taker fee is allowed to be bypassed completely. However
    /// In the future, we will charge a reduced taker fee instead of no fee at all.
    #[prost(string, repeated, tag = "6")]
    pub reduced_fee_whitelist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TakerFeeDistributionPercentage defines what percent of the taker fee category
/// gets distributed to the available categories.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TakerFeeDistributionPercentage")]
pub struct TakerFeeDistributionPercentage {
    #[prost(string, tag = "1")]
    pub staking_rewards: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub community_pool: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TakerFeesTracker")]
pub struct TakerFeesTracker {
    #[prost(message, repeated, tag = "1")]
    pub taker_fees_to_stakers:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "2")]
    pub taker_fees_to_community_pool:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height_accounting_starts_from: i64,
}
/// PoolVolume stores the KVStore entries for each pool's volume, which
/// is used in export/import genesis.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.PoolVolume")]
pub struct PoolVolume {
    /// pool_id is the id of the pool.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    /// pool_volume is the cumulative volume of the pool.
    #[prost(message, repeated, tag = "2")]
    pub pool_volume: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// DenomPairTakerFeeProposal is a type for adding/removing a custom taker fee(s)
/// for one or more denom pairs.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.DenomPairTakerFeeProposal")]
pub struct DenomPairTakerFeeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub denom_pair_taker_fee: ::prost::alloc::vec::Vec<DenomPairTakerFee>,
}
/// =============================== Params
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.ParamsRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/Params",
    response_type = ParamsResponse
)]
pub struct ParamsRequest {}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.ParamsResponse")]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// =============================== EstimateSwapExactAmountIn
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountInRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn",
    response_type = EstimateSwapExactAmountInResponse
)]
pub struct EstimateSwapExactAmountInRequest {
    #[deprecated]
    #[prost(uint64, tag = "2")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
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
#[proto_message(
    type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountInWithPrimitiveTypesRequest"
)]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountInWithPrimitiveTypes",
    response_type = EstimateSwapExactAmountInResponse
)]
pub struct EstimateSwapExactAmountInWithPrimitiveTypesRequest {
    #[deprecated]
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    #[serde(alias = "routes_poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub routes_pool_id: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, repeated, tag = "4")]
    pub routes_token_out_denom: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(
    type_url = "/osmosis.poolmanager.v1beta1.EstimateSinglePoolSwapExactAmountInRequest"
)]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/EstimateSinglePoolSwapExactAmountIn",
    response_type = EstimateSwapExactAmountInResponse
)]
pub struct EstimateSinglePoolSwapExactAmountInRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_out_denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountInResponse")]
pub struct EstimateSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// =============================== EstimateSwapExactAmountOut
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountOutRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut",
    response_type = EstimateSwapExactAmountOutResponse
)]
pub struct EstimateSwapExactAmountOutRequest {
    #[deprecated]
    #[prost(uint64, tag = "2")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountOutWithPrimitiveTypesRequest"
)]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOutWithPrimitiveTypes",
    response_type = EstimateSwapExactAmountOutResponse
)]
pub struct EstimateSwapExactAmountOutWithPrimitiveTypesRequest {
    #[deprecated]
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    #[serde(alias = "routes_poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub routes_pool_id: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, repeated, tag = "3")]
    pub routes_token_in_denom: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/osmosis.poolmanager.v1beta1.EstimateSinglePoolSwapExactAmountOutRequest"
)]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/EstimateSinglePoolSwapExactAmountOut",
    response_type = EstimateSwapExactAmountOutResponse
)]
pub struct EstimateSinglePoolSwapExactAmountOutRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_out: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountOutResponse")]
pub struct EstimateSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// =============================== NumPools
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.NumPoolsRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/NumPools",
    response_type = NumPoolsResponse
)]
pub struct NumPoolsRequest {}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.NumPoolsResponse")]
pub struct NumPoolsResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_pools: u64,
}
/// =============================== Pool
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.PoolRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/Pool",
    response_type = PoolResponse
)]
pub struct PoolRequest {
    #[prost(uint64, tag = "1")]
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.PoolResponse")]
pub struct PoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<crate::shim::Any>,
}
/// =============================== AllPools
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.AllPoolsRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/AllPools",
    response_type = AllPoolsResponse
)]
pub struct AllPoolsRequest {}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.AllPoolsResponse")]
pub struct AllPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// =======================================================
/// ListPoolsByDenomRequest
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.ListPoolsByDenomRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/ListPoolsByDenom",
    response_type = ListPoolsByDenomResponse
)]
pub struct ListPoolsByDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.ListPoolsByDenomResponse")]
pub struct ListPoolsByDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// ==========================================================
/// SpotPriceRequest defines the gRPC request structure for a SpotPrice
/// query.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.SpotPriceRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/SpotPrice",
    response_type = SpotPriceResponse
)]
pub struct SpotPriceRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset_denom: ::prost::alloc::string::String,
}
/// SpotPriceResponse defines the gRPC response structure for a SpotPrice
/// query.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.SpotPriceResponse")]
pub struct SpotPriceResponse {
    /// String of the Dec. Ex) 10.203uatom
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}
/// =============================== TotalPoolLiquidity
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TotalPoolLiquidityRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/TotalPoolLiquidity",
    response_type = TotalPoolLiquidityResponse
)]
pub struct TotalPoolLiquidityRequest {
    #[prost(uint64, tag = "1")]
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TotalPoolLiquidityResponse")]
pub struct TotalPoolLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// =============================== TotalLiquidity
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TotalLiquidityRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/TotalLiquidity",
    response_type = TotalLiquidityResponse
)]
pub struct TotalLiquidityRequest {}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TotalLiquidityResponse")]
pub struct TotalLiquidityResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// =============================== TotalVolumeForPool
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TotalVolumeForPoolRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/TotalVolumeForPool",
    response_type = TotalVolumeForPoolResponse
)]
pub struct TotalVolumeForPoolRequest {
    #[prost(uint64, tag = "1")]
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TotalVolumeForPoolResponse")]
pub struct TotalVolumeForPoolResponse {
    #[prost(message, repeated, tag = "1")]
    pub volume: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// =============================== TradingPairTakerFee
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TradingPairTakerFeeRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/TradingPairTakerFee",
    response_type = TradingPairTakerFeeResponse
)]
pub struct TradingPairTakerFeeRequest {
    #[prost(string, tag = "1")]
    pub denom_0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom_1: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TradingPairTakerFeeResponse")]
pub struct TradingPairTakerFeeResponse {
    #[prost(string, tag = "1")]
    pub taker_fee: ::prost::alloc::string::String,
}
/// EstimateTradeBasedOnPriceImpactRequest represents a request to estimate a
/// trade for Balancer/StableSwap/Concentrated liquidity pool types based on the
/// given parameters.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateTradeBasedOnPriceImpactRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/EstimateTradeBasedOnPriceImpact",
    response_type = EstimateTradeBasedOnPriceImpactResponse
)]
pub struct EstimateTradeBasedOnPriceImpactRequest {
    /// from_coin is the total amount of tokens that the user wants to sell.
    #[prost(message, optional, tag = "1")]
    pub from_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// to_coin_denom is the denom identifier of the token that the user wants to
    /// buy.
    #[prost(string, tag = "2")]
    pub to_coin_denom: ::prost::alloc::string::String,
    /// pool_id is the identifier of the liquidity pool that the trade will occur
    /// on.
    #[prost(uint64, tag = "3")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    /// max_price_impact is the maximum percentage that the user is willing
    /// to affect the price of the liquidity pool.
    #[prost(string, tag = "4")]
    pub max_price_impact: ::prost::alloc::string::String,
    /// external_price is an optional external price that the user can enter.
    /// It adjusts the MaxPriceImpact as the SpotPrice of a pool can be changed at
    /// any time.
    #[prost(string, tag = "5")]
    pub external_price: ::prost::alloc::string::String,
}
/// EstimateTradeBasedOnPriceImpactResponse represents the response data
/// for an estimated trade based on price impact. If a trade fails to be
/// estimated the response would be 0,0 for input_coin and output_coin and will
/// not error.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateTradeBasedOnPriceImpactResponse")]
pub struct EstimateTradeBasedOnPriceImpactResponse {
    /// input_coin is the actual input amount that would be tradeable
    /// under the specified price impact.
    #[prost(message, optional, tag = "1")]
    pub input_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// output_coin is the amount of tokens of the ToCoinDenom type
    /// that will be received for the actual InputCoin trade.
    #[prost(message, optional, tag = "2")]
    pub output_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.TrackedVolume")]
pub struct TrackedVolume {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
pub struct PoolmanagerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> PoolmanagerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<ParamsResponse, cosmwasm_std::StdError> {
        ParamsRequest {}.query(self.querier)
    }
    pub fn estimate_swap_exact_amount_in(
        &self,
        pool_id: u64,
        token_in: ::prost::alloc::string::String,
        routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    ) -> Result<EstimateSwapExactAmountInResponse, cosmwasm_std::StdError> {
        EstimateSwapExactAmountInRequest {
            pool_id,
            token_in,
            routes,
        }
        .query(self.querier)
    }
    pub fn estimate_swap_exact_amount_in_with_primitive_types(
        &self,
        pool_id: u64,
        token_in: ::prost::alloc::string::String,
        routes_pool_id: ::prost::alloc::vec::Vec<u64>,
        routes_token_out_denom: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ) -> Result<EstimateSwapExactAmountInResponse, cosmwasm_std::StdError> {
        EstimateSwapExactAmountInWithPrimitiveTypesRequest {
            pool_id,
            token_in,
            routes_pool_id,
            routes_token_out_denom,
        }
        .query(self.querier)
    }
    pub fn estimate_single_pool_swap_exact_amount_in(
        &self,
        pool_id: u64,
        token_in: ::prost::alloc::string::String,
        token_out_denom: ::prost::alloc::string::String,
    ) -> Result<EstimateSwapExactAmountInResponse, cosmwasm_std::StdError> {
        EstimateSinglePoolSwapExactAmountInRequest {
            pool_id,
            token_in,
            token_out_denom,
        }
        .query(self.querier)
    }
    pub fn estimate_swap_exact_amount_out(
        &self,
        pool_id: u64,
        routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
        token_out: ::prost::alloc::string::String,
    ) -> Result<EstimateSwapExactAmountOutResponse, cosmwasm_std::StdError> {
        EstimateSwapExactAmountOutRequest {
            pool_id,
            routes,
            token_out,
        }
        .query(self.querier)
    }
    pub fn estimate_swap_exact_amount_out_with_primitive_types(
        &self,
        pool_id: u64,
        routes_pool_id: ::prost::alloc::vec::Vec<u64>,
        routes_token_in_denom: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        token_out: ::prost::alloc::string::String,
    ) -> Result<EstimateSwapExactAmountOutResponse, cosmwasm_std::StdError> {
        EstimateSwapExactAmountOutWithPrimitiveTypesRequest {
            pool_id,
            routes_pool_id,
            routes_token_in_denom,
            token_out,
        }
        .query(self.querier)
    }
    pub fn estimate_single_pool_swap_exact_amount_out(
        &self,
        pool_id: u64,
        token_in_denom: ::prost::alloc::string::String,
        token_out: ::prost::alloc::string::String,
    ) -> Result<EstimateSwapExactAmountOutResponse, cosmwasm_std::StdError> {
        EstimateSinglePoolSwapExactAmountOutRequest {
            pool_id,
            token_in_denom,
            token_out,
        }
        .query(self.querier)
    }
    pub fn num_pools(&self) -> Result<NumPoolsResponse, cosmwasm_std::StdError> {
        NumPoolsRequest {}.query(self.querier)
    }
    pub fn pool(&self, pool_id: u64) -> Result<PoolResponse, cosmwasm_std::StdError> {
        PoolRequest { pool_id }.query(self.querier)
    }
    pub fn all_pools(&self) -> Result<AllPoolsResponse, cosmwasm_std::StdError> {
        AllPoolsRequest {}.query(self.querier)
    }
    pub fn list_pools_by_denom(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<ListPoolsByDenomResponse, cosmwasm_std::StdError> {
        ListPoolsByDenomRequest { denom }.query(self.querier)
    }
    pub fn spot_price(
        &self,
        pool_id: u64,
        base_asset_denom: ::prost::alloc::string::String,
        quote_asset_denom: ::prost::alloc::string::String,
    ) -> Result<SpotPriceResponse, cosmwasm_std::StdError> {
        SpotPriceRequest {
            pool_id,
            base_asset_denom,
            quote_asset_denom,
        }
        .query(self.querier)
    }
    pub fn total_pool_liquidity(
        &self,
        pool_id: u64,
    ) -> Result<TotalPoolLiquidityResponse, cosmwasm_std::StdError> {
        TotalPoolLiquidityRequest { pool_id }.query(self.querier)
    }
    pub fn total_liquidity(&self) -> Result<TotalLiquidityResponse, cosmwasm_std::StdError> {
        TotalLiquidityRequest {}.query(self.querier)
    }
    pub fn total_volume_for_pool(
        &self,
        pool_id: u64,
    ) -> Result<TotalVolumeForPoolResponse, cosmwasm_std::StdError> {
        TotalVolumeForPoolRequest { pool_id }.query(self.querier)
    }
    pub fn trading_pair_taker_fee(
        &self,
        denom_0: ::prost::alloc::string::String,
        denom_1: ::prost::alloc::string::String,
    ) -> Result<TradingPairTakerFeeResponse, cosmwasm_std::StdError> {
        TradingPairTakerFeeRequest { denom_0, denom_1 }.query(self.querier)
    }
    pub fn estimate_trade_based_on_price_impact(
        &self,
        from_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
        to_coin_denom: ::prost::alloc::string::String,
        pool_id: u64,
        max_price_impact: ::prost::alloc::string::String,
        external_price: ::prost::alloc::string::String,
    ) -> Result<EstimateTradeBasedOnPriceImpactResponse, cosmwasm_std::StdError> {
        EstimateTradeBasedOnPriceImpactRequest {
            from_coin,
            to_coin_denom,
            pool_id,
            max_price_impact,
            external_price,
        }
        .query(self.querier)
    }
}
