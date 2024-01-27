use std::path::PathBuf;

mod build_helpers;
pub use build_helpers::*;

fn main() {
    let codegen = Codegen::new();

    let node_list_file =
        PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated_node_list.json");

    codegen.generate_node_list_file(&node_list_file );

    let dispatch_table_file =
        PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated_dispatch_table_trait.rs");

    codegen.generate_dispatch_table_trait(&dispatch_table_file);

    let dispatch_table_lookup_impls_file =
        PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated_dispatch_table_lookup_impls.rs");

    codegen.generate_dispatch_table_lookup_impls(&dispatch_table_lookup_impls_file);

    let dispatch_impls_file =
        PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated_dispatch_impls.rs");

    codegen.generate_dispatch_impls(&dispatch_impls_file);

    let ast_node_impls_file =
        PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated_ast_node_impls.rs");

    let reachability_debug_file =
        PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("reachability-debug.txt");

    codegen.generate_ast_node_impls(&ast_node_impls_file, &reachability_debug_file);

    let concrete_node_enum_file =
        PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated_concrete_node_enum.rs");

    codegen.generated_concrete_node_enum(&concrete_node_enum_file);

    let concrete_node_enum_match_macro_file = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("generated_concrete_node_enum_match_macro.rs");

    codegen.generate_concrete_node_enum_match_macro(&concrete_node_enum_match_macro_file);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
