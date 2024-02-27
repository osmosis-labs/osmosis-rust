/// Allowing unknown fields in the deserialization of this struct for backwards compatibility
#[macro_export]
macro_rules! cw_serde_struct_allow_unknown_fields {
    ($($s:item)*)  => {
        $(
            #[derive(
                ::cosmwasm_schema::serde::Serialize,
                ::cosmwasm_schema::serde::Deserialize,
                ::std::clone::Clone,
                ::std::fmt::Debug,
                ::std::cmp::PartialEq,
                ::cosmwasm_schema::schemars::JsonSchema,
            )]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[serde(crate = "::cosmwasm_schema::serde")]
            #[schemars(crate = "::cosmwasm_schema::schemars")]
            $s
        )*
    };
}
