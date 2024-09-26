use osmosis_std_derive::CosmwasmExt;
/// ===================== ShareDenomResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.model.v3.ShareDenomResponse")]
pub struct ShareDenomResponse {
    /// share_denom is the share denomination.
    #[prost(string, tag = "1")]
    pub share_denom: ::prost::alloc::string::String,
}
/// ===================== TotalPoolLiquidityResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.model.v3.TotalPoolLiquidityResponse")]
pub struct TotalPoolLiquidityResponse {
    /// total_pool_liquidity is the total liquidity in the pool denominated in
    /// coins.
    #[prost(message, repeated, tag = "1")]
    pub total_pool_liquidity:
        ::prost::alloc::vec::Vec<super::super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// ===================== AssetConfig
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.model.v3.AssetConfig")]
pub struct AssetConfig {
    /// denom is the asset denomination.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// normalization_factor is the normalization factor for the asset.
    #[prost(string, tag = "2")]
    pub normalization_factor: ::prost::alloc::string::String,
}
/// ===================== ListAssetConfigsResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.cosmwasmpool.v1beta1.model.v3.ListAssetConfigsResponse")]
pub struct ListAssetConfigsResponse {
    /// asset_configs is the list of asset configurations.
    #[prost(message, repeated, tag = "1")]
    pub asset_configs: ::prost::alloc::vec::Vec<AssetConfig>,
}
