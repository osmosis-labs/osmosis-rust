use std::path::{Path, PathBuf};
use syn::{Attribute, Fields, File, Ident, Item, ItemMod, parse_quote, Type};
use std::collections::HashMap;
use prost_types::ServiceDescriptorProto;
use heck::ToUpperCamelCase;
use regex::Regex;
use syn::__private::quote::__private::TokenStream as TokenStream2;
use std::fs::{create_dir_all, remove_dir_all};
use walkdir::WalkDir;
use std::{fs, io};
use std::ffi::OsStr;
use log::debug;
use heck::ToSnakeCase;
use crate::{format_ident, quote};

pub fn append_attrs(
    src: &Path,
    ancestors: &[String],
    ident: &Ident,
    attrs: &mut Vec<Attribute>,
    query_services: &HashMap<String, ServiceDescriptorProto>,
) {
    let type_url = get_type_url(src, ident);
    let path: Vec<String> = type_url.split('.').map(|s| s.to_string()).collect();
    let type_url = [&path[..(path.len() - 1)], ancestors, &[ident.to_string()]]
        .concat()
        .join(".");
    attrs.append(&mut vec![
        syn::parse_quote! { #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, CosmwasmExt)] },
        syn::parse_quote! { #[proto_message(type_url = #type_url)] },
    ]);

    if let Some(attr) = get_query_attr(src, ident, query_services) {
        attrs.append(&mut vec![attr])
    }
}

fn get_query_attr(
    src: &Path,
    ident: &Ident,
    query_services: &HashMap<String, ServiceDescriptorProto>,
) -> Option<Attribute> {
    let package = src.file_stem().unwrap().to_str().unwrap();
    let service = query_services.get(package);

    let method = service?.method.iter().find(|m| {
        let input_type = (*m).input_type.clone().unwrap();
        let input_type = input_type.split('.').last().unwrap();
        *ident == input_type.to_upper_camel_case()
    });

    let method_name = method?.name.clone().unwrap();
    let response_type = method?.output_type.clone().unwrap();
    let response_type = response_type.split('.').last().unwrap();
    let response_type = format_ident!("{}", response_type.to_upper_camel_case());

    let path = format!("/{}.Query/{}", package, method_name);
    Some(syn::parse_quote! { #[proto_query(path = #path, response_type = #response_type)] })
}

fn get_type_url(src: &Path, ident: &Ident) -> String {
    let type_path = src.file_stem().unwrap().to_str().unwrap();
    format!("/{}.{}", type_path, ident)
}

pub fn prepend<T>(v: &mut Vec<T>, other: &mut Vec<T>) {
    v.splice(0..0, other.drain(..));
}

pub fn recur_append_attrs(
    items: Vec<Item>,
    src: &Path,
    ancestors: &[String],
    query_services: &HashMap<String, ServiceDescriptorProto>,
    nested_mod: bool,
) -> Vec<Item> {
    let mut items = items
        .into_iter()
        .map(|i| match i.clone() {
            Item::Struct(mut s) => {
                append_attrs(src, ancestors, &s.ident, &mut s.attrs, query_services);
                Item::Struct(s)
            }
            Item::Mod(m) => {
                let parent = &m.ident.to_string().to_upper_camel_case();

                Item::Mod(ItemMod {
                    content: m.content.map(|(brace, items)| {
                        (
                            brace,
                            recur_append_attrs(
                                items,
                                src,
                                &[ancestors, &[parent.to_string()]].concat(),
                                query_services,
                                true,
                            ),
                        )
                    }),
                    ..m
                })
            }
            _ => i,
        })
        .collect::<Vec<Item>>();

    let package = src.file_stem().unwrap().to_str().unwrap();
    let re = Regex::new(r"([^.]*)(\.v\d+(beta\d+)?)?$").unwrap();

    let package_stem = re.captures(&package).unwrap().get(1).unwrap().as_str();

    let querier_wrapper_ident = format_ident!("{}Querier", &package_stem.to_upper_camel_case());

    let query_fns = query_services.get(package).map(|service| service.method.iter().map(|method_desc| {
        if nested_mod {
            return quote! {};
        }

        let method_desc = method_desc.clone();

        let name = format_ident!("{}", method_desc.name.unwrap().as_str().to_snake_case());
        let req_type = format_ident!("{}", method_desc.input_type.unwrap().split('.').last().unwrap().to_string().to_upper_camel_case());
        let res_type = format_ident!("{}", method_desc.output_type.unwrap().split('.').last().unwrap().to_string().to_upper_camel_case());

        let req_args = items.clone().into_iter()
            .find_map(|item| match item {
                Item::Struct(s) => {
                    if s.ident == req_type {
                        match s.fields {
                            Fields::Named(fields_named) => {
                                Some(fields_named.named)
                            }
                            _ => None
                        }
                    } else {
                        None
                    }
                }
                _ => None
            });

        let arg_idents = req_args.clone().unwrap().into_iter().map(|arg| arg.ident.unwrap()).collect::<Vec<Ident>>();
        let arg_ty = req_args.clone().unwrap().into_iter().map(|arg| arg.ty).collect::<Vec<Type>>();

        quote! {
           pub fn #name( &self, #(#arg_idents : #arg_ty),* ) -> Result<#res_type, cosmwasm_std::StdError> {
               #req_type { #(#arg_idents),* }.query(self.querier)
           }
        }
    }).collect::<Vec<TokenStream2>>());

    if let Some(query_fns) = query_fns {
        if !nested_mod {
            items.append(&mut vec![
                parse_quote! {
                pub struct #querier_wrapper_ident<'a> {
                    querier: cosmwasm_std::QuerierWrapper<'a, cosmwasm_std::Empty>
                }
            },
                parse_quote! {
                impl<'a> #querier_wrapper_ident<'a> {
                    pub fn new(querier: cosmwasm_std::QuerierWrapper<'a, cosmwasm_std::Empty>) -> Self {
                        Self { querier }
                    }
                    #(#query_fns)*
                }
            },
            ]);
        }
    }

    prepend(
        &mut items,
        &mut vec![syn::parse_quote! {
            use osmosis_std_derive::CosmwasmExt;
        }],
    );
    items
}

pub fn copy_and_transform_generated_files(
    from_dir: &Path,
    to_dir: &Path,
    query_services: HashMap<String, ServiceDescriptorProto>,
) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let to_dir = root.join(to_dir);
    debug!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(&to_dir).unwrap_or_default();
    create_dir_all(&to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_transform(
                e.path(),
                format!("{}/{}", to_dir.display(), &filename),
                &query_services,
            )
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_transform(
    src: &Path,
    dest: impl AsRef<Path>,
    query_services: &HashMap<String, ServiceDescriptorProto>,
) -> io::Result<()> {
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Use `tendermint-proto` proto definitions
        ("(super::)+tendermint", "tendermint_proto"),
        // Feature-gate gRPC client modules
        (
            "/// Generated client implementations.",
            "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
        // Feature-gate gRPC client impls which use `tonic::transport`
        (
            "impl (.+)Client<tonic::transport::Channel>",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl ${1}Client<tonic::transport::Channel>",
        ),
    ];

    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = match fs::read_to_string(src) {
        Ok(c) => c,
        Err(e) => {
            debug!("{:?} – {}, copy_and_patch skipped", src, e);
            return Ok(());
        }
    };

    for &(regex, replacement) in REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    let file = syn::parse_file(&contents);
    if let Ok(file) = file {
        // only transform rust file (skipping `*_COMMIT` file)
        let items = recur_append_attrs(file.items, src, &[], query_services, false);

        contents = prettyplease::unparse(&File { items, ..file });
    }

    fs::write(dest, &*contents)
}

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &[
    "cosmos_proto",
    "gogoproto",
    "google",
    "tendermint",
];
