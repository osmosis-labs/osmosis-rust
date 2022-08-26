extern crate core;

use std::{env, path::PathBuf, process::Command};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let lib_name = "osmosistesting";

    let out_dir = env::var("OUT_DIR").unwrap();

    let header = format!("{}/lib{}.h", out_dir, lib_name);

    // TODO: remove only exist
    // std::fs::remove_file(format!("{}/{}", out_dir, "libosmosistesting.dylib")).unwrap();
    // std::fs::remove_file(format!("{}/{}", out_dir, "libosmosistesting.h")).unwrap();
    // std::fs::remove_file(format!("{}/{}", out_dir, "libwasmvm_muslc.aarch64.a")).unwrap();

    // std::fs::copy(
    //     manifest_dir.join("lib").join("libwasmvm_muslc.aarch64.a"),
    //     format!("{}/{}", out_dir, "libwasmvm_muslc.aarch64.a"),
    // )
    // .unwrap();
    // build go code as shared library
    let exit_status = Command::new("go")
        .current_dir(manifest_dir.join("go"))
        .arg("build")
        .arg("-buildmode=c-shared")
        .arg("-o")
        .arg(format!("{}/{}", out_dir, "libosmosistesting.dylib"))
        .arg("main.go")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    std::fs::copy(
        format!("{}/{}", out_dir, "libosmosistesting.dylib"),
        "/Users/supanatpotiwarakorn/osmosis/osmosis-rust/target/debug/libosmosistesting.dylib",
    )
    .unwrap();

    if !exit_status.success() {
        panic!("failed to build go code");
    }

    // rerun when go code is updated
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("go").join("main.go").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("go").join("go.mod").display()
    );

    // define lib name
    // println!("cargo:rustc-link-search=dylib={}", out_dir);
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib={}", lib_name);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(dead_code)]")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = manifest_dir.join("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
