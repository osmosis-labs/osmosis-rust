use osmosis_std_derive::CosmwasmExt;
/// Equivocation implements the Evidence interface and defines evidence of double
/// signing misbehavior.
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
#[proto_message(type_url = "/cosmos.evidence.v1beta1.Equivocation")]
pub struct Equivocation {
    /// height is the equivocation height.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    /// time is the equivocation time.
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    /// power is the equivocation validator power.
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
    /// consensus_address is the equivocation validator consensus address.
    #[prost(string, tag = "4")]
    pub consensus_address: ::prost::alloc::string::String,
}
/// GenesisState defines the evidence module's genesis state.
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
#[proto_message(type_url = "/cosmos.evidence.v1beta1.GenesisState")]
pub struct GenesisState {
    /// evidence defines all the evidence at genesis.
    #[prost(message, repeated, tag = "1")]
    pub evidence: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// QueryEvidenceRequest is the request type for the Query/Evidence RPC method.
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
#[proto_message(type_url = "/cosmos.evidence.v1beta1.QueryEvidenceRequest")]
#[proto_query(
    path = "/cosmos.evidence.v1beta1.Query/Evidence",
    response_type = QueryEvidenceResponse
)]
pub struct QueryEvidenceRequest {
    /// evidence_hash defines the hash of the requested evidence.
    /// Deprecated: Use hash, a HEX encoded string, instead.
    #[deprecated]
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
    /// hash defines the evidence hash of the requested evidence.
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
}
/// QueryEvidenceResponse is the response type for the Query/Evidence RPC method.
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
#[proto_message(type_url = "/cosmos.evidence.v1beta1.QueryEvidenceResponse")]
pub struct QueryEvidenceResponse {
    /// evidence returns the requested evidence.
    #[prost(message, optional, tag = "1")]
    pub evidence: ::core::option::Option<crate::shim::Any>,
}
/// QueryEvidenceRequest is the request type for the Query/AllEvidence RPC
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
#[proto_message(type_url = "/cosmos.evidence.v1beta1.QueryAllEvidenceRequest")]
#[proto_query(
    path = "/cosmos.evidence.v1beta1.Query/AllEvidence",
    response_type = QueryAllEvidenceResponse
)]
pub struct QueryAllEvidenceRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAllEvidenceResponse is the response type for the Query/AllEvidence RPC
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
#[proto_message(type_url = "/cosmos.evidence.v1beta1.QueryAllEvidenceResponse")]
pub struct QueryAllEvidenceResponse {
    /// evidence returns all evidences.
    #[prost(message, repeated, tag = "1")]
    pub evidence: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgSubmitEvidence represents a message that supports submitting arbitrary
/// Evidence of misbehavior such as equivocation or counterfactual signing.
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
#[proto_message(type_url = "/cosmos.evidence.v1beta1.MsgSubmitEvidence")]
pub struct MsgSubmitEvidence {
    /// submitter is the signer account address of evidence.
    #[prost(string, tag = "1")]
    pub submitter: ::prost::alloc::string::String,
    /// evidence defines the evidence of misbehavior.
    #[prost(message, optional, tag = "2")]
    pub evidence: ::core::option::Option<crate::shim::Any>,
}
/// MsgSubmitEvidenceResponse defines the Msg/SubmitEvidence response type.
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
#[proto_message(type_url = "/cosmos.evidence.v1beta1.MsgSubmitEvidenceResponse")]
pub struct MsgSubmitEvidenceResponse {
    /// hash defines the hash of the evidence.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
pub struct EvidenceQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> EvidenceQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn evidence(
        &self,
        evidence_hash: ::prost::alloc::vec::Vec<u8>,
        hash: ::prost::alloc::string::String,
    ) -> Result<QueryEvidenceResponse, cosmwasm_std::StdError> {
        QueryEvidenceRequest {
            evidence_hash,
            hash,
        }
        .query(self.querier)
    }
    pub fn all_evidence(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllEvidenceResponse, cosmwasm_std::StdError> {
        QueryAllEvidenceRequest { pagination }.query(self.querier)
    }
}
