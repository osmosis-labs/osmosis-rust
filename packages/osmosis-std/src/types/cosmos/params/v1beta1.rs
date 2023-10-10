use osmosis_std_derive::CosmwasmExt;
/// ParameterChangeProposal defines a proposal to change one or more parameters.
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
#[proto_message(type_url = "/cosmos.params.v1beta1.ParameterChangeProposal")]
pub struct ParameterChangeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub changes: ::prost::alloc::vec::Vec<ParamChange>,
}
/// ParamChange defines an individual parameter change, for use in
/// ParameterChangeProposal.
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
#[proto_message(type_url = "/cosmos.params.v1beta1.ParamChange")]
pub struct ParamChange {
    #[prost(string, tag = "1")]
    pub subspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/cosmos.params.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.params.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {
    /// subspace defines the module to query the parameter for.
    #[prost(string, tag = "1")]
    pub subspace: ::prost::alloc::string::String,
    /// key defines the key of the parameter in the subspace.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/cosmos.params.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// param defines the queried parameter.
    #[prost(message, optional, tag = "1")]
    pub param: ::core::option::Option<ParamChange>,
}
pub struct ParamsQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ParamsQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(
        &self,
        subspace: ::prost::alloc::string::String,
        key: ::prost::alloc::string::String,
    ) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest { subspace, key }.query(self.querier)
    }
}
