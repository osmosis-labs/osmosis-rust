use osmosis_std_derive::CosmwasmExt;
/// AccessTypeParam
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.AccessTypeParam")]
pub struct AccessTypeParam {
    #[prost(enumeration = "AccessType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub value: i32,
}
/// AccessConfig access control type.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.AccessConfig")]
pub struct AccessConfig {
    #[prost(enumeration = "AccessType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub permission: i32,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the set of wasm parameters.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.Params")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub code_upload_access: ::core::option::Option<AccessConfig>,
    #[prost(enumeration = "AccessType", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub instantiate_default_permission: i32,
}
/// CodeInfo is data for the uploaded contract WASM code
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.CodeInfo")]
pub struct CodeInfo {
    /// CodeHash is the unique identifier created by wasmvm
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    /// Creator address who initially stored the code
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    /// InstantiateConfig access control to apply on contract creation, optional
    #[prost(message, optional, tag = "5")]
    pub instantiate_config: ::core::option::Option<AccessConfig>,
}
/// ContractInfo stores a WASM contract instance
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.ContractInfo")]
pub struct ContractInfo {
    /// CodeID is the reference to the stored Wasm code
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Creator address who initially instantiated the contract
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "3")]
    pub admin: ::prost::alloc::string::String,
    /// Label is optional metadata to be stored with a contract instance.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// Created Tx position when the contract was instantiated.
    #[prost(message, optional, tag = "5")]
    pub created: ::core::option::Option<AbsoluteTxPosition>,
    #[prost(string, tag = "6")]
    #[serde(alias = "ibc_portID")]
    pub ibc_port_id: ::prost::alloc::string::String,
    /// Extension is an extension point to store custom metadata within the
    /// persistence model.
    #[prost(message, optional, tag = "7")]
    pub extension: ::core::option::Option<crate::shim::Any>,
}
/// ContractCodeHistoryEntry metadata to a contract.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.ContractCodeHistoryEntry")]
pub struct ContractCodeHistoryEntry {
    #[prost(enumeration = "ContractCodeHistoryOperationType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub operation: i32,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "2")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Updated Tx position when the operation was executed.
    #[prost(message, optional, tag = "3")]
    pub updated: ::core::option::Option<AbsoluteTxPosition>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// AbsoluteTxPosition is a unique transaction position that allows for global
/// ordering of transactions.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.AbsoluteTxPosition")]
pub struct AbsoluteTxPosition {
    /// BlockHeight is the block the contract was created at
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_height: u64,
    /// TxIndex is a monotonic counter within the block (actual transaction index,
    /// or gas consumed)
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tx_index: u64,
}
/// Model is a struct that holds a KV pair
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.Model")]
pub struct Model {
    /// hex-encode key to read it better (this is often ascii)
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// base64-encode raw value
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// AccessType permission types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum AccessType {
    /// AccessTypeUnspecified placeholder for empty value
    Unspecified = 0,
    /// AccessTypeNobody forbidden
    Nobody = 1,
    /// AccessTypeEverybody unrestricted
    Everybody = 3,
    /// AccessTypeAnyOfAddresses allow any of the addresses
    AnyOfAddresses = 4,
}
impl AccessType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessType::Unspecified => "ACCESS_TYPE_UNSPECIFIED",
            AccessType::Nobody => "ACCESS_TYPE_NOBODY",
            AccessType::Everybody => "ACCESS_TYPE_EVERYBODY",
            AccessType::AnyOfAddresses => "ACCESS_TYPE_ANY_OF_ADDRESSES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCESS_TYPE_NOBODY" => Some(Self::Nobody),
            "ACCESS_TYPE_EVERYBODY" => Some(Self::Everybody),
            "ACCESS_TYPE_ANY_OF_ADDRESSES" => Some(Self::AnyOfAddresses),
            _ => None,
        }
    }
}
/// ContractCodeHistoryOperationType actions that caused a code change
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ContractCodeHistoryOperationType {
    /// ContractCodeHistoryOperationTypeUnspecified placeholder for empty value
    Unspecified = 0,
    /// ContractCodeHistoryOperationTypeInit on chain contract instantiation
    Init = 1,
    /// ContractCodeHistoryOperationTypeMigrate code migration
    Migrate = 2,
    /// ContractCodeHistoryOperationTypeGenesis based on genesis data
    Genesis = 3,
}
impl ContractCodeHistoryOperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractCodeHistoryOperationType::Unspecified => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED"
            }
            ContractCodeHistoryOperationType::Init => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT",
            ContractCodeHistoryOperationType::Migrate => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
            }
            ContractCodeHistoryOperationType::Genesis => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT" => Some(Self::Init),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE" => Some(Self::Migrate),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS" => Some(Self::Genesis),
            _ => None,
        }
    }
}
/// StoreCodeAuthorization defines authorization for wasm code upload.
/// Since: wasmd 0.42
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.StoreCodeAuthorization")]
pub struct StoreCodeAuthorization {
    /// Grants for code upload
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<CodeGrant>,
}
/// ContractExecutionAuthorization defines authorization for wasm execute.
/// Since: wasmd 0.30
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.ContractExecutionAuthorization")]
pub struct ContractExecutionAuthorization {
    /// Grants for contract executions
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<ContractGrant>,
}
/// ContractMigrationAuthorization defines authorization for wasm contract
/// migration. Since: wasmd 0.30
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.ContractMigrationAuthorization")]
pub struct ContractMigrationAuthorization {
    /// Grants for contract migrations
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<ContractGrant>,
}
/// CodeGrant a granted permission for a single code
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.CodeGrant")]
pub struct CodeGrant {
    /// CodeHash is the unique identifier created by wasmvm
    /// Wildcard "*" is used to specify any kind of grant.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission is the superset access control to apply
    /// on contract creation.
    /// Optional
    #[prost(message, optional, tag = "2")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
}
/// ContractGrant a granted permission for a single contract
/// Since: wasmd 0.30
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.ContractGrant")]
pub struct ContractGrant {
    /// Contract is the bech32 address of the smart contract
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    /// Limit defines execution limits that are enforced and updated when the grant
    /// is applied. When the limit lapsed the grant is removed.
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<crate::shim::Any>,
    /// Filter define more fine-grained control on the message payload passed
    /// to the contract in the operation. When no filter applies on execution, the
    /// operation is prohibited.
    #[prost(message, optional, tag = "3")]
    pub filter: ::core::option::Option<crate::shim::Any>,
}
/// MaxCallsLimit limited number of calls to the contract. No funds transferable.
/// Since: wasmd 0.30
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MaxCallsLimit")]
pub struct MaxCallsLimit {
    /// Remaining number that is decremented on each execution
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub remaining: u64,
}
/// MaxFundsLimit defines the maximal amounts that can be sent to the contract.
/// Since: wasmd 0.30
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MaxFundsLimit")]
pub struct MaxFundsLimit {
    /// Amounts is the maximal amount of tokens transferable to the contract.
    #[prost(message, repeated, tag = "1")]
    pub amounts: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// CombinedLimit defines the maximal amounts that can be sent to a contract and
/// the maximal number of calls executable. Both need to remain >0 to be valid.
/// Since: wasmd 0.30
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.CombinedLimit")]
pub struct CombinedLimit {
    /// Remaining number that is decremented on each execution
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub calls_remaining: u64,
    /// Amounts is the maximal amount of tokens transferable to the contract.
    #[prost(message, repeated, tag = "2")]
    pub amounts: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// AllowAllMessagesFilter is a wildcard to allow any type of contract payload
/// message.
/// Since: wasmd 0.30
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.AllowAllMessagesFilter")]
pub struct AllowAllMessagesFilter {}
/// AcceptedMessageKeysFilter accept only the specific contract message keys in
/// the json object to be executed.
/// Since: wasmd 0.30
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.AcceptedMessageKeysFilter")]
pub struct AcceptedMessageKeysFilter {
    /// Messages is the list of unique keys
    #[prost(string, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AcceptedMessagesFilter accept only the specific raw contract messages to be
/// executed.
/// Since: wasmd 0.30
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.AcceptedMessagesFilter")]
pub struct AcceptedMessagesFilter {
    /// Messages is the list of raw contract messages
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// GenesisState - genesis state of x/wasm
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub codes: ::prost::alloc::vec::Vec<Code>,
    #[prost(message, repeated, tag = "3")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
    #[prost(message, repeated, tag = "4")]
    pub sequences: ::prost::alloc::vec::Vec<Sequence>,
}
/// Code struct encompasses CodeInfo and CodeBytes
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.Code")]
pub struct Code {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    #[prost(message, optional, tag = "2")]
    pub code_info: ::core::option::Option<CodeInfo>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
    /// Pinned to wasmvm cache
    #[prost(bool, tag = "4")]
    pub pinned: bool,
}
/// Contract struct encompasses ContractAddress, ContractInfo, and ContractState
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.Contract")]
pub struct Contract {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub contract_info: ::core::option::Option<ContractInfo>,
    #[prost(message, repeated, tag = "3")]
    pub contract_state: ::prost::alloc::vec::Vec<Model>,
    #[prost(message, repeated, tag = "4")]
    pub contract_code_history: ::prost::alloc::vec::Vec<ContractCodeHistoryEntry>,
}
/// Sequence key and value of an id generation counter
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.Sequence")]
pub struct Sequence {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub id_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub value: u64,
}
/// MsgIBCSend
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgIBCSend")]
pub struct MsgIbcSend {
    /// the channel by which the packet will be sent
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timeout_height: u64,
    /// Timeout timestamp (in nanoseconds) relative to the current block timestamp.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timeout_timestamp: u64,
    /// Data is the payload to transfer. We must not make assumption what format or
    /// content is in here.
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgIBCSendResponse
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgIBCSendResponse")]
pub struct MsgIbcSendResponse {
    /// Sequence number of the IBC packet sent
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
/// MsgIBCCloseChannel port and channel need to be owned by the contract
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgIBCCloseChannel")]
pub struct MsgIbcCloseChannel {
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit StoreCodeProposal. To submit WASM code to the system,
/// a simple MsgStoreCode can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.StoreCodeProposal")]
#[deprecated]
pub struct StoreCodeProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission to apply on contract creation, optional
    #[prost(message, optional, tag = "7")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
    /// UnpinCode code on upload, optional
    #[prost(bool, tag = "8")]
    pub unpin_code: bool,
    /// Source is the URL where the code is hosted
    #[prost(string, tag = "9")]
    pub source: ::prost::alloc::string::String,
    /// Builder is the docker image used to build the code deterministically, used
    /// for smart contract verification
    #[prost(string, tag = "10")]
    pub builder: ::prost::alloc::string::String,
    /// CodeHash is the SHA256 sum of the code outputted by builder, used for smart
    /// contract verification
    #[prost(bytes = "vec", tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit InstantiateContractProposal. To instantiate a contract,
/// a simple MsgInstantiateContract can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.InstantiateContractProposal")]
#[deprecated]
pub struct InstantiateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "4")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "5")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Label is optional metadata to be stored with a constract instance.
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "8")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit InstantiateContract2Proposal. To instantiate contract 2,
/// a simple MsgInstantiateContract2 can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.InstantiateContract2Proposal")]
#[deprecated]
pub struct InstantiateContract2Proposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's enviroment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "4")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "5")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Label is optional metadata to be stored with a constract instance.
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encode message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "8")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Salt is an arbitrary value provided by the sender. Size can be 1 to 64.
    #[prost(bytes = "vec", tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub salt: ::prost::alloc::vec::Vec<u8>,
    /// FixMsg include the msg value into the hash for the predictable address.
    /// Default is false
    #[prost(bool, tag = "10")]
    pub fix_msg: bool,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit MigrateContractProposal. To migrate a contract,
/// a simple MsgMigrateContract can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MigrateContractProposal")]
#[deprecated]
pub struct MigrateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    ///
    /// Note: skipping 3 as this was previously used for unneeded run_as
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
    /// CodeID references the new WASM code
    #[prost(uint64, tag = "5")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Msg json encoded message to be passed to the contract on migration
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit SudoContractProposal. To call sudo on a contract,
/// a simple MsgSudoContract can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.SudoContractProposal")]
#[deprecated]
pub struct SudoContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract as sudo
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit ExecuteContractProposal. To call execute on a contract,
/// a simple MsgExecuteContract can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.ExecuteContractProposal")]
#[deprecated]
pub struct ExecuteContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract as execute
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "6")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit UpdateAdminProposal. To set an admin for a contract,
/// a simple MsgUpdateAdmin can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.UpdateAdminProposal")]
#[deprecated]
pub struct UpdateAdminProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// NewAdmin address to be set
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit ClearAdminProposal. To clear the admin of a contract,
/// a simple MsgClearAdmin can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.ClearAdminProposal")]
#[deprecated]
pub struct ClearAdminProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit PinCodesProposal. To pin a set of code ids in the wasmvm
/// cache, a simple MsgPinCodes can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.PinCodesProposal")]
#[deprecated]
pub struct PinCodesProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// CodeIDs references the new WASM codes
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    #[serde(alias = "codeIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit UnpinCodesProposal. To unpin a set of code ids in the wasmvm
/// cache, a simple MsgUnpinCodes can be invoked from the x/gov module via
/// a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.UnpinCodesProposal")]
#[deprecated]
pub struct UnpinCodesProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// CodeIDs references the WASM codes
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    #[serde(alias = "codeIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
}
/// AccessConfigUpdate contains the code id and the access config to be
/// applied.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.AccessConfigUpdate")]
pub struct AccessConfigUpdate {
    /// CodeID is the reference to the stored WASM code to be updated
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// InstantiatePermission to apply to the set of code ids
    #[prost(message, optional, tag = "2")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit UpdateInstantiateConfigProposal. To update instantiate config
/// to a set of code ids, a simple MsgUpdateInstantiateConfig can be invoked from
/// the x/gov module via a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.UpdateInstantiateConfigProposal")]
#[deprecated]
pub struct UpdateInstantiateConfigProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// AccessConfigUpdate contains the list of code ids and the access config
    /// to be applied.
    #[prost(message, repeated, tag = "3")]
    pub access_config_updates: ::prost::alloc::vec::Vec<AccessConfigUpdate>,
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit StoreAndInstantiateContractProposal. To store and instantiate
/// the contract, a simple MsgStoreAndInstantiateContract can be invoked from
/// the x/gov module via a v1 governance proposal.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.StoreAndInstantiateContractProposal")]
#[deprecated]
pub struct StoreAndInstantiateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission to apply on contract creation, optional
    #[prost(message, optional, tag = "5")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
    /// UnpinCode code on upload, optional
    #[prost(bool, tag = "6")]
    pub unpin_code: bool,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "7")]
    pub admin: ::prost::alloc::string::String,
    /// Label is optional metadata to be stored with a constract instance.
    #[prost(string, tag = "8")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "10")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Source is the URL where the code is hosted
    #[prost(string, tag = "11")]
    pub source: ::prost::alloc::string::String,
    /// Builder is the docker image used to build the code deterministically, used
    /// for smart contract verification
    #[prost(string, tag = "12")]
    pub builder: ::prost::alloc::string::String,
    /// CodeHash is the SHA256 sum of the code outputted by builder, used for smart
    /// contract verification
    #[prost(bytes = "vec", tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
}
/// QueryContractInfoRequest is the request type for the Query/ContractInfo RPC
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryContractInfoRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/ContractInfo",
    response_type = QueryContractInfoResponse
)]
pub struct QueryContractInfoRequest {
    /// address is the address of the contract to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryContractInfoResponse is the response type for the Query/ContractInfo RPC
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryContractInfoResponse")]
pub struct QueryContractInfoResponse {
    /// address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub contract_info: ::core::option::Option<ContractInfo>,
}
/// QueryContractHistoryRequest is the request type for the Query/ContractHistory
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryContractHistoryRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/ContractHistory",
    response_type = QueryContractHistoryResponse
)]
pub struct QueryContractHistoryRequest {
    /// address is the address of the contract to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryContractHistoryResponse is the response type for the
/// Query/ContractHistory RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryContractHistoryResponse")]
pub struct QueryContractHistoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<ContractCodeHistoryEntry>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryContractsByCodeRequest is the request type for the Query/ContractsByCode
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryContractsByCodeRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/ContractsByCode",
    response_type = QueryContractsByCodeResponse
)]
pub struct QueryContractsByCodeRequest {
    /// grpc-gateway_out does not support Go style CodID
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryContractsByCodeResponse is the response type for the
/// Query/ContractsByCode RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryContractsByCodeResponse")]
pub struct QueryContractsByCodeResponse {
    /// contracts are a set of contract addresses
    #[prost(string, repeated, tag = "1")]
    pub contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryAllContractStateRequest is the request type for the
/// Query/AllContractState RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryAllContractStateRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/AllContractState",
    response_type = QueryAllContractStateResponse
)]
pub struct QueryAllContractStateRequest {
    /// address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllContractStateResponse is the response type for the
/// Query/AllContractState RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryAllContractStateResponse")]
pub struct QueryAllContractStateResponse {
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryRawContractStateRequest is the request type for the
/// Query/RawContractState RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryRawContractStateRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/RawContractState",
    response_type = QueryRawContractStateResponse
)]
pub struct QueryRawContractStateRequest {
    /// address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub query_data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryRawContractStateResponse is the response type for the
/// Query/RawContractState RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryRawContractStateResponse")]
pub struct QueryRawContractStateResponse {
    /// Data contains the raw store data
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QuerySmartContractStateRequest is the request type for the
/// Query/SmartContractState RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QuerySmartContractStateRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/SmartContractState",
    response_type = QuerySmartContractStateResponse
)]
pub struct QuerySmartContractStateRequest {
    /// address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// QueryData contains the query data passed to the contract
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub query_data: ::prost::alloc::vec::Vec<u8>,
}
/// QuerySmartContractStateResponse is the response type for the
/// Query/SmartContractState RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QuerySmartContractStateResponse")]
pub struct QuerySmartContractStateResponse {
    /// Data contains the json data returned from the smart contract
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryCodeRequest is the request type for the Query/Code RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryCodeRequest")]
#[proto_query(path = "/cosmwasm.wasm.v1.Query/Code", response_type = QueryCodeResponse)]
pub struct QueryCodeRequest {
    /// grpc-gateway_out does not support Go style CodID
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
}
/// CodeInfoResponse contains code meta data from CodeInfo
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.CodeInfoResponse")]
pub struct CodeInfoResponse {
    /// id for legacy support
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
}
/// QueryCodeResponse is the response type for the Query/Code RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryCodeResponse")]
pub struct QueryCodeResponse {
    #[prost(message, optional, tag = "1")]
    pub code_info: ::core::option::Option<CodeInfoResponse>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryCodesRequest is the request type for the Query/Codes RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryCodesRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/Codes",
    response_type = QueryCodesResponse
)]
pub struct QueryCodesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryCodesResponse is the response type for the Query/Codes RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryCodesResponse")]
pub struct QueryCodesResponse {
    #[prost(message, repeated, tag = "1")]
    pub code_infos: ::prost::alloc::vec::Vec<CodeInfoResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPinnedCodesRequest is the request type for the Query/PinnedCodes
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryPinnedCodesRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/PinnedCodes",
    response_type = QueryPinnedCodesResponse
)]
pub struct QueryPinnedCodesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPinnedCodesResponse is the response type for the
/// Query/PinnedCodes RPC method
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryPinnedCodesResponse")]
pub struct QueryPinnedCodesResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    #[serde(alias = "codeIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/Params",
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryContractsByCreatorRequest is the request type for the
/// Query/ContractsByCreator RPC method.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryContractsByCreatorRequest")]
#[proto_query(
    path = "/cosmwasm.wasm.v1.Query/ContractsByCreator",
    response_type = QueryContractsByCreatorResponse
)]
pub struct QueryContractsByCreatorRequest {
    /// CreatorAddress is the address of contract creator
    #[prost(string, tag = "1")]
    pub creator_address: ::prost::alloc::string::String,
    /// Pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryContractsByCreatorResponse is the response type for the
/// Query/ContractsByCreator RPC method.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.QueryContractsByCreatorResponse")]
pub struct QueryContractsByCreatorResponse {
    /// ContractAddresses result set
    #[prost(string, repeated, tag = "1")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgStoreCode submit Wasm code to the system
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgStoreCode")]
pub struct MsgStoreCode {
    /// Sender is the actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission access control to apply on contract creation,
    /// optional
    #[prost(message, optional, tag = "5")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
}
/// MsgStoreCodeResponse returns store result data.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgStoreCodeResponse")]
pub struct MsgStoreCodeResponse {
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Checksum is the sha256 hash of the stored code
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
}
/// MsgInstantiateContract create a new smart contract instance for the given
/// code id.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgInstantiateContract")]
pub struct MsgInstantiateContract {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "3")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Label is optional metadata to be stored with a contract instance.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "6")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgInstantiateContractResponse return instantiation result data
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgInstantiateContractResponse")]
pub struct MsgInstantiateContractResponse {
    /// Address is the bech32 address of the new contract instance.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgInstantiateContract2 create a new smart contract instance for the given
/// code id with a predicable address.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgInstantiateContract2")]
pub struct MsgInstantiateContract2 {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "3")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Label is optional metadata to be stored with a contract instance.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "6")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Salt is an arbitrary value provided by the sender. Size can be 1 to 64.
    #[prost(bytes = "vec", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub salt: ::prost::alloc::vec::Vec<u8>,
    /// FixMsg include the msg value into the hash for the predictable address.
    /// Default is false
    #[prost(bool, tag = "8")]
    pub fix_msg: bool,
}
/// MsgInstantiateContract2Response return instantiation result data
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgInstantiateContract2Response")]
pub struct MsgInstantiateContract2Response {
    /// Address is the bech32 address of the new contract instance.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgExecuteContract submits the given message data to a smart contract
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgExecuteContract")]
pub struct MsgExecuteContract {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "2")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on execution
    #[prost(message, repeated, tag = "5")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgExecuteContractResponse returns execution result data.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgExecuteContractResponse")]
pub struct MsgExecuteContractResponse {
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgMigrateContract runs a code upgrade/ downgrade for a smart contract
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgMigrateContract")]
pub struct MsgMigrateContract {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "2")]
    pub contract: ::prost::alloc::string::String,
    /// CodeID references the new WASM code
    #[prost(uint64, tag = "3")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Msg json encoded message to be passed to the contract on migration
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// MsgMigrateContractResponse returns contract migration result data.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgMigrateContractResponse")]
pub struct MsgMigrateContractResponse {
    /// Data contains same raw bytes returned as data from the wasm contract.
    /// (May be empty)
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgUpdateAdmin sets a new admin for a smart contract
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUpdateAdmin")]
pub struct MsgUpdateAdmin {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// NewAdmin address to be set
    #[prost(string, tag = "2")]
    pub new_admin: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
/// MsgUpdateAdminResponse returns empty data
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUpdateAdminResponse")]
pub struct MsgUpdateAdminResponse {}
/// MsgClearAdmin removes any admin stored for a smart contract
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgClearAdmin")]
pub struct MsgClearAdmin {
    /// Sender is the actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
/// MsgClearAdminResponse returns empty data
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgClearAdminResponse")]
pub struct MsgClearAdminResponse {}
/// MsgUpdateInstantiateConfig updates instantiate config for a smart contract
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUpdateInstantiateConfig")]
pub struct MsgUpdateInstantiateConfig {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// CodeID references the stored WASM code
    #[prost(uint64, tag = "2")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// NewInstantiatePermission is the new access control
    #[prost(message, optional, tag = "3")]
    pub new_instantiate_permission: ::core::option::Option<AccessConfig>,
}
/// MsgUpdateInstantiateConfigResponse returns empty data
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUpdateInstantiateConfigResponse")]
pub struct MsgUpdateInstantiateConfigResponse {}
/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/wasm parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgSudoContract is the MsgSudoContract request type.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgSudoContract")]
pub struct MsgSudoContract {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "2")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract as sudo
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// MsgSudoContractResponse defines the response structure for executing a
/// MsgSudoContract message.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgSudoContractResponse")]
pub struct MsgSudoContractResponse {
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgPinCodes is the MsgPinCodes request type.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgPinCodes")]
pub struct MsgPinCodes {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// CodeIDs references the new WASM codes
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    #[serde(alias = "codeIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
}
/// MsgPinCodesResponse defines the response structure for executing a
/// MsgPinCodes message.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgPinCodesResponse")]
pub struct MsgPinCodesResponse {}
/// MsgUnpinCodes is the MsgUnpinCodes request type.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUnpinCodes")]
pub struct MsgUnpinCodes {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// CodeIDs references the WASM codes
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    #[serde(alias = "codeIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
}
/// MsgUnpinCodesResponse defines the response structure for executing a
/// MsgUnpinCodes message.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUnpinCodesResponse")]
pub struct MsgUnpinCodesResponse {}
/// MsgStoreAndInstantiateContract is the MsgStoreAndInstantiateContract
/// request type.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgStoreAndInstantiateContract")]
pub struct MsgStoreAndInstantiateContract {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission to apply on contract creation, optional
    #[prost(message, optional, tag = "4")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
    /// UnpinCode code on upload, optional. As default the uploaded contract is
    /// pinned to cache.
    #[prost(bool, tag = "5")]
    pub unpin_code: bool,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "6")]
    pub admin: ::prost::alloc::string::String,
    /// Label is optional metadata to be stored with a constract instance.
    #[prost(string, tag = "7")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred from the authority account to the contract
    /// on instantiation
    #[prost(message, repeated, tag = "9")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Source is the URL where the code is hosted
    #[prost(string, tag = "10")]
    pub source: ::prost::alloc::string::String,
    /// Builder is the docker image used to build the code deterministically, used
    /// for smart contract verification
    #[prost(string, tag = "11")]
    pub builder: ::prost::alloc::string::String,
    /// CodeHash is the SHA256 sum of the code outputted by builder, used for smart
    /// contract verification
    #[prost(bytes = "vec", tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
}
/// MsgStoreAndInstantiateContractResponse defines the response structure
/// for executing a MsgStoreAndInstantiateContract message.
///
/// Since: 0.40
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgStoreAndInstantiateContractResponse")]
pub struct MsgStoreAndInstantiateContractResponse {
    /// Address is the bech32 address of the new contract instance.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgAddCodeUploadParamsAddresses is the
/// MsgAddCodeUploadParamsAddresses request type.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgAddCodeUploadParamsAddresses")]
pub struct MsgAddCodeUploadParamsAddresses {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgAddCodeUploadParamsAddressesResponse defines the response
/// structure for executing a MsgAddCodeUploadParamsAddresses message.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgAddCodeUploadParamsAddressesResponse")]
pub struct MsgAddCodeUploadParamsAddressesResponse {}
/// MsgRemoveCodeUploadParamsAddresses is the
/// MsgRemoveCodeUploadParamsAddresses request type.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgRemoveCodeUploadParamsAddresses")]
pub struct MsgRemoveCodeUploadParamsAddresses {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgRemoveCodeUploadParamsAddressesResponse defines the response
/// structure for executing a MsgRemoveCodeUploadParamsAddresses message.
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgRemoveCodeUploadParamsAddressesResponse")]
pub struct MsgRemoveCodeUploadParamsAddressesResponse {}
/// MsgStoreAndMigrateContract is the MsgStoreAndMigrateContract
/// request type.
///
/// Since: 0.42
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgStoreAndMigrateContract")]
pub struct MsgStoreAndMigrateContract {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission to apply on contract creation, optional
    #[prost(message, optional, tag = "3")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on migration
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// MsgStoreAndMigrateContractResponse defines the response structure
/// for executing a MsgStoreAndMigrateContract message.
///
/// Since: 0.42
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgStoreAndMigrateContractResponse")]
pub struct MsgStoreAndMigrateContractResponse {
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "1")]
    #[serde(alias = "codeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,
    /// Checksum is the sha256 hash of the stored code
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgUpdateContractLabel sets a new label for a smart contract
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUpdateContractLabel")]
pub struct MsgUpdateContractLabel {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// NewLabel string to be set
    #[prost(string, tag = "2")]
    pub new_label: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
/// MsgUpdateContractLabelResponse returns empty data
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
#[proto_message(type_url = "/cosmwasm.wasm.v1.MsgUpdateContractLabelResponse")]
pub struct MsgUpdateContractLabelResponse {}
pub struct WasmQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> WasmQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn contract_info(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryContractInfoResponse, cosmwasm_std::StdError> {
        QueryContractInfoRequest { address }.query(self.querier)
    }
    pub fn contract_history(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryContractHistoryResponse, cosmwasm_std::StdError> {
        QueryContractHistoryRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn contracts_by_code(
        &self,
        code_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryContractsByCodeResponse, cosmwasm_std::StdError> {
        QueryContractsByCodeRequest {
            code_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn all_contract_state(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllContractStateResponse, cosmwasm_std::StdError> {
        QueryAllContractStateRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn raw_contract_state(
        &self,
        address: ::prost::alloc::string::String,
        query_data: ::prost::alloc::vec::Vec<u8>,
    ) -> Result<QueryRawContractStateResponse, cosmwasm_std::StdError> {
        QueryRawContractStateRequest {
            address,
            query_data,
        }
        .query(self.querier)
    }
    pub fn smart_contract_state(
        &self,
        address: ::prost::alloc::string::String,
        query_data: ::prost::alloc::vec::Vec<u8>,
    ) -> Result<QuerySmartContractStateResponse, cosmwasm_std::StdError> {
        QuerySmartContractStateRequest {
            address,
            query_data,
        }
        .query(self.querier)
    }
    pub fn code(&self, code_id: u64) -> Result<QueryCodeResponse, cosmwasm_std::StdError> {
        QueryCodeRequest { code_id }.query(self.querier)
    }
    pub fn codes(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryCodesResponse, cosmwasm_std::StdError> {
        QueryCodesRequest { pagination }.query(self.querier)
    }
    pub fn pinned_codes(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPinnedCodesResponse, cosmwasm_std::StdError> {
        QueryPinnedCodesRequest { pagination }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn contracts_by_creator(
        &self,
        creator_address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryContractsByCreatorResponse, cosmwasm_std::StdError> {
        QueryContractsByCreatorRequest {
            creator_address,
            pagination,
        }
        .query(self.querier)
    }
}
