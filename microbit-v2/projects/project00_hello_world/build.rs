//! Build script: copy `memory.x` into the build `OUT_DIR` and instruct the
//! linker to search that directory. This is robust when building inside
//! a workspace or when Cargo's working directory differs from the crate root.
//!
//! # How Cargo build scripts work
//!
//! This build script runs *before* compilation and can emit special directives
//! via `println!("cargo:...")` to modify the build process. These are NOT
//! ordinary console messages—Cargo interprets lines starting with `cargo:` as
//! build instructions.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Retrieve the build output directory from Cargo.
    // This is where intermediate build artifacts are placed.
    // Environment variable is set by Cargo for every build script.
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Copy the linker script from the crate root into the build output directory.
    // `include_bytes!("memory.x")` embeds the file content at compile time,
    // ensuring it's available even if the project layout changes or is built
    // from a workspace.
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // ┌─── CARGO BUILD DIRECTIVE 1 ───────────────────────────────────────┐
    // │ cargo:rustc-link-search=<PATH>                                     │
    // │                                                                    │
    // │ Adds <PATH> to the linker's library/script search path.           │
    // │ This tells the linker where to find linker scripts referenced     │
    // │ with `-T<script>`. Without this directive, `-Tmemory.x` would    │
    // │ fail unless memory.x is in the linker's default search paths.    │
    // └────────────────────────────────────────────────────────────────────┘
    println!("cargo:rustc-link-search={}", out.display());

    // ┌─── CARGO BUILD DIRECTIVE 2 ───────────────────────────────────────┐
    // │ cargo:rerun-if-changed=<PATH>                                      │
    // │                                                                    │
    // │ Instructs Cargo to re-run this build script if the specified      │
    // │ file changes. Without this, Cargo might skip the build script     │
    // │ on rebuilds, causing linker script changes to be ignored.         │
    // │ Default: Cargo re-runs on any file change in the project.         │
    // └────────────────────────────────────────────────────────────────────┘
    println!("cargo:rerun-if-changed=memory.x");

    // ┌─── CARGO BUILD DIRECTIVE 3 ───────────────────────────────────────┐
    // │ cargo:rustc-link-arg=<ARG>                                         │
    // │                                                                    │
    // │ Adds <ARG> to the linker command line for this crate.             │
    // │ `-Tmemory.x` tells the linker to use the file `memory.x` as a    │
    // │ linker script, which defines memory regions (FLASH, RAM).         │
    // │ The linker resolves `-Tmemory.x` using the search paths we        │
    // │ added above (cargo:rustc-link-search).                            │
    // └────────────────────────────────────────────────────────────────────┘
    println!("cargo:rustc-link-arg=-Tmemory.x");
}
