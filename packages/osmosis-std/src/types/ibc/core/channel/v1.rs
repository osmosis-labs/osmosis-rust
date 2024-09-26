use osmosis_std_derive::CosmwasmExt;
/// Channel defines pipeline for exactly-once packet delivery between specific
/// modules on separate blockchains, which has at least one end capable of
/// sending packets and one end capable of receiving packets.
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
#[proto_message(type_url = "/ibc.core.channel.v1.Channel")]
pub struct Channel {
    /// current state of the channel end
    #[prost(enumeration = "State", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub state: i32,
    /// whether the channel is ordered or unordered
    #[prost(enumeration = "Order", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ordering: i32,
    /// counterparty channel end
    #[prost(message, optional, tag = "3")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// list of connection identifiers, in order, along which packets sent on
    /// this channel will travel
    #[prost(string, repeated, tag = "4")]
    pub connection_hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// opaque channel version, which is agreed upon during the handshake
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// upgrade sequence indicates the latest upgrade attempt performed by this channel
    /// the value of 0 indicates the channel has never been upgraded
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upgrade_sequence: u64,
}
/// IdentifiedChannel defines a channel with additional port and channel
/// identifier fields.
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
#[proto_message(type_url = "/ibc.core.channel.v1.IdentifiedChannel")]
pub struct IdentifiedChannel {
    /// current state of the channel end
    #[prost(enumeration = "State", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub state: i32,
    /// whether the channel is ordered or unordered
    #[prost(enumeration = "Order", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ordering: i32,
    /// counterparty channel end
    #[prost(message, optional, tag = "3")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// list of connection identifiers, in order, along which packets sent on
    /// this channel will travel
    #[prost(string, repeated, tag = "4")]
    pub connection_hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// opaque channel version, which is agreed upon during the handshake
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// port identifier
    #[prost(string, tag = "6")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel identifier
    #[prost(string, tag = "7")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// upgrade sequence indicates the latest upgrade attempt performed by this channel
    /// the value of 0 indicates the channel has never been upgraded
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upgrade_sequence: u64,
}
/// Counterparty defines a channel end counterparty
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
#[proto_message(type_url = "/ibc.core.channel.v1.Counterparty")]
pub struct Counterparty {
    /// port on the counterparty chain which owns the other end of the channel.
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel end on the counterparty chain
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// Packet defines a type that carries data across different chains through IBC
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
#[proto_message(type_url = "/ibc.core.channel.v1.Packet")]
pub struct Packet {
    /// number corresponds to the order of sends and receives, where a Packet
    /// with an earlier sequence number must be sent and received before a Packet
    /// with a later sequence number.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
    /// identifies the port on the sending chain.
    #[prost(string, tag = "2")]
    pub source_port: ::prost::alloc::string::String,
    /// identifies the channel end on the sending chain.
    #[prost(string, tag = "3")]
    pub source_channel: ::prost::alloc::string::String,
    /// identifies the port on the receiving chain.
    #[prost(string, tag = "4")]
    pub destination_port: ::prost::alloc::string::String,
    /// identifies the channel end on the receiving chain.
    #[prost(string, tag = "5")]
    pub destination_channel: ::prost::alloc::string::String,
    /// actual opaque bytes transferred directly to the application module
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// block height after which the packet times out
    #[prost(message, optional, tag = "7")]
    pub timeout_height: ::core::option::Option<super::super::client::v1::Height>,
    /// block timestamp (in nanoseconds) after which the packet times out
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timeout_timestamp: u64,
}
/// PacketState defines the generic type necessary to retrieve and store
/// packet commitments, acknowledgements, and receipts.
/// Caller is responsible for knowing the context necessary to interpret this
/// state as a commitment, acknowledgement, or a receipt.
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
#[proto_message(type_url = "/ibc.core.channel.v1.PacketState")]
pub struct PacketState {
    /// channel port identifier.
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier.
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
    /// embedded data that represents packet state.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// PacketId is an identifer for a unique Packet
/// Source chains refer to packets by source port/channel
/// Destination chains refer to packets by destination port/channel
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
#[proto_message(type_url = "/ibc.core.channel.v1.PacketId")]
pub struct PacketId {
    /// channel port identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
/// Acknowledgement is the recommended acknowledgement format to be used by
/// app-specific protocols.
/// NOTE: The field numbers 21 and 22 were explicitly chosen to avoid accidental
/// conflicts with other protobuf message formats used for acknowledgements.
/// The first byte of any message with this format will be the non-ASCII values
/// `0xaa` (result) or `0xb2` (error). Implemented as defined by ICS:
/// <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#acknowledgement-envelope>
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
#[proto_message(type_url = "/ibc.core.channel.v1.Acknowledgement")]
pub struct Acknowledgement {
    /// response contains either a result or an error and must be non-empty
    #[prost(oneof = "acknowledgement::Response", tags = "21, 22")]
    pub response: ::core::option::Option<acknowledgement::Response>,
}
/// Nested message and enum types in `Acknowledgement`.
pub mod acknowledgement {
    use osmosis_std_derive::CosmwasmExt;
    /// response contains either a result or an error and must be non-empty
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
    pub enum Response {
        #[prost(bytes, tag = "21")]
        Result(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "22")]
        Error(::prost::alloc::string::String),
    }
}
/// Timeout defines an execution deadline structure for 04-channel handlers.
/// This includes packet lifecycle handlers as well as the upgrade handshake handlers.
/// A valid Timeout contains either one or both of a timestamp and block height (sequence).
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
#[proto_message(type_url = "/ibc.core.channel.v1.Timeout")]
pub struct Timeout {
    /// block height after which the packet or upgrade times out
    #[prost(message, optional, tag = "1")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
    /// block timestamp (in nanoseconds) after which the packet or upgrade times out
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: u64,
}
/// Params defines the set of IBC channel parameters.
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
#[proto_message(type_url = "/ibc.core.channel.v1.Params")]
pub struct Params {
    /// the relative timeout after which channel upgrades will time out.
    #[prost(message, optional, tag = "1")]
    pub upgrade_timeout: ::core::option::Option<Timeout>,
}
/// State defines if a channel is in one of the following states:
/// CLOSED, INIT, TRYOPEN, OPEN, FLUSHING, FLUSHCOMPLETE or UNINITIALIZED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum State {
    /// Default State
    UninitializedUnspecified = 0,
    /// A channel has just started the opening handshake.
    Init = 1,
    /// A channel has acknowledged the handshake step on the counterparty chain.
    Tryopen = 2,
    /// A channel has completed the handshake. Open channels are
    /// ready to send and receive packets.
    Open = 3,
    /// A channel has been closed and can no longer be used to send or receive
    /// packets.
    Closed = 4,
    /// A channel has just accepted the upgrade handshake attempt and is flushing in-flight packets.
    Flushing = 5,
    /// A channel has just completed flushing any in-flight packets.
    Flushcomplete = 6,
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
            State::Closed => "STATE_CLOSED",
            State::Flushing => "STATE_FLUSHING",
            State::Flushcomplete => "STATE_FLUSHCOMPLETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNINITIALIZED_UNSPECIFIED" => Some(Self::UninitializedUnspecified),
            "STATE_INIT" => Some(Self::Init),
            "STATE_TRYOPEN" => Some(Self::Tryopen),
            "STATE_OPEN" => Some(Self::Open),
            "STATE_CLOSED" => Some(Self::Closed),
            "STATE_FLUSHING" => Some(Self::Flushing),
            "STATE_FLUSHCOMPLETE" => Some(Self::Flushcomplete),
            _ => None,
        }
    }
}
/// Order defines if a channel is ORDERED or UNORDERED
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Order {
    /// zero-value for channel ordering
    NoneUnspecified = 0,
    /// packets can be delivered in any order, which may differ from the order in
    /// which they were sent.
    Unordered = 1,
    /// packets are delivered exactly in the order which they were sent
    Ordered = 2,
}
impl Order {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Order::NoneUnspecified => "ORDER_NONE_UNSPECIFIED",
            Order::Unordered => "ORDER_UNORDERED",
            Order::Ordered => "ORDER_ORDERED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_NONE_UNSPECIFIED" => Some(Self::NoneUnspecified),
            "ORDER_UNORDERED" => Some(Self::Unordered),
            "ORDER_ORDERED" => Some(Self::Ordered),
            _ => None,
        }
    }
}
/// GenesisState defines the ibc channel submodule's genesis state.
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
#[proto_message(type_url = "/ibc.core.channel.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<IdentifiedChannel>,
    #[prost(message, repeated, tag = "2")]
    pub acknowledgements: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "3")]
    pub commitments: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "4")]
    pub receipts: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "5")]
    pub send_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
    #[prost(message, repeated, tag = "6")]
    pub recv_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
    #[prost(message, repeated, tag = "7")]
    pub ack_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
    /// the sequence for the next generated channel identifier
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_channel_sequence: u64,
    #[prost(message, optional, tag = "9")]
    pub params: ::core::option::Option<Params>,
}
/// PacketSequence defines the genesis type necessary to retrieve and store
/// next send and receive sequences.
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
#[proto_message(type_url = "/ibc.core.channel.v1.PacketSequence")]
pub struct PacketSequence {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
/// Upgrade is a verifiable type which contains the relevant information
/// for an attempted upgrade. It provides the proposed changes to the channel
/// end, the timeout for this upgrade attempt and the next packet sequence
/// which allows the counterparty to efficiently know the highest sequence it has received.
/// The next sequence send is used for pruning and upgrading from unordered to ordered channels.
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
#[proto_message(type_url = "/ibc.core.channel.v1.Upgrade")]
pub struct Upgrade {
    #[prost(message, optional, tag = "1")]
    pub fields: ::core::option::Option<UpgradeFields>,
    #[prost(message, optional, tag = "2")]
    pub timeout: ::core::option::Option<Timeout>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_sequence_send: u64,
}
/// UpgradeFields are the fields in a channel end which may be changed
/// during a channel upgrade.
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
#[proto_message(type_url = "/ibc.core.channel.v1.UpgradeFields")]
pub struct UpgradeFields {
    #[prost(enumeration = "Order", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ordering: i32,
    #[prost(string, repeated, tag = "2")]
    pub connection_hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
/// ErrorReceipt defines a type which encapsulates the upgrade sequence and error associated with the
/// upgrade handshake failure. When a channel upgrade handshake is aborted both chains are expected to increment to the
/// next sequence.
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
#[proto_message(type_url = "/ibc.core.channel.v1.ErrorReceipt")]
pub struct ErrorReceipt {
    /// the channel upgrade sequence
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
    /// the error message detailing the cause of failure
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// QueryChannelRequest is the request type for the Query/Channel RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/Channel",
    response_type = QueryChannelResponse
)]
pub struct QueryChannelRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryChannelResponse is the response type for the Query/Channel RPC method.
/// Besides the Channel end, it includes a proof and the height from which the
/// proof was retrieved.
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelResponse")]
pub struct QueryChannelResponse {
    /// channel associated with the request identifiers
    #[prost(message, optional, tag = "1")]
    pub channel: ::core::option::Option<Channel>,
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
/// QueryChannelsRequest is the request type for the Query/Channels RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelsRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/Channels",
    response_type = QueryChannelsResponse
)]
pub struct QueryChannelsRequest {
    /// pagination request
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryChannelsResponse is the response type for the Query/Channels RPC method.
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelsResponse")]
pub struct QueryChannelsResponse {
    /// list of stored channels of the chain.
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<IdentifiedChannel>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionChannelsRequest is the request type for the
/// Query/QueryConnectionChannels RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryConnectionChannelsRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/ConnectionChannels",
    response_type = QueryConnectionChannelsResponse
)]
pub struct QueryConnectionChannelsRequest {
    /// connection unique identifier
    #[prost(string, tag = "1")]
    pub connection: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryConnectionChannelsResponse is the Response type for the
/// Query/QueryConnectionChannels RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryConnectionChannelsResponse")]
pub struct QueryConnectionChannelsResponse {
    /// list of channels associated with a connection.
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<IdentifiedChannel>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryChannelClientStateRequest is the request type for the Query/ClientState
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelClientStateRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/ChannelClientState",
    response_type = QueryChannelClientStateResponse
)]
pub struct QueryChannelClientStateRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryChannelClientStateResponse is the Response type for the
/// Query/QueryChannelClientState RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelClientStateResponse")]
pub struct QueryChannelClientStateResponse {
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
/// QueryChannelConsensusStateRequest is the request type for the
/// Query/ConsensusState RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelConsensusStateRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/ChannelConsensusState",
    response_type = QueryChannelConsensusStateResponse
)]
pub struct QueryChannelConsensusStateRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// revision number of the consensus state
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision_number: u64,
    /// revision height of the consensus state
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision_height: u64,
}
/// QueryChannelClientStateResponse is the Response type for the
/// Query/QueryChannelClientState RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelConsensusStateResponse")]
pub struct QueryChannelConsensusStateResponse {
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
/// QueryPacketCommitmentRequest is the request type for the
/// Query/PacketCommitment RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketCommitmentRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/PacketCommitment",
    response_type = QueryPacketCommitmentResponse
)]
pub struct QueryPacketCommitmentRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
/// QueryPacketCommitmentResponse defines the client query response for a packet
/// which also includes a proof and the height from which the proof was
/// retrieved
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketCommitmentResponse")]
pub struct QueryPacketCommitmentResponse {
    /// packet associated with the request fields
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub commitment: ::prost::alloc::vec::Vec<u8>,
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
/// QueryPacketCommitmentsRequest is the request type for the
/// Query/QueryPacketCommitments RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketCommitmentsRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/PacketCommitments",
    response_type = QueryPacketCommitmentsResponse
)]
pub struct QueryPacketCommitmentsRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryPacketCommitmentsResponse is the request type for the
/// Query/QueryPacketCommitments RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketCommitmentsResponse")]
pub struct QueryPacketCommitmentsResponse {
    #[prost(message, repeated, tag = "1")]
    pub commitments: ::prost::alloc::vec::Vec<PacketState>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketReceiptRequest is the request type for the
/// Query/PacketReceipt RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketReceiptRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/PacketReceipt",
    response_type = QueryPacketReceiptResponse
)]
pub struct QueryPacketReceiptRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
/// QueryPacketReceiptResponse defines the client query response for a packet
/// receipt which also includes a proof, and the height from which the proof was
/// retrieved
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketReceiptResponse")]
pub struct QueryPacketReceiptResponse {
    /// success flag for if receipt exists
    #[prost(bool, tag = "2")]
    pub received: bool,
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
/// QueryPacketAcknowledgementRequest is the request type for the
/// Query/PacketAcknowledgement RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketAcknowledgementRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/PacketAcknowledgement",
    response_type = QueryPacketAcknowledgementResponse
)]
pub struct QueryPacketAcknowledgementRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
/// QueryPacketAcknowledgementResponse defines the client query response for a
/// packet which also includes a proof and the height from which the
/// proof was retrieved
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketAcknowledgementResponse")]
pub struct QueryPacketAcknowledgementResponse {
    /// packet associated with the request fields
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub acknowledgement: ::prost::alloc::vec::Vec<u8>,
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
/// QueryPacketAcknowledgementsRequest is the request type for the
/// Query/QueryPacketCommitments RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketAcknowledgementsRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/PacketAcknowledgements",
    response_type = QueryPacketAcknowledgementsResponse
)]
pub struct QueryPacketAcknowledgementsRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    /// list of packet sequences
    #[prost(uint64, repeated, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
}
/// QueryPacketAcknowledgemetsResponse is the request type for the
/// Query/QueryPacketAcknowledgements RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryPacketAcknowledgementsResponse")]
pub struct QueryPacketAcknowledgementsResponse {
    #[prost(message, repeated, tag = "1")]
    pub acknowledgements: ::prost::alloc::vec::Vec<PacketState>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryUnreceivedPacketsRequest is the request type for the
/// Query/UnreceivedPackets RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryUnreceivedPacketsRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/UnreceivedPackets",
    response_type = QueryUnreceivedPacketsResponse
)]
pub struct QueryUnreceivedPacketsRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// list of packet sequences
    #[prost(uint64, repeated, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
}
/// QueryUnreceivedPacketsResponse is the response type for the
/// Query/UnreceivedPacketCommitments RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryUnreceivedPacketsResponse")]
pub struct QueryUnreceivedPacketsResponse {
    /// list of unreceived packet sequences
    #[prost(uint64, repeated, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub sequences: ::prost::alloc::vec::Vec<u64>,
    /// query block height
    #[prost(message, optional, tag = "2")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryUnreceivedAcks is the request type for the
/// Query/UnreceivedAcks RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryUnreceivedAcksRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/UnreceivedAcks",
    response_type = QueryUnreceivedAcksResponse
)]
pub struct QueryUnreceivedAcksRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// list of acknowledgement sequences
    #[prost(uint64, repeated, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub packet_ack_sequences: ::prost::alloc::vec::Vec<u64>,
}
/// QueryUnreceivedAcksResponse is the response type for the
/// Query/UnreceivedAcks RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryUnreceivedAcksResponse")]
pub struct QueryUnreceivedAcksResponse {
    /// list of unreceived acknowledgement sequences
    #[prost(uint64, repeated, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub sequences: ::prost::alloc::vec::Vec<u64>,
    /// query block height
    #[prost(message, optional, tag = "2")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryNextSequenceReceiveRequest is the request type for the
/// Query/QueryNextSequenceReceiveRequest RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryNextSequenceReceiveRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/NextSequenceReceive",
    response_type = QueryNextSequenceReceiveResponse
)]
pub struct QueryNextSequenceReceiveRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QuerySequenceResponse is the response type for the
/// Query/QueryNextSequenceReceiveResponse RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryNextSequenceReceiveResponse")]
pub struct QueryNextSequenceReceiveResponse {
    /// next sequence receive number
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_sequence_receive: u64,
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
/// QueryNextSequenceSendRequest is the request type for the
/// Query/QueryNextSequenceSend RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryNextSequenceSendRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/NextSequenceSend",
    response_type = QueryNextSequenceSendResponse
)]
pub struct QueryNextSequenceSendRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryNextSequenceSendResponse is the request type for the
/// Query/QueryNextSequenceSend RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryNextSequenceSendResponse")]
pub struct QueryNextSequenceSendResponse {
    /// next sequence send number
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_sequence_send: u64,
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
/// QueryUpgradeErrorRequest is the request type for the Query/QueryUpgradeError RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryUpgradeErrorRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/UpgradeError",
    response_type = QueryUpgradeErrorResponse
)]
pub struct QueryUpgradeErrorRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryUpgradeErrorResponse is the response type for the Query/QueryUpgradeError RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryUpgradeErrorResponse")]
pub struct QueryUpgradeErrorResponse {
    #[prost(message, optional, tag = "1")]
    pub error_receipt: ::core::option::Option<ErrorReceipt>,
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
/// QueryUpgradeRequest is the request type for the QueryUpgradeRequest RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryUpgradeRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/Upgrade",
    response_type = QueryUpgradeResponse
)]
pub struct QueryUpgradeRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryUpgradeResponse is the response type for the QueryUpgradeResponse RPC method
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryUpgradeResponse")]
pub struct QueryUpgradeResponse {
    #[prost(message, optional, tag = "1")]
    pub upgrade: ::core::option::Option<Upgrade>,
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
/// QueryChannelParamsRequest is the request type for the Query/ChannelParams RPC method.
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelParamsRequest")]
#[proto_query(
    path = "/ibc.core.channel.v1.Query/ChannelParams",
    response_type = QueryChannelParamsResponse
)]
pub struct QueryChannelParamsRequest {}
/// QueryChannelParamsResponse is the response type for the Query/ChannelParams RPC method.
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
#[proto_message(type_url = "/ibc.core.channel.v1.QueryChannelParamsResponse")]
pub struct QueryChannelParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgChannelOpenInit defines an sdk.Msg to initialize a channel handshake. It
/// is called by a relayer on Chain A.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelOpenInit")]
pub struct MsgChannelOpenInit {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub channel: ::core::option::Option<Channel>,
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelOpenInitResponse defines the Msg/ChannelOpenInit response type.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelOpenInitResponse")]
pub struct MsgChannelOpenInitResponse {
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// MsgChannelOpenInit defines a msg sent by a Relayer to try to open a channel
/// on Chain B. The version field within the Channel field has been deprecated. Its
/// value will be ignored by core IBC.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelOpenTry")]
pub struct MsgChannelOpenTry {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// Deprecated: this field is unused. Crossing hello's are no longer supported in core IBC.
    #[deprecated]
    #[prost(string, tag = "2")]
    #[serde(alias = "previous_channelID")]
    pub previous_channel_id: ::prost::alloc::string::String,
    /// NOTE: the version field within the channel has been deprecated. Its value will be ignored by core IBC.
    #[prost(message, optional, tag = "3")]
    pub channel: ::core::option::Option<Channel>,
    #[prost(string, tag = "4")]
    pub counterparty_version: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_init: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "7")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelOpenTryResponse defines the Msg/ChannelOpenTry response type.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelOpenTryResponse")]
pub struct MsgChannelOpenTryResponse {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// MsgChannelOpenAck defines a msg sent by a Relayer to Chain A to acknowledge
/// the change of channel state to TRYOPEN on Chain B.
/// WARNING: a channel upgrade MUST NOT initialize an upgrade for this channel
/// in the same block as executing this message otherwise the counterparty will
/// be incapable of opening.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelOpenAck")]
pub struct MsgChannelOpenAck {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "counterparty_channelID")]
    pub counterparty_channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub counterparty_version: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_try: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "7")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelOpenAckResponse defines the Msg/ChannelOpenAck response type.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelOpenAckResponse")]
pub struct MsgChannelOpenAckResponse {}
/// MsgChannelOpenConfirm defines a msg sent by a Relayer to Chain B to
/// acknowledge the change of channel state to OPEN on Chain A.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelOpenConfirm")]
pub struct MsgChannelOpenConfirm {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_ack: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelOpenConfirmResponse defines the Msg/ChannelOpenConfirm response
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelOpenConfirmResponse")]
pub struct MsgChannelOpenConfirmResponse {}
/// MsgChannelCloseInit defines a msg sent by a Relayer to Chain A
/// to close a channel with Chain B.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelCloseInit")]
pub struct MsgChannelCloseInit {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelCloseInitResponse defines the Msg/ChannelCloseInit response type.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelCloseInitResponse")]
pub struct MsgChannelCloseInitResponse {}
/// MsgChannelCloseConfirm defines a msg sent by a Relayer to Chain B
/// to acknowledge the change of channel state to CLOSED on Chain A.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelCloseConfirm")]
pub struct MsgChannelCloseConfirm {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_init: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub counterparty_upgrade_sequence: u64,
}
/// MsgChannelCloseConfirmResponse defines the Msg/ChannelCloseConfirm response
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelCloseConfirmResponse")]
pub struct MsgChannelCloseConfirmResponse {}
/// MsgRecvPacket receives incoming IBC packet
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgRecvPacket")]
pub struct MsgRecvPacket {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_commitment: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgRecvPacketResponse defines the Msg/RecvPacket response type.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgRecvPacketResponse")]
pub struct MsgRecvPacketResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
}
/// MsgTimeout receives timed-out packet
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgTimeout")]
pub struct MsgTimeout {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_unreceived: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_sequence_recv: u64,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgTimeoutResponse defines the Msg/Timeout response type.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgTimeoutResponse")]
pub struct MsgTimeoutResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
}
/// MsgTimeoutOnClose timed-out packet upon counterparty channel closure.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgTimeoutOnClose")]
pub struct MsgTimeoutOnClose {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_unreceived: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_close: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_sequence_recv: u64,
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub counterparty_upgrade_sequence: u64,
}
/// MsgTimeoutOnCloseResponse defines the Msg/TimeoutOnClose response type.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgTimeoutOnCloseResponse")]
pub struct MsgTimeoutOnCloseResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
}
/// MsgAcknowledgement receives incoming IBC acknowledgement
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgAcknowledgement")]
pub struct MsgAcknowledgement {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub acknowledgement: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_acked: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgAcknowledgementResponse defines the Msg/Acknowledgement response type.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgAcknowledgementResponse")]
pub struct MsgAcknowledgementResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
}
/// MsgChannelUpgradeInit defines the request type for the ChannelUpgradeInit rpc
/// WARNING: Initializing a channel upgrade in the same block as opening the channel
/// may result in the counterparty being incapable of opening.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeInit")]
pub struct MsgChannelUpgradeInit {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub fields: ::core::option::Option<UpgradeFields>,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelUpgradeInitResponse defines the MsgChannelUpgradeInit response type
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeInitResponse")]
pub struct MsgChannelUpgradeInitResponse {
    #[prost(message, optional, tag = "1")]
    pub upgrade: ::core::option::Option<Upgrade>,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upgrade_sequence: u64,
}
/// MsgChannelUpgradeTry defines the request type for the ChannelUpgradeTry rpc
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeTry")]
pub struct MsgChannelUpgradeTry {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub proposed_upgrade_connection_hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub counterparty_upgrade_fields: ::core::option::Option<UpgradeFields>,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub counterparty_upgrade_sequence: u64,
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_upgrade: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "8")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "9")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelUpgradeTryResponse defines the MsgChannelUpgradeTry response type
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeTryResponse")]
pub struct MsgChannelUpgradeTryResponse {
    #[prost(message, optional, tag = "1")]
    pub upgrade: ::core::option::Option<Upgrade>,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upgrade_sequence: u64,
    #[prost(enumeration = "ResponseResultType", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
}
/// MsgChannelUpgradeAck defines the request type for the ChannelUpgradeAck rpc
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeAck")]
pub struct MsgChannelUpgradeAck {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub counterparty_upgrade: ::core::option::Option<Upgrade>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_upgrade: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "7")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelUpgradeAckResponse defines MsgChannelUpgradeAck response type
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeAckResponse")]
pub struct MsgChannelUpgradeAckResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
}
/// MsgChannelUpgradeConfirm defines the request type for the ChannelUpgradeConfirm rpc
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeConfirm")]
pub struct MsgChannelUpgradeConfirm {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(enumeration = "State", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub counterparty_channel_state: i32,
    #[prost(message, optional, tag = "4")]
    pub counterparty_upgrade: ::core::option::Option<Upgrade>,
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_upgrade: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "8")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelUpgradeConfirmResponse defines MsgChannelUpgradeConfirm response type
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeConfirmResponse")]
pub struct MsgChannelUpgradeConfirmResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
}
/// MsgChannelUpgradeOpen defines the request type for the ChannelUpgradeOpen rpc
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeOpen")]
pub struct MsgChannelUpgradeOpen {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(enumeration = "State", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub counterparty_channel_state: i32,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub counterparty_upgrade_sequence: u64,
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "7")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelUpgradeOpenResponse defines the MsgChannelUpgradeOpen response type
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeOpenResponse")]
pub struct MsgChannelUpgradeOpenResponse {}
/// MsgChannelUpgradeTimeout defines the request type for the ChannelUpgradeTimeout rpc
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeTimeout")]
pub struct MsgChannelUpgradeTimeout {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub counterparty_channel: ::core::option::Option<Channel>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_channel: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelUpgradeTimeoutRepsonse defines the MsgChannelUpgradeTimeout response type
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeTimeoutResponse")]
pub struct MsgChannelUpgradeTimeoutResponse {}
/// MsgChannelUpgradeCancel defines the request type for the ChannelUpgradeCancel rpc
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeCancel")]
pub struct MsgChannelUpgradeCancel {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub error_receipt: ::core::option::Option<ErrorReceipt>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proof_error_receipt: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelUpgradeCancelResponse defines the MsgChannelUpgradeCancel response type
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgChannelUpgradeCancelResponse")]
pub struct MsgChannelUpgradeCancelResponse {}
/// MsgUpdateParams is the MsgUpdateParams request type.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the channel parameters to update.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgPruneAcknowledgements defines the request type for the PruneAcknowledgements rpc.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgPruneAcknowledgements")]
pub struct MsgPruneAcknowledgements {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub limit: u64,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgPruneAcknowledgementsResponse defines the response type for the PruneAcknowledgements rpc.
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
#[proto_message(type_url = "/ibc.core.channel.v1.MsgPruneAcknowledgementsResponse")]
pub struct MsgPruneAcknowledgementsResponse {
    /// Number of sequences pruned (includes both packet acknowledgements and packet receipts where appropriate).
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_pruned_sequences: u64,
    /// Number of sequences left after pruning.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_remaining_sequences: u64,
}
/// ResponseResultType defines the possible outcomes of the execution of a message
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ResponseResultType {
    /// Default zero value enumeration
    Unspecified = 0,
    /// The message did not call the IBC application callbacks (because, for example, the packet had already been relayed)
    Noop = 1,
    /// The message was executed successfully
    Success = 2,
    /// The message was executed unsuccessfully
    Failure = 3,
}
impl ResponseResultType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResponseResultType::Unspecified => "RESPONSE_RESULT_TYPE_UNSPECIFIED",
            ResponseResultType::Noop => "RESPONSE_RESULT_TYPE_NOOP",
            ResponseResultType::Success => "RESPONSE_RESULT_TYPE_SUCCESS",
            ResponseResultType::Failure => "RESPONSE_RESULT_TYPE_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESPONSE_RESULT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RESPONSE_RESULT_TYPE_NOOP" => Some(Self::Noop),
            "RESPONSE_RESULT_TYPE_SUCCESS" => Some(Self::Success),
            "RESPONSE_RESULT_TYPE_FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
pub struct ChannelQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ChannelQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn channel(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
    ) -> Result<QueryChannelResponse, cosmwasm_std::StdError> {
        QueryChannelRequest {
            port_id,
            channel_id,
        }
        .query(self.querier)
    }
    pub fn channels(
        &self,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryChannelsResponse, cosmwasm_std::StdError> {
        QueryChannelsRequest { pagination }.query(self.querier)
    }
    pub fn connection_channels(
        &self,
        connection: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryConnectionChannelsResponse, cosmwasm_std::StdError> {
        QueryConnectionChannelsRequest {
            connection,
            pagination,
        }
        .query(self.querier)
    }
    pub fn channel_client_state(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
    ) -> Result<QueryChannelClientStateResponse, cosmwasm_std::StdError> {
        QueryChannelClientStateRequest {
            port_id,
            channel_id,
        }
        .query(self.querier)
    }
    pub fn channel_consensus_state(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
        revision_number: u64,
        revision_height: u64,
    ) -> Result<QueryChannelConsensusStateResponse, cosmwasm_std::StdError> {
        QueryChannelConsensusStateRequest {
            port_id,
            channel_id,
            revision_number,
            revision_height,
        }
        .query(self.querier)
    }
    pub fn packet_commitment(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
        sequence: u64,
    ) -> Result<QueryPacketCommitmentResponse, cosmwasm_std::StdError> {
        QueryPacketCommitmentRequest {
            port_id,
            channel_id,
            sequence,
        }
        .query(self.querier)
    }
    pub fn packet_commitments(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPacketCommitmentsResponse, cosmwasm_std::StdError> {
        QueryPacketCommitmentsRequest {
            port_id,
            channel_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn packet_receipt(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
        sequence: u64,
    ) -> Result<QueryPacketReceiptResponse, cosmwasm_std::StdError> {
        QueryPacketReceiptRequest {
            port_id,
            channel_id,
            sequence,
        }
        .query(self.querier)
    }
    pub fn packet_acknowledgement(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
        sequence: u64,
    ) -> Result<QueryPacketAcknowledgementResponse, cosmwasm_std::StdError> {
        QueryPacketAcknowledgementRequest {
            port_id,
            channel_id,
            sequence,
        }
        .query(self.querier)
    }
    pub fn packet_acknowledgements(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
        packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
    ) -> Result<QueryPacketAcknowledgementsResponse, cosmwasm_std::StdError> {
        QueryPacketAcknowledgementsRequest {
            port_id,
            channel_id,
            pagination,
            packet_commitment_sequences,
        }
        .query(self.querier)
    }
    pub fn unreceived_packets(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
        packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
    ) -> Result<QueryUnreceivedPacketsResponse, cosmwasm_std::StdError> {
        QueryUnreceivedPacketsRequest {
            port_id,
            channel_id,
            packet_commitment_sequences,
        }
        .query(self.querier)
    }
    pub fn unreceived_acks(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
        packet_ack_sequences: ::prost::alloc::vec::Vec<u64>,
    ) -> Result<QueryUnreceivedAcksResponse, cosmwasm_std::StdError> {
        QueryUnreceivedAcksRequest {
            port_id,
            channel_id,
            packet_ack_sequences,
        }
        .query(self.querier)
    }
    pub fn next_sequence_receive(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
    ) -> Result<QueryNextSequenceReceiveResponse, cosmwasm_std::StdError> {
        QueryNextSequenceReceiveRequest {
            port_id,
            channel_id,
        }
        .query(self.querier)
    }
    pub fn next_sequence_send(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
    ) -> Result<QueryNextSequenceSendResponse, cosmwasm_std::StdError> {
        QueryNextSequenceSendRequest {
            port_id,
            channel_id,
        }
        .query(self.querier)
    }
    pub fn upgrade_error(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
    ) -> Result<QueryUpgradeErrorResponse, cosmwasm_std::StdError> {
        QueryUpgradeErrorRequest {
            port_id,
            channel_id,
        }
        .query(self.querier)
    }
    pub fn upgrade(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
    ) -> Result<QueryUpgradeResponse, cosmwasm_std::StdError> {
        QueryUpgradeRequest {
            port_id,
            channel_id,
        }
        .query(self.querier)
    }
    pub fn channel_params(&self) -> Result<QueryChannelParamsResponse, cosmwasm_std::StdError> {
        QueryChannelParamsRequest {}.query(self.querier)
    }
}
