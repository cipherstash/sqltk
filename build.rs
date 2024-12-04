use std::path::PathBuf;

mod build_helpers;
use build_helpers::*;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let codegen = Codegen::new();

    std::fs::create_dir_all(output_path()).unwrap();

    codegen.generate_visitable_impls(
        &output_path().join("visitable_impls.rs"),
        None, // Some(&output_path().join("reachability-debug.txt")),
    );

    codegen.generate_apply_transform_impls(&output_path().join("transformable_impls.rs"));

    Ok(())
}

fn output_path() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR to be present")).join("generated")
}
