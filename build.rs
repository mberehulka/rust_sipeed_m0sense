extern crate riscv_target;

use std::env;
use std::fs;
use std::path::PathBuf;

const TARGET: &'static str = "riscv32i-unknown-none-elf";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    fs::copy(
        format!("assets/trap_{TARGET}.a"),
        out_dir.join(format!("lib{name}.a"))
    ).unwrap();

    println!("cargo:rustc-link-lib=static={name}");
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-arg=-Tmemory.x");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}