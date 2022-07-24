//! Build Osmosis proto files. This build script clones the CosmosSDK and Osmosis version
//! specified in the COSMOS_SDK_REV and OSMOSIS_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use itertools::Itertools;
use regex::Regex;
use std::{
    env,
    ffi::OsStr,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process::{self, Command},
    sync::atomic::{self, AtomicBool},
};
use syn::{
    Attribute, File, Ident, LitStr,
    __private::{
        quote::{format_ident, quote},
        Span,
    },
};
use walkdir::WalkDir;

/// Suppress log messages
// TODO(tarcieri): use a logger for this
static QUIET: AtomicBool = AtomicBool::new(false);

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.45.4";

/// The osmosis commit or tag to be cloned and used to build the proto files
const OSMOSIS_REV: &str = "v10.0.1";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OSMOSIS_PROTO_DIR: &str = "../osmosis-std/src/types/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk";
/// Directory where the osmosis submodule is located
const OSMOSIS_DIR: &str = "../../dependencies/osmosis";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &[
    "cosmos",
    "cosmos_proto",
    "gogoproto",
    "google",
    "tendermint",
];

/// Log info to the console (if `QUIET` is disabled)
// TODO(tarcieri): use a logger for this
macro_rules! info {
    ($msg:expr) => {
        if !is_quiet() {
            println!("[info] {}", $msg)
        }
    };
    ($fmt:expr, $($arg:tt)+) => {
        info!(&format!($fmt, $($arg)+))
    };
}

fn main() {
    if is_github() {
        set_quiet();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = OSMOSIS_PROTO_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    let temp_osmosis_dir = tmp_build_dir.join("osmosis");

    fs::create_dir_all(&temp_osmosis_dir).unwrap();

    update_submodules();
    output_osmosis_version(&temp_osmosis_dir);
    compile_osmosis_proto(&temp_osmosis_dir);

    copy_generated_files(&temp_osmosis_dir, &proto_dir);
    generate_mod_file(&proto_dir);

    // format osmosis std
    env::set_current_dir(Path::new("../osmosis-std")).unwrap();
    Command::new("cargo")
        .arg("fmt")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if is_github() {
        println!(
            "Rebuild protos with proto-build (osmosis rev: {})",
            OSMOSIS_REV
        );
    }
}

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

/// Parse `--github` flag passed to `proto-build` on the eponymous GitHub Actions job.
/// Disables `info`-level log messages, instead outputting only a commit message.
fn is_github() -> bool {
    env::args().any(|arg| arg == "--github")
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

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

fn compile_osmosis_proto(out_dir: &Path) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let osmosis_dir = root.join(OSMOSIS_DIR);
    let sdk_dir = root.join(COSMOS_SDK_DIR);

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

    // Compile all proto client for GRPC services
    info!("Compiling wasmd proto clients for GRPC services!");
    tonic_build::configure()
        .build_client(false)
        .build_server(false)
        .out_dir(out_dir)
        .extern_path(".tendermint", "::tendermint_proto")
        .extern_path(".cosmos", "cosmos_sdk_proto::cosmos")
        .compile(&protos, &includes)
        .unwrap();

    info!("=> Done!");
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

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let to_dir = root.join(to_dir);
    info!("Copying generated files into '{}'...", to_dir.display());

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
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
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

fn copy_and_patch(src: &Path, dest: impl AsRef<Path>) -> io::Result<()> {
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

    let mut contents = fs::read_to_string(src)?;

    for &(regex, replacement) in REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    let file = syn::parse_file(&contents);
    if let Ok(file) = file {
        // only transform rust file (skipping `*_COMMIT` file)
        let mut items = file
            .items
            .into_iter()
            .map(|i| match i.clone() {
                syn::Item::Struct(mut s) => {
                    append_attrs(src, &s.ident, &mut s.attrs);
                    syn::Item::Struct(s)
                }
                _ => i,
            })
            .collect::<Vec<syn::Item>>();

        prepend(
            &mut items,
            &mut vec![syn::parse_quote! {
                use osmosis_std_derive::CosmwasmExt;
            }],
        );

        contents = prettyplease::unparse(&File { items, ..file });
    }

    fs::write(dest, &*contents)
}

fn append_attrs(src: &Path, ident: &Ident, attrs: &mut Vec<Attribute>) {
    let type_url = get_type_url(src, ident);
    attrs.append(&mut vec![
        syn::parse_quote! { #[derive(CosmwasmExt)] },
        syn::parse_quote! { #[proto(type_url = #type_url)] },
    ]);
}

fn get_type_url(src: &Path, ident: &Ident) -> LitStr {
    let type_path = src.file_stem().unwrap().to_str().unwrap();
    let type_url = LitStr::new(
        format!("/{}.{}", type_path, ident).as_str(),
        Span::call_site(),
    );
    type_url
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
