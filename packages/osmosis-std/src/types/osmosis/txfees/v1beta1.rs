use osmosis_std_derive::CosmwasmExt;
/// FeeToken is a struct that specifies a coin denom, and pool ID pair.
/// This marks the token as eligible for use as a tx fee asset in Osmosis.
/// Its price in osmo is derived through looking at the provided pool ID.
/// The pool ID must have osmo as one of its assets.
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.FeeToken")]
pub struct FeeToken {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
/// Params holds parameters for the txfees module
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.Params")]
pub struct Params {
    #[prost(string, repeated, tag = "1")]
    pub whitelisted_fee_token_setters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GenesisState defines the txfees module's genesis state.
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(string, tag = "1")]
    pub basedenom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub feetokens: ::prost::alloc::vec::Vec<FeeToken>,
    /// DEPRECATED
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub tx_fees_tracker: ::core::option::Option<TxFeesTracker>,
    /// params is the container of txfees parameters.
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.TxFeesTracker")]
pub struct TxFeesTracker {
    #[prost(message, repeated, tag = "1")]
    pub tx_fees: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height_accounting_starts_from: i64,
}
/// UpdateFeeTokenProposal is a gov Content type for adding new whitelisted fee
/// token(s). It must specify a denom along with gamm pool ID to use as a spot
/// price calculator. It can be used to add new denoms to the whitelist. It can
/// also be used to update the Pool to associate with the denom. If Pool ID is
/// set to 0, it will remove the denom from the whitelisted set.
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.UpdateFeeTokenProposal")]
pub struct UpdateFeeTokenProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub feetokens: ::prost::alloc::vec::Vec<FeeToken>,
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryFeeTokensRequest")]
#[proto_query(
    path = "/osmosis.txfees.v1beta1.Query/FeeTokens",
    response_type = QueryFeeTokensResponse
)]
pub struct QueryFeeTokensRequest {}
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryFeeTokensResponse")]
pub struct QueryFeeTokensResponse {
    #[prost(message, repeated, tag = "1")]
    pub fee_tokens: ::prost::alloc::vec::Vec<FeeToken>,
}
/// QueryDenomSpotPriceRequest defines grpc request structure for querying spot
/// price for the specified tx fee denom
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryDenomSpotPriceRequest")]
#[proto_query(
    path = "/osmosis.txfees.v1beta1.Query/DenomSpotPrice",
    response_type = QueryDenomSpotPriceResponse
)]
pub struct QueryDenomSpotPriceRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryDenomSpotPriceRequest defines grpc response structure for querying spot
/// price for the specified tx fee denom
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryDenomSpotPriceResponse")]
pub struct QueryDenomSpotPriceResponse {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub spot_price: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryDenomPoolIdRequest")]
#[proto_query(
    path = "/osmosis.txfees.v1beta1.Query/DenomPoolId",
    response_type = QueryDenomPoolIdResponse
)]
pub struct QueryDenomPoolIdRequest {
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryDenomPoolIdResponse")]
pub struct QueryDenomPoolIdResponse {
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryBaseDenomRequest")]
#[proto_query(
    path = "/osmosis.txfees.v1beta1.Query/BaseDenom",
    response_type = QueryBaseDenomResponse
)]
pub struct QueryBaseDenomRequest {}
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryBaseDenomResponse")]
pub struct QueryBaseDenomResponse {
    #[prost(string, tag = "1")]
    pub base_denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryEipBaseFeeRequest")]
#[proto_query(
    path = "/osmosis.txfees.v1beta1.Query/GetEipBaseFee",
    response_type = QueryEipBaseFeeResponse
)]
pub struct QueryEipBaseFeeRequest {}
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.QueryEipBaseFeeResponse")]
pub struct QueryEipBaseFeeResponse {
    #[prost(string, tag = "1")]
    pub base_fee: ::prost::alloc::string::String,
}
/// ===================== MsgSetFeeTokens
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.MsgSetFeeTokens")]
pub struct MsgSetFeeTokens {
    #[prost(message, repeated, tag = "1")]
    pub fee_tokens: ::prost::alloc::vec::Vec<FeeToken>,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.txfees.v1beta1.MsgSetFeeTokensResponse")]
pub struct MsgSetFeeTokensResponse {}
pub struct TxfeesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TxfeesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn fee_tokens(&self) -> Result<QueryFeeTokensResponse, cosmwasm_std::StdError> {
        QueryFeeTokensRequest {}.query(self.querier)
    }
    pub fn denom_spot_price(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryDenomSpotPriceResponse, cosmwasm_std::StdError> {
        QueryDenomSpotPriceRequest { denom }.query(self.querier)
    }
    pub fn denom_pool_id(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryDenomPoolIdResponse, cosmwasm_std::StdError> {
        QueryDenomPoolIdRequest { denom }.query(self.querier)
    }
    pub fn base_denom(&self) -> Result<QueryBaseDenomResponse, cosmwasm_std::StdError> {
        QueryBaseDenomRequest {}.query(self.querier)
    }
    pub fn get_eip_base_fee(&self) -> Result<QueryEipBaseFeeResponse, cosmwasm_std::StdError> {
        QueryEipBaseFeeRequest {}.query(self.querier)
    }
}
