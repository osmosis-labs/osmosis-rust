use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CosmwasmExt, attributes(proto))]
pub fn derive_cosmwasm_ext(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let type_url = get_type_url(&input.attrs);

    // `EncodeError` always indicates that a message failed to encode because the
    // provided buffer had insufficient capacity. Message encoding is otherwise
    // infallible.

    quote! {
        impl #ident {
            pub const TYPE_URL: &'static str = #type_url;
        }

        impl From<#ident> for cosmwasm_std::CosmosMsg {
            fn from(msg: #ident) -> Self {
                let mut bytes = Vec::new();
                prost::Message::encode(&msg, &mut bytes)
                    .expect("Message encoding must be infallible");

                cosmwasm_std::CosmosMsg::<cosmwasm_std::Empty>::Stargate {
                    type_url: #type_url.to_string(),
                    value: cosmwasm_std::Binary(bytes),
                }
            }
        }

        impl TryFrom<cosmwasm_std::Binary> for #ident {
            type Error = cosmwasm_std::StdError;

            fn try_from(binary: cosmwasm_std::Binary) -> Result<Self, Self::Error> {
                use prost::Message;
                Self::decode(&binary[..]).map_err(|e| {
                    cosmwasm_std::StdError::ParseErr {
                        target_type: stringify!(#ident).to_string(),
                        msg: format!(
                            "Unable to decode binary: \n  - base64: {}\n  - bytes array: {:?}\n\n{:?}",
                            binary,
                            binary.to_vec(),
                            e
                        ),
                    }
                })
            }
        }
    }.into()
}

fn get_type_url(attrs: &Vec<syn::Attribute>) -> proc_macro2::TokenStream {
    let proto = get_attr("proto", attrs).and_then(|a| a.parse_meta().ok());

    if let Some(syn::Meta::List(meta)) = proto.clone() {
        match meta.nested[0].clone() {
            syn::NestedMeta::Meta(syn::Meta::NameValue(meta)) => {
                if meta.path.is_ident("type_url") {
                    match meta.lit {
                        syn::Lit::Str(s) => quote!(#s),
                        _ => proto_attr_error(meta.lit),
                    }
                } else {
                    proto_attr_error(meta.path)
                }
            }
            t => proto_attr_error(t),
        }
    } else {
        proto_attr_error(proto)
    }
}

fn get_attr<'a>(attr_ident: &str, attrs: &'a Vec<syn::Attribute>) -> Option<&'a syn::Attribute> {
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == attr_ident {
            return Some(attr);
        }
    }
    None
}

fn proto_attr_error<T: quote::ToTokens>(tokens: T) -> proc_macro2::TokenStream {
    syn::Error::new_spanned(tokens, "expected `proto(type_url = \"...\")`").to_compile_error()
}
