//! Build Osmosis proto files. This build script clones the CosmosSDK and Osmosis version
//! specified in the COSMOS_SDK_REV and OSMOSIS_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process::{self, Command},
};

use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use log::{debug, info};
use prost::Message;
use prost_types::{FileDescriptorSet, ServiceDescriptorProto};
use regex::Regex;
use syn::{__private::quote::{format_ident, quote}, Attribute, Fields, File, Ident, Item, ItemMod, parse_quote, Type};
use syn::__private::TokenStream2;
use walkdir::WalkDir;

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.45.4";

/// The osmosis commit or tag to be cloned and used to build the proto files
const OSMOSIS_REV: &str = "v11.0.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const PROTO_DIR: &str = "../osmosis-std/src/types/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk";
/// Directory where the osmosis submodule is located
const OSMOSIS_DIR: &str = "../../dependencies/osmosis";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &[
    // "cosmos",
    "cosmos_proto",
    "gogoproto",
    "google",
    "tendermint",
];

fn main() {
    pretty_env_logger::init();

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = PROTO_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    let temp_osmosis_dir = tmp_build_dir.join("osmosis");

    fs::create_dir_all(&temp_osmosis_dir).unwrap();

    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--update-deps") {
        update_submodules();
    }

    output_osmosis_version(&temp_osmosis_dir);

    let query_services = compile_osmosis_proto(&temp_osmosis_dir);



    info!("🧪  Embellishing modules to expose nice API for library user...");
    copy_and_patch_generated_files(&temp_osmosis_dir, &proto_dir, query_services);
    generate_mod_file(&proto_dir);

    // format osmosis std
    env::set_current_dir(Path::new("../osmosis-std")).unwrap();
    Command::new("cargo")
        .arg("fmt")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();


    info!("✨  `osmosis-std` is successfully generated!");
}


fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = process::Stdio::inherit();

    let exit_status = process::Command::new("git")
        .args(args)
        .stdout(stdout)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        panic!("git exited with error code: {:?}", exit_status.code());
    }
}

fn update_submodules() {
    let full_path = |p: &str| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p);

    info!("Updating cosmos/cosmos-sdk submodule...");
    run_git(&["submodule", "update", "--init"]);
    run_git(&["-C", full_path(COSMOS_SDK_DIR).to_str().unwrap(), "fetch"]);
    run_git(&[
        "-C",
        full_path(COSMOS_SDK_DIR).to_str().unwrap(),
        "reset",
        "--hard",
        COSMOS_SDK_REV,
    ]);

    info!("Updating osmosis submodule...");
    run_git(&["submodule", "update", "--init"]);
    run_git(&["-C", full_path(OSMOSIS_DIR).to_str().unwrap(), "fetch"]);
    run_git(&[
        "-C",
        full_path(OSMOSIS_DIR).to_str().unwrap(),
        "reset",
        "--hard",
        OSMOSIS_REV,
    ]);
}

fn output_osmosis_version(out_dir: &Path) {
    let path = out_dir.join("OSMOSIS_COMMIT");
    fs::write(path, OSMOSIS_REV).unwrap();
}

fn compile_osmosis_proto(out_dir: &Path) -> HashMap<String, ServiceDescriptorProto> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let osmosis_dir = root.join(OSMOSIS_DIR);
    let sdk_dir = root.join(COSMOS_SDK_DIR);

    let descriptor_file = out_dir.join("descriptors.bin");

    let proto_includes_paths = [
        sdk_dir.join("proto"),
        sdk_dir.join("third_party/proto"),
        osmosis_dir.join("proto"),
    ];

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    let proto_paths = fs::read_dir(osmosis_dir.join("proto/osmosis"))
        .unwrap()
        .map(|d| d.unwrap().path().to_string_lossy().to_string())
        .collect::<Vec<String>>();

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    info!("🧪 Compiling Osmosis' types from protobuf definitions...");

    tonic_build::configure()
        .build_client(false)
        .build_server(false)
        .out_dir(out_dir)
        .extern_path(".google.protobuf.Timestamp", "crate::shim::Timestamp")
        .extern_path(".google.protobuf.Duration", "crate::shim::Duration")
        .extern_path(".google.protobuf.Any", "crate::shim::Any")
        .file_descriptor_set_path(&descriptor_file)
        .compile(&protos, &includes)
        .unwrap();

    let descriptor_bytes = &std::fs::read(descriptor_file).unwrap()[..];

    let descriptor = FileDescriptorSet::decode(descriptor_bytes).unwrap();

    let query_services: HashMap<String, ServiceDescriptorProto> = descriptor
        .file
        .into_iter()
        .filter_map(|f| {
            let service = f
                .service
                .into_iter()
                .find(|s| s.name == Some("Query".to_string()));

            if let Some(service) = service {
                Some((
                    f.package.expect("Missing package name in file descriptor"),
                    service,
                ))
            } else {
                None
            }
        })
        .collect();

    info!("✨  Osmosis' types from protobuf definitions is compiled successfully!");

    query_services
}

/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}

fn generate_mod_file(for_dir: &Path) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let types_dir = root.join(for_dir);

    let paths = fs::read_dir(&types_dir)
        .expect("[error] Unable to read dir")
        .filter_map(|d| {
            let f = d.expect("[error] Unable to get dir entry");
            if f.path().extension() == Some(OsStr::new("rs")) {
                f.path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            } else {
                None
            }
        })
        .map(|s| s.split('.').map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    paths
        .iter()
        .for_each(|p| fs::create_dir_all(for_dir.join(p[..(p.len() - 1)].join("/"))).unwrap());

    recur_gen_mod(&types_dir, &types_dir, paths, "");
}

fn recur_gen_mod(for_dir: &Path, start_dir: &Path, paths: Vec<Vec<String>>, include_file: &str) {
    let uniq_keys = paths
        .iter()
        .filter_map(|p| (*p).get(0))
        .map(|s| s.to_owned())
        .unique()
        .sorted()
        .collect::<Vec<String>>();

    // base case
    if uniq_keys.is_empty() {
        let from = start_dir.join(format!("{}.rs", include_file.replace('/', ".")));
        let to = for_dir
            .parent()
            .unwrap()
            .join(format!("{}.rs", include_file.split('.').last().unwrap()));
        fs::rename(from, to).unwrap();
    } else {
        let ts = uniq_keys.iter().map(|k| {
            let module = format_ident!("{}", k);
            quote! { pub mod #module; }
        });

        let additional_mod_content = if paths.iter().any(|p| p.is_empty()) && paths.len() > 1 {
            let src_file = start_dir.join(format!("{}.rs", include_file));
            let tk = fs::read_to_string(src_file.clone())
                .unwrap()
                .parse::<syn::__private::TokenStream2>()
                .unwrap();

            fs::remove_file(src_file).unwrap();

            tk
        } else {
            quote!()
        };

        create_mod_rs(
            quote! {
                #(#ts)*

                #additional_mod_content
            },
            for_dir,
        );

        for k in uniq_keys {
            let paths: Vec<Vec<String>> = paths
                .iter()
                // only if head = k
                .filter(|p| (**p).get(0) == Some(&k))
                // get tail
                .map(|p| p.split_at(1).1.to_vec())
                .collect();
            let include_file = if include_file.is_empty() {
                k.clone()
            } else {
                format!("{include_file}.{k}")
            };

            recur_gen_mod(
                &for_dir.join(k.clone()),
                start_dir,
                paths.clone(),
                &include_file,
            );
        }
    }
}

fn copy_and_patch_generated_files(
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
            copy_and_patch(
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

fn copy_and_patch(
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

fn recur_append_attrs(
    items: Vec<syn::Item>,
    src: &Path,
    ancestors: &[String],
    query_services: &HashMap<String, ServiceDescriptorProto>,
    nested_mod: bool,
) -> Vec<syn::Item> {
    let mut items = items
        .into_iter()
        .map(|i| match i.clone() {
            syn::Item::Struct(mut s) => {
                append_attrs(src, ancestors, &s.ident, &mut s.attrs, query_services);
                syn::Item::Struct(s)
            }
            syn::Item::Mod(m) => {
                let parent = &m.ident.to_string().to_upper_camel_case();

                syn::Item::Mod(ItemMod {
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
        .collect::<Vec<syn::Item>>();

    let package = src.file_stem().unwrap().to_str().unwrap();
    let re = Regex::new(r"([^\.]*)(\.v\d+(beta\d+)?)?$").unwrap();

    let package_stem = re.captures(&package).unwrap().get(1).unwrap().as_str();

    let querier_wrapper_ident =
        format_ident!("{}Querier", &package_stem.to_upper_camel_case());

    let query_fns = query_services.get(package).map(|service| service.method.iter().map(|method_desc| {
        if nested_mod {
            return quote! {}
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
            },
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

fn append_attrs(
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

fn prepend<T>(v: &mut Vec<T>, other: &mut Vec<T>) {
    v.splice(0..0, other.drain(..));
}

fn create_mod_rs(ts: syn::__private::TokenStream2, path: &Path) {
    let file = syn::parse_file(ts.to_string().as_str())
        .expect("[error] Unable to parse generated content as file while genrating mod.rs");

    let write = fs::write(path.join("mod.rs"), prettyplease::unparse(&file));

    if let Err(e) = write {
        panic!("[error] Error while generating mod.rs: {}", e);
    }
}


// pass file descriptor in and have fn to get query service from it
// with file descriptor we can have real detail about type paths
// now, type that contains `ID` will not be serialized properly