use osmosis_std_derive::CosmwasmExt;
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.authenticator.Params")]
pub struct Params {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub maximum_unauthenticated_gas: u64,
    #[prost(string, tag = "2")]
    pub cosigner_contract: ::prost::alloc::string::String,
}
/// AccountAuthenticator represents a foundational model for all authenticators.
/// It provides extensibility by allowing concrete types to interpret and
/// validate transactions based on the encapsulated data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.authenticator.AccountAuthenticator")]
pub struct AccountAuthenticator {
    /// ID uniquely identifies the authenticator instance.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// Type specifies the category of the AccountAuthenticator.
    /// This type information is essential for differentiating authenticators
    /// and ensuring precise data retrieval from the storage layer.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Data is a versatile field used in conjunction with the specific type of
    /// account authenticator to facilitate complex authentication processes.
    /// The interpretation of this field is overloaded, enabling multiple
    /// authenticators to utilize it for their respective purposes.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// AuthenticatorData represents a genesis exported account with Authenticators.
/// The address is used as the key, and the account authenticators are stored in
/// the authenticators field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.authenticator.AuthenticatorData")]
pub struct AuthenticatorData {
    /// address is an account address, one address can have many authenticators
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// authenticators are the account's authenticators, these can be multiple
    /// types including SignatureVerificationAuthenticator, AllOfAuthenticators and
    /// CosmWasmAuthenticators.
    #[prost(message, repeated, tag = "2")]
    pub authenticators: ::prost::alloc::vec::Vec<AccountAuthenticator>,
}
/// GenesisState defines the authenticator module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.authenticator.GenesisState")]
pub struct GenesisState {
    /// params define the parameters for the authenticator module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// next_authenticator_id is the next available authenticator ID.
    #[prost(uint64, tag = "2")]
    #[serde(alias = "next_authenticatorID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_authenticator_id: u64,
    /// authenticator_data contains the data for multiple accounts, each with their
    /// authenticators.
    #[prost(message, repeated, tag = "3")]
    pub authenticator_data: ::prost::alloc::vec::Vec<AuthenticatorData>,
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
#[proto_message(type_url = "/osmosis.authenticator.QueryParamsRequest")]
#[proto_query(
    path = "/osmosis.authenticator.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
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
#[proto_message(type_url = "/osmosis.authenticator.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgGetAuthenticatorsRequest defines the Msg/GetAuthenticators request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.authenticator.GetAuthenticatorsRequest")]
#[proto_query(
    path = "/osmosis.authenticator.Query/GetAuthenticators",
    response_type = GetAuthenticatorsResponse
)]
pub struct GetAuthenticatorsRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
/// MsgGetAuthenticatorsResponse defines the Msg/GetAuthenticators response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.authenticator.GetAuthenticatorsResponse")]
pub struct GetAuthenticatorsResponse {
    #[prost(message, repeated, tag = "1")]
    pub account_authenticators: ::prost::alloc::vec::Vec<AccountAuthenticator>,
}
/// MsgAddAuthenticatorRequest defines the Msg/AddAuthenticator request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.authenticator.MsgAddAuthenticator")]
pub struct MsgAddAuthenticator {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgAddAuthenticatorResponse defines the Msg/AddAuthenticator response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.authenticator.MsgAddAuthenticatorResponse")]
pub struct MsgAddAuthenticatorResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// MsgRemoveAuthenticatorRequest defines the Msg/RemoveAuthenticator request
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
#[proto_message(type_url = "/osmosis.authenticator.MsgRemoveAuthenticator")]
pub struct MsgRemoveAuthenticator {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
/// MsgRemoveAuthenticatorResponse defines the Msg/RemoveAuthenticator response
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
#[proto_message(type_url = "/osmosis.authenticator.MsgRemoveAuthenticatorResponse")]
pub struct MsgRemoveAuthenticatorResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// TxExtension allows for additional authenticator-specific data in
/// transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.authenticator.TxExtension")]
pub struct TxExtension {
    /// selected_authenticators holds indices for the chosen authenticators per
    /// message.
    #[prost(int32, repeated, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub selected_authenticators: ::prost::alloc::vec::Vec<i32>,
}
pub struct AuthenticatorQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AuthenticatorQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn get_authenticators(
        &self,
        account: ::prost::alloc::string::String,
    ) -> Result<GetAuthenticatorsResponse, cosmwasm_std::StdError> {
        GetAuthenticatorsRequest { account }.query(self.querier)
    }
}
