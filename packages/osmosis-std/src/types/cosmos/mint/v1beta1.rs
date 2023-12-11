use osmosis_std_derive::CosmwasmExt;
/// Minter represents the minting state.
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.Minter")]
pub struct Minter {
    /// current annual inflation rate
    #[prost(string, tag = "1")]
    pub inflation: ::prost::alloc::string::String,
    /// current annual expected provisions
    #[prost(string, tag = "2")]
    pub annual_provisions: ::prost::alloc::string::String,
}
/// Params defines the parameters for the x/mint module.
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.Params")]
pub struct Params {
    /// type of coin to mint
    #[prost(string, tag = "1")]
    pub mint_denom: ::prost::alloc::string::String,
    /// maximum annual change in inflation rate
    #[prost(string, tag = "2")]
    pub inflation_rate_change: ::prost::alloc::string::String,
    /// maximum inflation rate
    #[prost(string, tag = "3")]
    pub inflation_max: ::prost::alloc::string::String,
    /// minimum inflation rate
    #[prost(string, tag = "4")]
    pub inflation_min: ::prost::alloc::string::String,
    /// goal of percent bonded atoms
    #[prost(string, tag = "5")]
    pub goal_bonded: ::prost::alloc::string::String,
    /// expected blocks per year
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub blocks_per_year: u64,
}
/// GenesisState defines the mint module's genesis state.
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.GenesisState")]
pub struct GenesisState {
    /// minter is a space for holding current inflation information.
    #[prost(message, optional, tag = "1")]
    pub minter: ::core::option::Option<Minter>,
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.mint.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryInflationRequest is the request type for the Query/Inflation RPC method.
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.QueryInflationRequest")]
#[proto_query(
    path = "/cosmos.mint.v1beta1.Query/Inflation",
    response_type = QueryInflationResponse
)]
pub struct QueryInflationRequest {}
/// QueryInflationResponse is the response type for the Query/Inflation RPC
/// method.
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.QueryInflationResponse")]
pub struct QueryInflationResponse {
    /// inflation is the current minting inflation value.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub inflation: ::prost::alloc::vec::Vec<u8>,
}
/// QueryAnnualProvisionsRequest is the request type for the
/// Query/AnnualProvisions RPC method.
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.QueryAnnualProvisionsRequest")]
#[proto_query(
    path = "/cosmos.mint.v1beta1.Query/AnnualProvisions",
    response_type = QueryAnnualProvisionsResponse
)]
pub struct QueryAnnualProvisionsRequest {}
/// QueryAnnualProvisionsResponse is the response type for the
/// Query/AnnualProvisions RPC method.
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.QueryAnnualProvisionsResponse")]
pub struct QueryAnnualProvisionsResponse {
    /// annual_provisions is the current minting annual provisions value.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub annual_provisions: ::prost::alloc::vec::Vec<u8>,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/mint parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/cosmos.mint.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct MintQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> MintQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn inflation(&self) -> Result<QueryInflationResponse, cosmwasm_std::StdError> {
        QueryInflationRequest {}.query(self.querier)
    }
    pub fn annual_provisions(
        &self,
    ) -> Result<QueryAnnualProvisionsResponse, cosmwasm_std::StdError> {
        QueryAnnualProvisionsRequest {}.query(self.querier)
    }
}
