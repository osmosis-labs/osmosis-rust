use osmosis_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/osmosis.gamm.twap.v1beta1.GetArithmeticTwapRequest")]
#[proto_query(
    path = "/osmosis.gamm.twap.v1beta1.Query/GetArithmeticTwap",
    response_type = GetArithmeticTwapResponse
)]
pub struct GetArithmeticTwapRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_asset: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<crate::shim::Timestamp>,
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
#[proto_message(type_url = "/osmosis.gamm.twap.v1beta1.GetArithmeticTwapResponse")]
pub struct GetArithmeticTwapResponse {
    #[prost(string, tag = "1")]
    pub arithmetic_twap: ::prost::alloc::string::String,
}
pub struct TwapQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TwapQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn get_arithmetic_twap(
        &self,
        pool_id: u64,
        base_asset: ::prost::alloc::string::String,
        quote_asset: ::prost::alloc::string::String,
        start_time: ::core::option::Option<crate::shim::Timestamp>,
        end_time: ::core::option::Option<crate::shim::Timestamp>,
    ) -> Result<GetArithmeticTwapResponse, cosmwasm_std::StdError> {
        GetArithmeticTwapRequest {
            pool_id,
            base_asset,
            quote_asset,
            start_time,
            end_time,
        }
        .query(self.querier)
    }
}
