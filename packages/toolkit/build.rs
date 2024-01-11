use std::path::PathBuf;

use sqltk_codegen::{
    generate_dispatch_impls, generate_visitable_impls, generated_concrete_node_enum,
    generate_concrete_node_enum_match_macro,
};

fn main() {
    let dispatch_impls_file =
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_dispatch_impls.rs");

    generate_dispatch_impls(&dispatch_impls_file);

    let visitable_impls_file =
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_ast_node_impls.rs");

    generate_visitable_impls(&visitable_impls_file);

    let concrete_node_enum_file =
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_concrete_node_enum.rs");

    generated_concrete_node_enum(&concrete_node_enum_file);

    let concrete_node_enum_match_macro_file = PathBuf::from(&std::env::var("OUT_DIR").unwrap())
        .join("generated_concrete_node_enum_match_macro.rs");

    generate_concrete_node_enum_match_macro(&concrete_node_enum_match_macro_file);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
