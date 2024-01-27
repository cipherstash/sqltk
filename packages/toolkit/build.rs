use std::path::PathBuf;

use sqltk_codegen;
use std::fs;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_dispatch_table_trait.rs"),
        *sqltk_codegen::VISITOR_DISPATCH_TABLE
    )?;

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_dispatch_table_lookup_impls.rs"),
        *sqltk_codegen::VISITOR_DISPATCH_TABLE_LOOKUP_IMPLS
    )?;

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_dispatch_impls.rs"),
        *sqltk_codegen::VISITOR_DISPATCH_IMPLS,
    )?;

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_ast_node_impls.rs"),
        *sqltk_codegen::AST_NODE_IMPLS,
    )?;

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_concrete_node_enum.rs"),
        *sqltk_codegen::CONCRETE_NODE_ENUM,
    )?;

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap())
            .join("generated_concrete_node_enum_match_macro.rs"),
        *sqltk_codegen::CONCRETE_NODE_ENUM_MATCH_MACRO,
    )?;

    Ok(())
}
