use osmosis_std_derive::CosmwasmExt;
/// ClientState from Tendermint tracks the current validator set, latest height,
/// and a possible frozen height.
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
#[proto_message(type_url = "/ibc.lightclients.tendermint.v1.ClientState")]
pub struct ClientState {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub trust_level: ::core::option::Option<Fraction>,
    /// duration of the period since the LastestTimestamp during which the
    /// submitted headers are valid for upgrade
    #[prost(message, optional, tag = "3")]
    pub trusting_period: ::core::option::Option<crate::shim::Duration>,
    /// duration of the staking unbonding period
    #[prost(message, optional, tag = "4")]
    pub unbonding_period: ::core::option::Option<crate::shim::Duration>,
    /// defines how much new (untrusted) header's Time can drift into the future.
    #[prost(message, optional, tag = "5")]
    pub max_clock_drift: ::core::option::Option<crate::shim::Duration>,
    /// Block height when the client was frozen due to a misbehaviour
    #[prost(message, optional, tag = "6")]
    pub frozen_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    /// Latest height the client was updated to
    #[prost(message, optional, tag = "7")]
    pub latest_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    /// Proof specifications used in verifying counterparty state
    #[prost(message, repeated, tag = "8")]
    pub proof_specs:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::ics23::v1::ProofSpec>,
    /// Path at which next upgraded client will be committed.
    /// Each element corresponds to the key for a single CommitmentProof in the
    /// chained proof. NOTE: ClientState must stored under
    /// `{upgradePath}/{upgradeHeight}/clientState` ConsensusState must be stored
    /// under `{upgradepath}/{upgradeHeight}/consensusState` For SDK chains using
    /// the default upgrade module, upgrade_path should be []string{"upgrade",
    /// "upgradedIBCState"}`
    #[prost(string, repeated, tag = "9")]
    pub upgrade_path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// allow_update_after_expiry is deprecated
    #[deprecated]
    #[prost(bool, tag = "10")]
    pub allow_update_after_expiry: bool,
    /// allow_update_after_misbehaviour is deprecated
    #[deprecated]
    #[prost(bool, tag = "11")]
    pub allow_update_after_misbehaviour: bool,
}
/// ConsensusState defines the consensus state from Tendermint.
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
#[proto_message(type_url = "/ibc.lightclients.tendermint.v1.ConsensusState")]
pub struct ConsensusState {
    /// timestamp that corresponds to the block height in which the ConsensusState
    /// was stored.
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    /// commitment root (i.e app hash)
    #[prost(message, optional, tag = "2")]
    pub root: ::core::option::Option<super::super::super::core::commitment::v1::MerkleRoot>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
}
/// Misbehaviour is a wrapper over two conflicting Headers
/// that implements Misbehaviour interface expected by ICS-02
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
#[proto_message(type_url = "/ibc.lightclients.tendermint.v1.Misbehaviour")]
pub struct Misbehaviour {
    /// ClientID is deprecated
    #[deprecated]
    #[prost(string, tag = "1")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub header_1: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "3")]
    pub header_2: ::core::option::Option<Header>,
}
/// Header defines the Tendermint client consensus Header.
/// It encapsulates all the information necessary to update from a trusted
/// Tendermint ConsensusState. The inclusion of TrustedHeight and
/// TrustedValidators allows this update to process correctly, so long as the
/// ConsensusState for the TrustedHeight exists, this removes race conditions
/// among relayers The SignedHeader and ValidatorSet are the new untrusted update
/// fields for the client. The TrustedHeight is the height of a stored
/// ConsensusState on the client that will be used to verify the new untrusted
/// header. The Trusted ConsensusState must be within the unbonding period of
/// current time in order to correctly verify, and the TrustedValidators must
/// hash to TrustedConsensusState.NextValidatorsHash since that is the last
/// trusted validator set at the TrustedHeight.
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
#[proto_message(type_url = "/ibc.lightclients.tendermint.v1.Header")]
pub struct Header {
    #[prost(message, optional, tag = "1")]
    pub signed_header:
        ::core::option::Option<super::super::super::super::tendermint::types::SignedHeader>,
    #[prost(message, optional, tag = "2")]
    pub validator_set:
        ::core::option::Option<super::super::super::super::tendermint::types::ValidatorSet>,
    #[prost(message, optional, tag = "3")]
    pub trusted_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    #[prost(message, optional, tag = "4")]
    pub trusted_validators:
        ::core::option::Option<super::super::super::super::tendermint::types::ValidatorSet>,
}
/// Fraction defines the protobuf message type for tmmath.Fraction that only
/// supports positive values.
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
#[proto_message(type_url = "/ibc.lightclients.tendermint.v1.Fraction")]
pub struct Fraction {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub numerator: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub denominator: u64,
}
