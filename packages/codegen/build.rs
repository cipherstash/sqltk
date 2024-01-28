use std::path::PathBuf;

mod build_helpers;
use build_helpers::*;

fn main() {
    let codegen = Codegen::new();

    std::fs::create_dir_all(PathBuf::from(manifest_dir()).join("generated")).unwrap();

    codegen.generate_node_list_file(&output_path("node_list.json"));
    codegen.generate_dispatch_table_trait(&output_path("dispatch_table_trait.rs"));
    codegen.generate_dispatch_table_lookup_impls(&output_path("dispatch_table_lookup_impls.rs"));
    codegen.generate_dispatch_impls(&output_path("dispatch_impls.rs"));
    codegen.generate_ast_node_impls(
        &output_path("ast_node_impls.rs"),
        &output_path("reachability-debug.txt"),
    );
    codegen.generated_concrete_node_enum(&output_path("concrete_node_enum.rs"));
    codegen
        .generate_concrete_node_enum_match_macro(&output_path("concrete_node_enum_match_macro.rs"));

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
}
fn manifest_dir() -> String {
    std::env::var("CARGO_MANIFEST_DIR").unwrap()
}

fn output_path(file: &str) -> PathBuf {
    PathBuf::from(manifest_dir()).join("generated").join(file)
}
