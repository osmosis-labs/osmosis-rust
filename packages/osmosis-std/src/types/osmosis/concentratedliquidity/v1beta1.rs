use osmosis_std_derive::CosmwasmExt;
/// ===================== MsgCreatePosition
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCreatePosition")]
pub struct MsgCreatePosition {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick: i64,
    #[prost(message, optional, tag = "5")]
    pub token_desired0: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub token_desired1: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "7")]
    pub token_min_amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub token_min_amount1: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub frozen_until: ::core::option::Option<crate::shim::Timestamp>,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCreatePositionResponse")]
pub struct MsgCreatePositionResponse {
    #[prost(string, tag = "1")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub liquidity_created: ::prost::alloc::string::String,
}
/// ===================== MsgWithdrawPosition
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgWithdrawPosition")]
pub struct MsgWithdrawPosition {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick: i64,
    #[prost(string, tag = "5")]
    pub liquidity_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub frozen_until: ::core::option::Option<crate::shim::Timestamp>,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgWithdrawPositionResponse")]
pub struct MsgWithdrawPositionResponse {
    #[prost(string, tag = "1")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount1: ::prost::alloc::string::String,
}
/// ===================== MsgCollectFees
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCollectFees")]
pub struct MsgCollectFees {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick: i64,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCollectFeesResponse")]
pub struct MsgCollectFeesResponse {
    #[prost(message, repeated, tag = "1")]
    pub token_out: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.TickInfo")]
pub struct TickInfo {
    #[prost(string, tag = "1")]
    pub liquidity_gross: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub liquidity_net: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub fee_growth_outside:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.Pool")]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// Amount of total liquidity
    #[prost(string, tag = "3")]
    pub liquidity: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token0: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token1: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub current_sqrt_price: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub current_tick: ::prost::alloc::string::String,
    /// tick_spacing must be one of the authorized_tick_spacing values set in the
    /// concentrated-liquidity parameters
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_spacing: u64,
    #[prost(string, tag = "9")]
    pub precision_factor_at_price_one: ::prost::alloc::string::String,
    /// swap_fee is the ratio that is charged on the amount of token in.
    #[prost(string, tag = "10")]
    pub swap_fee: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.Position")]
pub struct Position {
    #[prost(string, tag = "1")]
    pub liquidity: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub frozen_until: ::core::option::Option<crate::shim::Timestamp>,
}
/// ===================== MsgCreateConcentratedPool
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCreateConcentratedPool")]
pub struct MsgCreateConcentratedPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom0: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom1: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_spacing: u64,
    #[prost(string, tag = "5")]
    pub precision_factor_at_price_one: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub swap_fee: ::prost::alloc::string::String,
}
/// Returns a unique poolID to identify the pool with.
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
#[proto_message(
    type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCreateConcentratedPoolResponse"
)]
pub struct MsgCreateConcentratedPoolResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
///=============================== Positions
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryUserPositionsRequest")]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/UserPositions",
    response_type = QueryUserPositionsResponse
)]
pub struct QueryUserPositionsRequest {
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryUserPositionsResponse")]
pub struct QueryUserPositionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub positions: ::prost::alloc::vec::Vec<FullPositionByOwnerResult>,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.FullPositionByOwnerResult")]
pub struct FullPositionByOwnerResult {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick: i64,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick: i64,
    #[prost(message, optional, tag = "4")]
    pub frozen_until: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "5")]
    pub liquidity: ::prost::alloc::string::String,
}
///=============================== Pool
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryPoolRequest")]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/Pool",
    response_type = QueryPoolResponse
)]
pub struct QueryPoolRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryPoolResponse")]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<crate::shim::Any>,
}
///=============================== Pools
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryPoolsRequest")]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/Pools",
    response_type = QueryPoolsResponse
)]
pub struct QueryPoolsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryPoolsResponse")]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
///=============================== ModuleParams
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::Params>,
}
///=============================== LiquidityDepthsForRange
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
#[proto_message(
    type_url = "/osmosis.concentratedliquidity.v1beta1.QueryLiquidityDepthsForRangeRequest"
)]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/LiquidityDepthsForRange",
    response_type = QueryLiquidityDepthsForRangeResponse
)]
pub struct QueryLiquidityDepthsForRangeRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub lower_tick: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub upper_tick: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/osmosis.concentratedliquidity.v1beta1.QueryLiquidityDepthsForRangeResponse"
)]
pub struct QueryLiquidityDepthsForRangeResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity_depths: ::prost::alloc::vec::Vec<LiquidityDepth>,
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.LiquidityDepth")]
pub struct LiquidityDepth {
    #[prost(string, tag = "1")]
    pub liquidity_net: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tick_index: ::prost::alloc::string::String,
}
/// GenesisState defines the concentrated liquidity module's genesis state.
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params are all the parameters of the module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::Params>,
    #[prost(message, repeated, tag = "2")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
pub struct ConcentratedliquidityQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ConcentratedliquidityQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn pools(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPoolsResponse, cosmwasm_std::StdError> {
        QueryPoolsRequest { pagination }.query(self.querier)
    }
    pub fn pool(&self, pool_id: u64) -> Result<QueryPoolResponse, cosmwasm_std::StdError> {
        QueryPoolRequest { pool_id }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn liquidity_depths_for_range(
        &self,
        pool_id: u64,
        lower_tick: ::prost::alloc::string::String,
        upper_tick: ::prost::alloc::string::String,
    ) -> Result<QueryLiquidityDepthsForRangeResponse, cosmwasm_std::StdError> {
        QueryLiquidityDepthsForRangeRequest {
            pool_id,
            lower_tick,
            upper_tick,
        }
        .query(self.querier)
    }
    pub fn user_positions(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryUserPositionsResponse, cosmwasm_std::StdError> {
        QueryUserPositionsRequest { address }.query(self.querier)
    }
}
