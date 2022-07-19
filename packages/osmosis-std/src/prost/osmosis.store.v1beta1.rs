#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(message, repeated, tag = "1")]
    pub children: ::prost::alloc::vec::Vec<Child>,
}
impl crate::cosmwasm::ToCosmosMsg for Node {
    const TYPE_URL: &'static str = "/osmosis.store.v1beta1.Node";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Child {
    #[prost(bytes = "vec", tag = "1")]
    pub index: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub accumulation: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for Child {
    const TYPE_URL: &'static str = "/osmosis.store.v1beta1.Child";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Leaf {
    #[prost(message, optional, tag = "1")]
    pub leaf: ::core::option::Option<Child>,
}
impl crate::cosmwasm::ToCosmosMsg for Leaf {
    const TYPE_URL: &'static str = "/osmosis.store.v1beta1.Leaf";
}
