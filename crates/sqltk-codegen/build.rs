use std::path::PathBuf;

mod build_helpers;
use build_helpers::*;

fn main() {
    let codegen = Codegen::new();

    std::fs::create_dir_all(PathBuf::from(manifest_dir()).join("generated")).unwrap();

    codegen.generate_visitable_impls(
        &output_path("visitable_impls.rs"),
        &output_path("reachability-debug.txt"),
    );

    codegen.generate_apply_transform_impls(&output_path("apply_transform_impls.rs"));

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=../Cargo.toml");
    println!("cargo:rerun-if-changed=../Cargo.lock");
}
fn manifest_dir() -> String {
    std::env::var("CARGO_MANIFEST_DIR").unwrap()
}

fn output_path(file: &str) -> PathBuf {
    PathBuf::from(manifest_dir()).join("generated").join(file)
}
