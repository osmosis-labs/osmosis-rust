//! Build Osmosis proto files. This build script clones the CosmosSDK and Osmosis version
//! specified in the COSMOS_SDK_REV and OSMOSIS_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use regex::Regex;
use std::{
    env,
    ffi::OsStr,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process,
    sync::atomic::{self, AtomicBool},
};
use syn::{Attribute, File, Ident, LitStr, __private::Span};
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
const OSMOSIS_PROTO_DIR: &str = "../osmosis-std/src/prost/";
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
