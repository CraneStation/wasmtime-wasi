extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = Config::new("sandboxed-system-primitives").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=SandboxedSystemPrimitives");

    let bindings_builder = bindgen::Builder::default()
        .header("sandboxed-system-primitives/include/cloudabi_syscalls.h")
        .header("sandboxed-system-primitives/src/posix.h")
        .whitelist_function("wasmtime_ssp_.*")
        .whitelist_function("fd_table_init")
        .whitelist_function("fd_table_insert_existing")
        .whitelist_type("cloudabi_.*")
        .whitelist_type("fd_table")
        .whitelist_var("CLOUDABI_.*");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings_builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("wasmtime_ssp.rs"))
        .expect("Couldn't write bindings!");
}
