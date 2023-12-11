use osmosis_std_derive::CosmwasmExt;
/// EventSend is emitted on Msg/Send
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.EventSend")]
pub struct EventSend {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the nft
    #[prost(string, tag = "2")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// sender is the address of the owner of nft
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// receiver is the receiver address of nft
    #[prost(string, tag = "4")]
    pub receiver: ::prost::alloc::string::String,
}
/// EventMint is emitted on Mint
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.EventMint")]
pub struct EventMint {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the nft
    #[prost(string, tag = "2")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// owner is the owner address of the nft
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// EventBurn is emitted on Burn
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.EventBurn")]
pub struct EventBurn {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the nft
    #[prost(string, tag = "2")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// owner is the owner address of the nft
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// Class defines the class of the nft type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.Class")]
pub struct Class {
    /// id defines the unique identifier of the NFT classification, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the NFT classification. Optional
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// symbol is an abbreviated name for nft classification. Optional
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    /// description is a brief description of nft classification. Optional
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// uri for the class metadata stored off chain. It can define schema for Class and NFT `Data` attributes. Optional
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// uri_hash is a hash of the document pointed by uri. Optional
    #[prost(string, tag = "6")]
    pub uri_hash: ::prost::alloc::string::String,
    /// data is the app specific metadata of the NFT class. Optional
    #[prost(message, optional, tag = "7")]
    pub data: ::core::option::Option<crate::shim::Any>,
}
/// NFT defines the NFT.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.NFT")]
pub struct Nft {
    /// class_id associated with the NFT, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the NFT
    #[prost(string, tag = "2")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// uri for the NFT metadata stored off chain
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// uri_hash is a hash of the document pointed by uri
    #[prost(string, tag = "4")]
    pub uri_hash: ::prost::alloc::string::String,
    /// data is an app specific data of the NFT. Optional
    #[prost(message, optional, tag = "10")]
    pub data: ::core::option::Option<crate::shim::Any>,
}
/// GenesisState defines the nft module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.GenesisState")]
pub struct GenesisState {
    /// class defines the class of the nft type.
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
    /// entry defines all nft owned by a person.
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<Entry>,
}
/// Entry Defines all nft owned by a person
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.Entry")]
pub struct Entry {
    /// owner is the owner address of the following nft
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// nfts is a group of nfts of the same owner
    #[prost(message, repeated, tag = "2")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryBalanceRequest")]
#[proto_query(
    path = "/cosmos.nft.v1beta1.Query/Balance",
    response_type = QueryBalanceResponse
)]
pub struct QueryBalanceRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// owner is the owner address of the nft
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryBalanceResponse")]
pub struct QueryBalanceResponse {
    /// amount is the number of all NFTs of a given class owned by the owner
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub amount: u64,
}
/// QueryOwnerRequest is the request type for the Query/Owner RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryOwnerRequest")]
#[proto_query(
    path = "/cosmos.nft.v1beta1.Query/Owner",
    response_type = QueryOwnerResponse
)]
pub struct QueryOwnerRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the NFT
    #[prost(string, tag = "2")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
/// QueryOwnerResponse is the response type for the Query/Owner RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryOwnerResponse")]
pub struct QueryOwnerResponse {
    /// owner is the owner address of the nft
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// QuerySupplyRequest is the request type for the Query/Supply RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QuerySupplyRequest")]
#[proto_query(
    path = "/cosmos.nft.v1beta1.Query/Supply",
    response_type = QuerySupplyResponse
)]
pub struct QuerySupplyRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
}
/// QuerySupplyResponse is the response type for the Query/Supply RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QuerySupplyResponse")]
pub struct QuerySupplyResponse {
    /// amount is the number of all NFTs from the given class
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub amount: u64,
}
/// QueryNFTstRequest is the request type for the Query/NFTs RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryNFTsRequest")]
#[proto_query(
    path = "/cosmos.nft.v1beta1.Query/NFTs",
    response_type = QueryNfTsResponse
)]
pub struct QueryNfTsRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// owner is the owner address of the nft
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryNFTsResponse is the response type for the Query/NFTs RPC methods
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryNFTsResponse")]
pub struct QueryNfTsResponse {
    /// NFT defines the NFT
    #[prost(message, repeated, tag = "1")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryNFTRequest is the request type for the Query/NFT RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryNFTRequest")]
#[proto_query(path = "/cosmos.nft.v1beta1.Query/NFT", response_type = QueryNftResponse)]
pub struct QueryNftRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the NFT
    #[prost(string, tag = "2")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
/// QueryNFTResponse is the response type for the Query/NFT RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryNFTResponse")]
pub struct QueryNftResponse {
    /// owner is the owner address of the nft
    #[prost(message, optional, tag = "1")]
    pub nft: ::core::option::Option<Nft>,
}
/// QueryClassRequest is the request type for the Query/Class RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryClassRequest")]
#[proto_query(
    path = "/cosmos.nft.v1beta1.Query/Class",
    response_type = QueryClassResponse
)]
pub struct QueryClassRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
}
/// QueryClassResponse is the response type for the Query/Class RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryClassResponse")]
pub struct QueryClassResponse {
    /// class defines the class of the nft type.
    #[prost(message, optional, tag = "1")]
    pub class: ::core::option::Option<Class>,
}
/// QueryClassesRequest is the request type for the Query/Classes RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryClassesRequest")]
#[proto_query(
    path = "/cosmos.nft.v1beta1.Query/Classes",
    response_type = QueryClassesResponse
)]
pub struct QueryClassesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryClassesResponse is the response type for the Query/Classes RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.QueryClassesResponse")]
pub struct QueryClassesResponse {
    /// class defines the class of the nft type.
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgSend represents a message to send a nft from one account to another account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.MsgSend")]
pub struct MsgSend {
    /// class_id defines the unique identifier of the nft classification, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    #[serde(alias = "classID")]
    pub class_id: ::prost::alloc::string::String,
    /// id defines the unique identification of nft
    #[prost(string, tag = "2")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// sender is the address of the owner of nft
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// receiver is the receiver address of nft
    #[prost(string, tag = "4")]
    pub receiver: ::prost::alloc::string::String,
}
/// MsgSendResponse defines the Msg/Send response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.nft.v1beta1.MsgSendResponse")]
pub struct MsgSendResponse {}
pub struct NftQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> NftQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn balance(
        &self,
        class_id: ::prost::alloc::string::String,
        owner: ::prost::alloc::string::String,
    ) -> Result<QueryBalanceResponse, cosmwasm_std::StdError> {
        QueryBalanceRequest { class_id, owner }.query(self.querier)
    }
    pub fn owner(
        &self,
        class_id: ::prost::alloc::string::String,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryOwnerResponse, cosmwasm_std::StdError> {
        QueryOwnerRequest { class_id, id }.query(self.querier)
    }
    pub fn supply(
        &self,
        class_id: ::prost::alloc::string::String,
    ) -> Result<QuerySupplyResponse, cosmwasm_std::StdError> {
        QuerySupplyRequest { class_id }.query(self.querier)
    }
    pub fn nf_ts(
        &self,
        class_id: ::prost::alloc::string::String,
        owner: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryNfTsResponse, cosmwasm_std::StdError> {
        QueryNfTsRequest {
            class_id,
            owner,
            pagination,
        }
        .query(self.querier)
    }
    pub fn nft(
        &self,
        class_id: ::prost::alloc::string::String,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryNftResponse, cosmwasm_std::StdError> {
        QueryNftRequest { class_id, id }.query(self.querier)
    }
    pub fn class(
        &self,
        class_id: ::prost::alloc::string::String,
    ) -> Result<QueryClassResponse, cosmwasm_std::StdError> {
        QueryClassRequest { class_id }.query(self.querier)
    }
    pub fn classes(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryClassesResponse, cosmwasm_std::StdError> {
        QueryClassesRequest { pagination }.query(self.querier)
    }
}
