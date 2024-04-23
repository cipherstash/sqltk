use std::path::PathBuf;

use std::fs;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_visitable_impls.rs"),
        *sqltk_codegen::AST_NODE_IMPLS,
    )?;

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_node_enum.rs"),
        *sqltk_codegen::NODE_ENUM,
    )?;

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap())
            .join("generated_node_enum_match_macro.rs"),
        *sqltk_codegen::NODE_ENUM_MATCH_MACRO,
    )?;

    Ok(())
}
