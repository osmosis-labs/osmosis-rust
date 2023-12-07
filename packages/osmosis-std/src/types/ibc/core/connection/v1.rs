use osmosis_std_derive::CosmwasmExt;
/// ConnectionEnd defines a stateful object on a chain connected to another
/// separate one.
/// NOTE: there must only be 2 defined ConnectionEnds to establish
/// a connection between two chains.
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
#[proto_message(type_url = "/ibc.core.connection.v1.ConnectionEnd")]
pub struct ConnectionEnd {
    /// client associated with this connection.
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// IBC version which can be utilised to determine encodings or protocols for
    /// channels or packets utilising this connection.
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// current state of the connection end.
    #[prost(enumeration = "State", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub state: i32,
    /// counterparty chain associated with this connection.
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// delay period that must pass before a consensus state can be used for
    /// packet-verification NOTE: delay period logic is only implemented by some
    /// clients.
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub delay_period: u64,
}
/// IdentifiedConnection defines a connection with additional connection
/// identifier field.
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
#[proto_message(type_url = "/ibc.core.connection.v1.IdentifiedConnection")]
pub struct IdentifiedConnection {
    /// connection identifier.
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// client associated with this connection.
    #[prost(string, tag = "2")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// IBC version which can be utilised to determine encodings or protocols for
    /// channels or packets utilising this connection
    #[prost(message, repeated, tag = "3")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// current state of the connection end.
    #[prost(enumeration = "State", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub state: i32,
    /// counterparty chain associated with this connection.
    #[prost(message, optional, tag = "5")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// delay period associated with this connection.
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub delay_period: u64,
}
/// Counterparty defines the counterparty chain associated with a connection end.
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
#[proto_message(type_url = "/ibc.core.connection.v1.Counterparty")]
pub struct Counterparty {
    /// identifies the client on the counterparty chain associated with a given
    /// connection.
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// identifies the connection end on the counterparty chain associated with a
    /// given connection.
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    /// commitment merkle prefix of the counterparty chain.
    #[prost(message, optional, tag = "3")]
    pub prefix: ::core::option::Option<super::super::commitment::v1::MerklePrefix>,
}
/// ClientPaths define all the connection paths for a client state.
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
#[proto_message(type_url = "/ibc.core.connection.v1.ClientPaths")]
pub struct ClientPaths {
    /// list of connection paths
    #[prost(string, repeated, tag = "1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ConnectionPaths define all the connection paths for a given client state.
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
#[proto_message(type_url = "/ibc.core.connection.v1.ConnectionPaths")]
pub struct ConnectionPaths {
    /// client state unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// list of connection paths
    #[prost(string, repeated, tag = "2")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Version defines the versioning scheme used to negotiate the IBC verison in
/// the connection handshake.
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
#[proto_message(type_url = "/ibc.core.connection.v1.Version")]
pub struct Version {
    /// unique version identifier
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    /// list of features compatible with the specified identifier
    #[prost(string, repeated, tag = "2")]
    pub features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the set of Connection parameters.
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
#[proto_message(type_url = "/ibc.core.connection.v1.Params")]
pub struct Params {
    /// maximum expected time per block (in nanoseconds), used to enforce block delay. This parameter should reflect the
    /// largest amount of time that the chain might reasonably take to produce the next block under normal operating
    /// conditions. A safe choice is 3-5x the expected time per block.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_expected_time_per_block: u64,
}
/// State defines if a connection is in one of the following states:
/// INIT, TRYOPEN, OPEN or UNINITIALIZED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum State {
    /// Default State
    UninitializedUnspecified = 0,
    /// A connection end has just started the opening handshake.
    Init = 1,
    /// A connection end has acknowledged the handshake step on the counterparty
    /// chain.
    Tryopen = 2,
    /// A connection end has completed the handshake.
    Open = 3,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::UninitializedUnspecified => "STATE_UNINITIALIZED_UNSPECIFIED",
            State::Init => "STATE_INIT",
            State::Tryopen => "STATE_TRYOPEN",
            State::Open => "STATE_OPEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNINITIALIZED_UNSPECIFIED" => Some(Self::UninitializedUnspecified),
            "STATE_INIT" => Some(Self::Init),
            "STATE_TRYOPEN" => Some(Self::Tryopen),
            "STATE_OPEN" => Some(Self::Open),
            _ => None,
        }
    }
}
/// GenesisState defines the ibc connection submodule's genesis state.
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
#[proto_message(type_url = "/ibc.core.connection.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub connections: ::prost::alloc::vec::Vec<IdentifiedConnection>,
    #[prost(message, repeated, tag = "2")]
    pub client_connection_paths: ::prost::alloc::vec::Vec<ConnectionPaths>,
    /// the sequence for the next generated connection identifier
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_connection_sequence: u64,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<Params>,
}
/// QueryConnectionRequest is the request type for the Query/Connection RPC
/// method
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionRequest")]
#[proto_query(
    path = "/ibc.core.connection.v1.Query/Connection",
    response_type = QueryConnectionResponse
)]
pub struct QueryConnectionRequest {
    /// connection unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
}
/// QueryConnectionResponse is the response type for the Query/Connection RPC
/// method. Besides the connection end, it includes a proof and the height from
/// which the proof was retrieved.
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionResponse")]
pub struct QueryConnectionResponse {
    /// connection associated with the request identifier
    #[prost(message, optional, tag = "1")]
    pub connection: ::core::option::Option<ConnectionEnd>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionsRequest is the request type for the Query/Connections RPC
/// method
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionsRequest")]
#[proto_query(
    path = "/ibc.core.connection.v1.Query/Connections",
    response_type = QueryConnectionsResponse
)]
pub struct QueryConnectionsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryConnectionsResponse is the response type for the Query/Connections RPC
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionsResponse")]
pub struct QueryConnectionsResponse {
    /// list of stored connections of the chain.
    #[prost(message, repeated, tag = "1")]
    pub connections: ::prost::alloc::vec::Vec<IdentifiedConnection>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryClientConnectionsRequest is the request type for the
/// Query/ClientConnections RPC method
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryClientConnectionsRequest")]
#[proto_query(
    path = "/ibc.core.connection.v1.Query/ClientConnections",
    response_type = QueryClientConnectionsResponse
)]
pub struct QueryClientConnectionsRequest {
    /// client identifier associated with a connection
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryClientConnectionsResponse is the response type for the
/// Query/ClientConnections RPC method
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryClientConnectionsResponse")]
pub struct QueryClientConnectionsResponse {
    /// slice of all the connection paths associated with a client.
    #[prost(string, repeated, tag = "1")]
    pub connection_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was generated
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionClientStateRequest is the request type for the
/// Query/ConnectionClientState RPC method
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionClientStateRequest")]
#[proto_query(
    path = "/ibc.core.connection.v1.Query/ConnectionClientState",
    response_type = QueryConnectionClientStateResponse
)]
pub struct QueryConnectionClientStateRequest {
    /// connection identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
}
/// QueryConnectionClientStateResponse is the response type for the
/// Query/ConnectionClientState RPC method
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionClientStateResponse")]
pub struct QueryConnectionClientStateResponse {
    /// client state associated with the channel
    #[prost(message, optional, tag = "1")]
    pub identified_client_state:
        ::core::option::Option<super::super::client::v1::IdentifiedClientState>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionConsensusStateRequest is the request type for the
/// Query/ConnectionConsensusState RPC method
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionConsensusStateRequest")]
#[proto_query(
    path = "/ibc.core.connection.v1.Query/ConnectionConsensusState",
    response_type = QueryConnectionConsensusStateResponse
)]
pub struct QueryConnectionConsensusStateRequest {
    /// connection identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision_number: u64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision_height: u64,
}
/// QueryConnectionConsensusStateResponse is the response type for the
/// Query/ConnectionConsensusState RPC method
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionConsensusStateResponse")]
pub struct QueryConnectionConsensusStateResponse {
    /// consensus state associated with the channel
    #[prost(message, optional, tag = "1")]
    pub consensus_state: ::core::option::Option<crate::shim::Any>,
    /// client ID associated with the consensus state
    #[prost(string, tag = "2")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionParamsRequest is the request type for the Query/ConnectionParams RPC method.
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionParamsRequest")]
#[proto_query(
    path = "/ibc.core.connection.v1.Query/ConnectionParams",
    response_type = QueryConnectionParamsResponse
)]
pub struct QueryConnectionParamsRequest {}
/// QueryConnectionParamsResponse is the response type for the Query/ConnectionParams RPC method.
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
#[proto_message(type_url = "/ibc.core.connection.v1.QueryConnectionParamsResponse")]
pub struct QueryConnectionParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgConnectionOpenInit defines the msg sent by an account on Chain A to
/// initialize a connection with Chain B.
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
#[proto_message(type_url = "/ibc.core.connection.v1.MsgConnectionOpenInit")]
pub struct MsgConnectionOpenInit {
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub counterparty: ::core::option::Option<Counterparty>,
    #[prost(message, optional, tag = "3")]
    pub version: ::core::option::Option<Version>,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub delay_period: u64,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgConnectionOpenInitResponse defines the Msg/ConnectionOpenInit response
/// type.
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
#[proto_message(type_url = "/ibc.core.connection.v1.MsgConnectionOpenInitResponse")]
pub struct MsgConnectionOpenInitResponse {}
/// MsgConnectionOpenTry defines a msg sent by a Relayer to try to open a
/// connection on Chain B.
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
#[proto_message(type_url = "/ibc.core.connection.v1.MsgConnectionOpenTry")]
pub struct MsgConnectionOpenTry {
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// Deprecated: this field is unused. Crossing hellos are no longer supported in core IBC.
    #[deprecated]
    #[prost(string, tag = "2")]
    #[serde(alias = "previous_connectionID")]
    pub previous_connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub client_state: ::core::option::Option<crate::shim::Any>,
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<Counterparty>,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub delay_period: u64,
    #[prost(message, repeated, tag = "6")]
    pub counterparty_versions: ::prost::alloc::vec::Vec<Version>,
    #[prost(message, optional, tag = "7")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    /// proof of the initialization the connection on Chain A: `UNITIALIZED ->
    /// INIT`
    #[prost(bytes = "vec", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_init: ::prost::alloc::vec::Vec<u8>,
    /// proof of client state included in message
    #[prost(bytes = "vec", tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_client: ::prost::alloc::vec::Vec<u8>,
    /// proof of client consensus state
    #[prost(bytes = "vec", tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_consensus: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "11")]
    pub consensus_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "12")]
    pub signer: ::prost::alloc::string::String,
    /// optional proof data for host state machines that are unable to introspect their own consensus state
    #[prost(bytes = "vec", tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub host_consensus_state_proof: ::prost::alloc::vec::Vec<u8>,
}
/// MsgConnectionOpenTryResponse defines the Msg/ConnectionOpenTry response type.
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
#[proto_message(type_url = "/ibc.core.connection.v1.MsgConnectionOpenTryResponse")]
pub struct MsgConnectionOpenTryResponse {}
/// MsgConnectionOpenAck defines a msg sent by a Relayer to Chain A to
/// acknowledge the change of connection state to TRYOPEN on Chain B.
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
#[proto_message(type_url = "/ibc.core.connection.v1.MsgConnectionOpenAck")]
pub struct MsgConnectionOpenAck {
    #[prost(string, tag = "1")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "counterparty_connectionID")]
    pub counterparty_connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub version: ::core::option::Option<Version>,
    #[prost(message, optional, tag = "4")]
    pub client_state: ::core::option::Option<crate::shim::Any>,
    #[prost(message, optional, tag = "5")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    /// proof of the initialization the connection on Chain B: `UNITIALIZED ->
    /// TRYOPEN`
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_try: ::prost::alloc::vec::Vec<u8>,
    /// proof of client state included in message
    #[prost(bytes = "vec", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_client: ::prost::alloc::vec::Vec<u8>,
    /// proof of client consensus state
    #[prost(bytes = "vec", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_consensus: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "9")]
    pub consensus_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "10")]
    pub signer: ::prost::alloc::string::String,
    /// optional proof data for host state machines that are unable to introspect their own consensus state
    #[prost(bytes = "vec", tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub host_consensus_state_proof: ::prost::alloc::vec::Vec<u8>,
}
/// MsgConnectionOpenAckResponse defines the Msg/ConnectionOpenAck response type.
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
#[proto_message(type_url = "/ibc.core.connection.v1.MsgConnectionOpenAckResponse")]
pub struct MsgConnectionOpenAckResponse {}
/// MsgConnectionOpenConfirm defines a msg sent by a Relayer to Chain B to
/// acknowledge the change of connection state to OPEN on Chain A.
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
#[proto_message(type_url = "/ibc.core.connection.v1.MsgConnectionOpenConfirm")]
pub struct MsgConnectionOpenConfirm {
    #[prost(string, tag = "1")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    /// proof for the change of the connection state on Chain A: `INIT -> OPEN`
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_ack: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgConnectionOpenConfirmResponse defines the Msg/ConnectionOpenConfirm
/// response type.
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
#[proto_message(type_url = "/ibc.core.connection.v1.MsgConnectionOpenConfirmResponse")]
pub struct MsgConnectionOpenConfirmResponse {}
pub struct ConnectionQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ConnectionQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn connection(
        &self,
        connection_id: ::prost::alloc::string::String,
    ) -> Result<QueryConnectionResponse, cosmwasm_std::StdError> {
        QueryConnectionRequest { connection_id }.query(self.querier)
    }
    pub fn connections(
        &self,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryConnectionsResponse, cosmwasm_std::StdError> {
        QueryConnectionsRequest { pagination }.query(self.querier)
    }
    pub fn client_connections(
        &self,
        client_id: ::prost::alloc::string::String,
    ) -> Result<QueryClientConnectionsResponse, cosmwasm_std::StdError> {
        QueryClientConnectionsRequest { client_id }.query(self.querier)
    }
    pub fn connection_client_state(
        &self,
        connection_id: ::prost::alloc::string::String,
    ) -> Result<QueryConnectionClientStateResponse, cosmwasm_std::StdError> {
        QueryConnectionClientStateRequest { connection_id }.query(self.querier)
    }
    pub fn connection_consensus_state(
        &self,
        connection_id: ::prost::alloc::string::String,
        revision_number: u64,
        revision_height: u64,
    ) -> Result<QueryConnectionConsensusStateResponse, cosmwasm_std::StdError> {
        QueryConnectionConsensusStateRequest {
            connection_id,
            revision_number,
            revision_height,
        }
        .query(self.querier)
    }
    pub fn connection_params(
        &self,
    ) -> Result<QueryConnectionParamsResponse, cosmwasm_std::StdError> {
        QueryConnectionParamsRequest {}.query(self.querier)
    }
}
