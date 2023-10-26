use osmosis_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/osmosis.poolmanager.v2.SpotPriceRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v2.Query/SpotPriceV2",
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
#[proto_message(type_url = "/osmosis.poolmanager.v2.SpotPriceResponse")]
pub struct SpotPriceResponse {
    /// String of the BigDec. Ex) 10.203uatom
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}
pub struct PoolmanagerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> PoolmanagerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn spot_price_v2(
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
}
