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
#[proto_message(type_url = "/tendermint.privval.RemoteSignerError")]
pub struct RemoteSignerError {
    #[prost(int32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// PubKeyRequest requests the consensus public key from the remote signer.
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
#[proto_message(type_url = "/tendermint.privval.PubKeyRequest")]
pub struct PubKeyRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// PubKeyResponse is a response message containing the public key.
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
#[proto_message(type_url = "/tendermint.privval.PubKeyResponse")]
pub struct PubKeyResponse {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<super::crypto::PublicKey>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// SignVoteRequest is a request to sign a vote
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
#[proto_message(type_url = "/tendermint.privval.SignVoteRequest")]
pub struct SignVoteRequest {
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<super::types::Vote>,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// SignedVoteResponse is a response containing a signed vote or an error
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
#[proto_message(type_url = "/tendermint.privval.SignedVoteResponse")]
pub struct SignedVoteResponse {
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<super::types::Vote>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// SignProposalRequest is a request to sign a proposal
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
#[proto_message(type_url = "/tendermint.privval.SignProposalRequest")]
pub struct SignProposalRequest {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<super::types::Proposal>,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// SignedProposalResponse is response containing a signed proposal or an error
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
#[proto_message(type_url = "/tendermint.privval.SignedProposalResponse")]
pub struct SignedProposalResponse {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<super::types::Proposal>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// PingRequest is a request to confirm that the connection is alive.
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
#[proto_message(type_url = "/tendermint.privval.PingRequest")]
pub struct PingRequest {}
/// PingResponse is a response to confirm that the connection is alive.
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
#[proto_message(type_url = "/tendermint.privval.PingResponse")]
pub struct PingResponse {}
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
#[proto_message(type_url = "/tendermint.privval.Message")]
pub struct Message {
    #[prost(oneof = "message::Sum", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    use osmosis_std_derive::CosmwasmExt;
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Sum {
        #[prost(message, tag = "1")]
        PubKeyRequest(super::PubKeyRequest),
        #[prost(message, tag = "2")]
        PubKeyResponse(super::PubKeyResponse),
        #[prost(message, tag = "3")]
        SignVoteRequest(super::SignVoteRequest),
        #[prost(message, tag = "4")]
        SignedVoteResponse(super::SignedVoteResponse),
        #[prost(message, tag = "5")]
        SignProposalRequest(super::SignProposalRequest),
        #[prost(message, tag = "6")]
        SignedProposalResponse(super::SignedProposalResponse),
        #[prost(message, tag = "7")]
        PingRequest(super::PingRequest),
        #[prost(message, tag = "8")]
        PingResponse(super::PingResponse),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Errors {
    Unknown = 0,
    UnexpectedResponse = 1,
    NoConnection = 2,
    ConnectionTimeout = 3,
    ReadTimeout = 4,
    WriteTimeout = 5,
}
impl Errors {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Errors::Unknown => "ERRORS_UNKNOWN",
            Errors::UnexpectedResponse => "ERRORS_UNEXPECTED_RESPONSE",
            Errors::NoConnection => "ERRORS_NO_CONNECTION",
            Errors::ConnectionTimeout => "ERRORS_CONNECTION_TIMEOUT",
            Errors::ReadTimeout => "ERRORS_READ_TIMEOUT",
            Errors::WriteTimeout => "ERRORS_WRITE_TIMEOUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERRORS_UNKNOWN" => Some(Self::Unknown),
            "ERRORS_UNEXPECTED_RESPONSE" => Some(Self::UnexpectedResponse),
            "ERRORS_NO_CONNECTION" => Some(Self::NoConnection),
            "ERRORS_CONNECTION_TIMEOUT" => Some(Self::ConnectionTimeout),
            "ERRORS_READ_TIMEOUT" => Some(Self::ReadTimeout),
            "ERRORS_WRITE_TIMEOUT" => Some(Self::WriteTimeout),
            _ => None,
        }
    }
}
