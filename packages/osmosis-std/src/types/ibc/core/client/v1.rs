use osmosis_std_derive::CosmwasmExt;
/// IdentifiedClientState defines a client state with an additional client
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
#[proto_message(type_url = "/ibc.core.client.v1.IdentifiedClientState")]
pub struct IdentifiedClientState {
    /// client identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// client state
    #[prost(message, optional, tag = "2")]
    pub client_state: ::core::option::Option<crate::shim::Any>,
}
/// ConsensusStateWithHeight defines a consensus state with an additional height
/// field.
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
#[proto_message(type_url = "/ibc.core.client.v1.ConsensusStateWithHeight")]
pub struct ConsensusStateWithHeight {
    /// consensus state height
    #[prost(message, optional, tag = "1")]
    pub height: ::core::option::Option<Height>,
    /// consensus state
    #[prost(message, optional, tag = "2")]
    pub consensus_state: ::core::option::Option<crate::shim::Any>,
}
/// ClientConsensusStates defines all the stored consensus states for a given
/// client.
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
#[proto_message(type_url = "/ibc.core.client.v1.ClientConsensusStates")]
pub struct ClientConsensusStates {
    /// client identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus states and their heights associated with the client
    #[prost(message, repeated, tag = "2")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
}
/// Height is a monotonically increasing data type
/// that can be compared against another Height for the purposes of updating and
/// freezing clients
///
/// Normally the RevisionHeight is incremented at each height while keeping
/// RevisionNumber the same. However some consensus algorithms may choose to
/// reset the height in certain conditions e.g. hard forks, state-machine
/// breaking changes In these cases, the RevisionNumber is incremented so that
/// height continues to be monitonically increasing even as the RevisionHeight
/// gets reset
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
#[proto_message(type_url = "/ibc.core.client.v1.Height")]
pub struct Height {
    /// the revision that the client is currently on
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision_number: u64,
    /// the height within the given revision
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision_height: u64,
}
/// Params defines the set of IBC light client parameters.
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
#[proto_message(type_url = "/ibc.core.client.v1.Params")]
pub struct Params {
    /// allowed_clients defines the list of allowed client state types which can be created
    /// and interacted with. If a client type is removed from the allowed clients list, usage
    /// of this client will be disabled until it is added again to the list.
    #[prost(string, repeated, tag = "1")]
    pub allowed_clients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ClientUpdateProposal is a legacy governance proposal. If it passes, the substitute
/// client's latest consensus state is copied over to the subject client. The proposal
/// handler may fail if the subject and the substitute do not match in client and
/// chain parameters (with exception to latest height, frozen height, and chain-id).
///
/// Deprecated: Please use MsgRecoverClient in favour of this message type.
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
#[proto_message(type_url = "/ibc.core.client.v1.ClientUpdateProposal")]
#[deprecated]
pub struct ClientUpdateProposal {
    /// the title of the update proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the client identifier for the client to be updated if the proposal passes
    #[prost(string, tag = "3")]
    #[serde(alias = "subject_clientID")]
    pub subject_client_id: ::prost::alloc::string::String,
    /// the substitute client identifier for the client standing in for the subject
    /// client
    #[prost(string, tag = "4")]
    #[serde(alias = "substitute_clientID")]
    pub substitute_client_id: ::prost::alloc::string::String,
}
/// UpgradeProposal is a gov Content type for initiating an IBC breaking
/// upgrade.
///
/// Deprecated: Please use MsgIBCSoftwareUpgrade in favour of this message type.
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
#[proto_message(type_url = "/ibc.core.client.v1.UpgradeProposal")]
#[deprecated]
pub struct UpgradeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub plan: ::core::option::Option<super::super::super::super::cosmos::upgrade::v1beta1::Plan>,
    /// An UpgradedClientState must be provided to perform an IBC breaking upgrade.
    /// This will make the chain commit to the correct upgraded (self) client state
    /// before the upgrade occurs, so that connecting chains can verify that the
    /// new upgraded client is valid by verifying a proof on the previous version
    /// of the chain. This will allow IBC connections to persist smoothly across
    /// planned chain upgrades
    #[prost(message, optional, tag = "4")]
    pub upgraded_client_state: ::core::option::Option<crate::shim::Any>,
}
/// GenesisState defines the ibc client submodule's genesis state.
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
#[proto_message(type_url = "/ibc.core.client.v1.GenesisState")]
pub struct GenesisState {
    /// client states with their corresponding identifiers
    #[prost(message, repeated, tag = "1")]
    pub clients: ::prost::alloc::vec::Vec<IdentifiedClientState>,
    /// consensus states from each client
    #[prost(message, repeated, tag = "2")]
    pub clients_consensus: ::prost::alloc::vec::Vec<ClientConsensusStates>,
    /// metadata from each client
    #[prost(message, repeated, tag = "3")]
    pub clients_metadata: ::prost::alloc::vec::Vec<IdentifiedGenesisMetadata>,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<Params>,
    /// Deprecated: create_localhost has been deprecated.
    /// The localhost client is automatically created at genesis.
    #[deprecated]
    #[prost(bool, tag = "5")]
    pub create_localhost: bool,
    /// the sequence for the next generated client identifier
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_client_sequence: u64,
}
/// GenesisMetadata defines the genesis type for metadata that clients may return
/// with ExportMetadata
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
#[proto_message(type_url = "/ibc.core.client.v1.GenesisMetadata")]
pub struct GenesisMetadata {
    /// store key of metadata without clientID-prefix
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// metadata value
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// IdentifiedGenesisMetadata has the client metadata with the corresponding
/// client id.
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
#[proto_message(type_url = "/ibc.core.client.v1.IdentifiedGenesisMetadata")]
pub struct IdentifiedGenesisMetadata {
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub client_metadata: ::prost::alloc::vec::Vec<GenesisMetadata>,
}
/// QueryClientStateRequest is the request type for the Query/ClientState RPC
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryClientStateRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/ClientState",
    response_type = QueryClientStateResponse
)]
pub struct QueryClientStateRequest {
    /// client state unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryClientStateResponse is the response type for the Query/ClientState RPC
/// method. Besides the client state, it includes a proof and the height from
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryClientStateResponse")]
pub struct QueryClientStateResponse {
    /// client state associated with the request identifier
    #[prost(message, optional, tag = "1")]
    pub client_state: ::core::option::Option<crate::shim::Any>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<Height>,
}
/// QueryClientStatesRequest is the request type for the Query/ClientStates RPC
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryClientStatesRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/ClientStates",
    response_type = QueryClientStatesResponse
)]
pub struct QueryClientStatesRequest {
    /// pagination request
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryClientStatesResponse is the response type for the Query/ClientStates RPC
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryClientStatesResponse")]
pub struct QueryClientStatesResponse {
    /// list of stored ClientStates of the chain.
    #[prost(message, repeated, tag = "1")]
    pub client_states: ::prost::alloc::vec::Vec<IdentifiedClientState>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryConsensusStateRequest is the request type for the Query/ConsensusState
/// RPC method. Besides the consensus state, it includes a proof and the height
/// from which the proof was retrieved.
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryConsensusStateRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/ConsensusState",
    response_type = QueryConsensusStateResponse
)]
pub struct QueryConsensusStateRequest {
    /// client identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus state revision number
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision_number: u64,
    /// consensus state revision height
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision_height: u64,
    /// latest_height overrrides the height field and queries the latest stored
    /// ConsensusState
    #[prost(bool, tag = "4")]
    pub latest_height: bool,
}
/// QueryConsensusStateResponse is the response type for the Query/ConsensusState
/// RPC method
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryConsensusStateResponse")]
pub struct QueryConsensusStateResponse {
    /// consensus state associated with the client identifier at the given height
    #[prost(message, optional, tag = "1")]
    pub consensus_state: ::core::option::Option<crate::shim::Any>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<Height>,
}
/// QueryConsensusStatesRequest is the request type for the Query/ConsensusStates
/// RPC method.
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryConsensusStatesRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/ConsensusStates",
    response_type = QueryConsensusStatesResponse
)]
pub struct QueryConsensusStatesRequest {
    /// client identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryConsensusStatesResponse is the response type for the
/// Query/ConsensusStates RPC method
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryConsensusStatesResponse")]
pub struct QueryConsensusStatesResponse {
    /// consensus states associated with the identifier
    #[prost(message, repeated, tag = "1")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryConsensusStateHeightsRequest is the request type for Query/ConsensusStateHeights
/// RPC method.
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryConsensusStateHeightsRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/ConsensusStateHeights",
    response_type = QueryConsensusStateHeightsResponse
)]
pub struct QueryConsensusStateHeightsRequest {
    /// client identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryConsensusStateHeightsResponse is the response type for the
/// Query/ConsensusStateHeights RPC method
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryConsensusStateHeightsResponse")]
pub struct QueryConsensusStateHeightsResponse {
    /// consensus state heights
    #[prost(message, repeated, tag = "1")]
    pub consensus_state_heights: ::prost::alloc::vec::Vec<Height>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryClientStatusRequest is the request type for the Query/ClientStatus RPC
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryClientStatusRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/ClientStatus",
    response_type = QueryClientStatusResponse
)]
pub struct QueryClientStatusRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryClientStatusResponse is the response type for the Query/ClientStatus RPC
/// method. It returns the current status of the IBC client.
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryClientStatusResponse")]
pub struct QueryClientStatusResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
/// QueryClientParamsRequest is the request type for the Query/ClientParams RPC
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryClientParamsRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/ClientParams",
    response_type = QueryClientParamsResponse
)]
pub struct QueryClientParamsRequest {}
/// QueryClientParamsResponse is the response type for the Query/ClientParams RPC
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryClientParamsResponse")]
pub struct QueryClientParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryUpgradedClientStateRequest is the request type for the
/// Query/UpgradedClientState RPC method
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryUpgradedClientStateRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/UpgradedClientState",
    response_type = QueryUpgradedClientStateResponse
)]
pub struct QueryUpgradedClientStateRequest {}
/// QueryUpgradedClientStateResponse is the response type for the
/// Query/UpgradedClientState RPC method.
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryUpgradedClientStateResponse")]
pub struct QueryUpgradedClientStateResponse {
    /// client state associated with the request identifier
    #[prost(message, optional, tag = "1")]
    pub upgraded_client_state: ::core::option::Option<crate::shim::Any>,
}
/// QueryUpgradedConsensusStateRequest is the request type for the
/// Query/UpgradedConsensusState RPC method
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryUpgradedConsensusStateRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/UpgradedConsensusState",
    response_type = QueryUpgradedConsensusStateResponse
)]
pub struct QueryUpgradedConsensusStateRequest {}
/// QueryUpgradedConsensusStateResponse is the response type for the
/// Query/UpgradedConsensusState RPC method.
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryUpgradedConsensusStateResponse")]
pub struct QueryUpgradedConsensusStateResponse {
    /// Consensus state associated with the request identifier
    #[prost(message, optional, tag = "1")]
    pub upgraded_consensus_state: ::core::option::Option<crate::shim::Any>,
}
/// QueryVerifyMembershipRequest is the request type for the Query/VerifyMembership RPC method
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryVerifyMembershipRequest")]
#[proto_query(
    path = "/ibc.core.client.v1.Query/VerifyMembership",
    response_type = QueryVerifyMembershipResponse
)]
pub struct QueryVerifyMembershipRequest {
    /// client unique identifier.
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// the proof to be verified by the client.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// the height of the commitment root at which the proof is verified.
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<Height>,
    /// the commitment key path.
    #[prost(message, optional, tag = "4")]
    pub merkle_path: ::core::option::Option<super::super::commitment::v1::MerklePath>,
    /// the value which is proven.
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// optional time delay
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub time_delay: u64,
    /// optional block delay
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_delay: u64,
}
/// QueryVerifyMembershipResponse is the response type for the Query/VerifyMembership RPC method
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
#[proto_message(type_url = "/ibc.core.client.v1.QueryVerifyMembershipResponse")]
pub struct QueryVerifyMembershipResponse {
    /// boolean indicating success or failure of proof verification.
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// MsgCreateClient defines a message to create an IBC client
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgCreateClient")]
pub struct MsgCreateClient {
    /// light client state
    #[prost(message, optional, tag = "1")]
    pub client_state: ::core::option::Option<crate::shim::Any>,
    /// consensus state associated with the client that corresponds to a given
    /// height.
    #[prost(message, optional, tag = "2")]
    pub consensus_state: ::core::option::Option<crate::shim::Any>,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgCreateClientResponse defines the Msg/CreateClient response type.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgCreateClientResponse")]
pub struct MsgCreateClientResponse {}
/// MsgUpdateClient defines an sdk.Msg to update a IBC client state using
/// the given client message.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgUpdateClient")]
pub struct MsgUpdateClient {
    /// client unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// client message to update the light client
    #[prost(message, optional, tag = "2")]
    pub client_message: ::core::option::Option<crate::shim::Any>,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgUpdateClientResponse defines the Msg/UpdateClient response type.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgUpdateClientResponse")]
pub struct MsgUpdateClientResponse {}
/// MsgUpgradeClient defines an sdk.Msg to upgrade an IBC client to a new client
/// state
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgUpgradeClient")]
pub struct MsgUpgradeClient {
    /// client unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// upgraded client state
    #[prost(message, optional, tag = "2")]
    pub client_state: ::core::option::Option<crate::shim::Any>,
    /// upgraded consensus state, only contains enough information to serve as a
    /// basis of trust in update logic
    #[prost(message, optional, tag = "3")]
    pub consensus_state: ::core::option::Option<crate::shim::Any>,
    /// proof that old chain committed to new client
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_upgrade_client: ::prost::alloc::vec::Vec<u8>,
    /// proof that old chain committed to new consensus state
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_upgrade_consensus_state: ::prost::alloc::vec::Vec<u8>,
    /// signer address
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgUpgradeClientResponse defines the Msg/UpgradeClient response type.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgUpgradeClientResponse")]
pub struct MsgUpgradeClientResponse {}
/// MsgSubmitMisbehaviour defines an sdk.Msg type that submits Evidence for
/// light client misbehaviour.
/// This message has been deprecated. Use MsgUpdateClient instead.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgSubmitMisbehaviour")]
#[deprecated]
pub struct MsgSubmitMisbehaviour {
    /// client unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// misbehaviour used for freezing the light client
    #[prost(message, optional, tag = "2")]
    pub misbehaviour: ::core::option::Option<crate::shim::Any>,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgSubmitMisbehaviourResponse defines the Msg/SubmitMisbehaviour response
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgSubmitMisbehaviourResponse")]
pub struct MsgSubmitMisbehaviourResponse {}
/// MsgRecoverClient defines the message used to recover a frozen or expired client.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgRecoverClient")]
pub struct MsgRecoverClient {
    /// the client identifier for the client to be updated if the proposal passes
    #[prost(string, tag = "1")]
    #[serde(alias = "subject_clientID")]
    pub subject_client_id: ::prost::alloc::string::String,
    /// the substitute client identifier for the client which will replace the subject
    /// client
    #[prost(string, tag = "2")]
    #[serde(alias = "substitute_clientID")]
    pub substitute_client_id: ::prost::alloc::string::String,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgRecoverClientResponse defines the Msg/RecoverClient response type.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgRecoverClientResponse")]
pub struct MsgRecoverClientResponse {}
/// MsgIBCSoftwareUpgrade defines the message used to schedule an upgrade of an IBC client using a v1 governance proposal
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgIBCSoftwareUpgrade")]
pub struct MsgIbcSoftwareUpgrade {
    #[prost(message, optional, tag = "1")]
    pub plan: ::core::option::Option<super::super::super::super::cosmos::upgrade::v1beta1::Plan>,
    /// An UpgradedClientState must be provided to perform an IBC breaking upgrade.
    /// This will make the chain commit to the correct upgraded (self) client state
    /// before the upgrade occurs, so that connecting chains can verify that the
    /// new upgraded client is valid by verifying a proof on the previous version
    /// of the chain. This will allow IBC connections to persist smoothly across
    /// planned chain upgrades. Correspondingly, the UpgradedClientState field has been
    /// deprecated in the Cosmos SDK to allow for this logic to exist solely in
    /// the 02-client module.
    #[prost(message, optional, tag = "2")]
    pub upgraded_client_state: ::core::option::Option<crate::shim::Any>,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgIBCSoftwareUpgradeResponse defines the Msg/IBCSoftwareUpgrade response type.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgIBCSoftwareUpgradeResponse")]
pub struct MsgIbcSoftwareUpgradeResponse {}
/// MsgUpdateParams defines the sdk.Msg type to update the client parameters.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// signer address
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// params defines the client parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the MsgUpdateParams response type.
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
#[proto_message(type_url = "/ibc.core.client.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct ClientQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ClientQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn client_state(
        &self,
        client_id: ::prost::alloc::string::String,
    ) -> Result<QueryClientStateResponse, cosmwasm_std::StdError> {
        QueryClientStateRequest { client_id }.query(self.querier)
    }
    pub fn client_states(
        &self,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryClientStatesResponse, cosmwasm_std::StdError> {
        QueryClientStatesRequest { pagination }.query(self.querier)
    }
    pub fn consensus_state(
        &self,
        client_id: ::prost::alloc::string::String,
        revision_number: u64,
        revision_height: u64,
        latest_height: bool,
    ) -> Result<QueryConsensusStateResponse, cosmwasm_std::StdError> {
        QueryConsensusStateRequest {
            client_id,
            revision_number,
            revision_height,
            latest_height,
        }
        .query(self.querier)
    }
    pub fn consensus_states(
        &self,
        client_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryConsensusStatesResponse, cosmwasm_std::StdError> {
        QueryConsensusStatesRequest {
            client_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn consensus_state_heights(
        &self,
        client_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryConsensusStateHeightsResponse, cosmwasm_std::StdError> {
        QueryConsensusStateHeightsRequest {
            client_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn client_status(
        &self,
        client_id: ::prost::alloc::string::String,
    ) -> Result<QueryClientStatusResponse, cosmwasm_std::StdError> {
        QueryClientStatusRequest { client_id }.query(self.querier)
    }
    pub fn client_params(&self) -> Result<QueryClientParamsResponse, cosmwasm_std::StdError> {
        QueryClientParamsRequest {}.query(self.querier)
    }
    pub fn upgraded_client_state(
        &self,
    ) -> Result<QueryUpgradedClientStateResponse, cosmwasm_std::StdError> {
        QueryUpgradedClientStateRequest {}.query(self.querier)
    }
    pub fn upgraded_consensus_state(
        &self,
    ) -> Result<QueryUpgradedConsensusStateResponse, cosmwasm_std::StdError> {
        QueryUpgradedConsensusStateRequest {}.query(self.querier)
    }
    pub fn verify_membership(
        &self,
        client_id: ::prost::alloc::string::String,
        proof: ::prost::alloc::vec::Vec<u8>,
        proof_height: ::core::option::Option<Height>,
        merkle_path: ::core::option::Option<super::super::commitment::v1::MerklePath>,
        value: ::prost::alloc::vec::Vec<u8>,
        time_delay: u64,
        block_delay: u64,
    ) -> Result<QueryVerifyMembershipResponse, cosmwasm_std::StdError> {
        QueryVerifyMembershipRequest {
            client_id,
            proof,
            proof_height,
            merkle_path,
            value,
            time_delay,
            block_delay,
        }
        .query(self.querier)
    }
}
