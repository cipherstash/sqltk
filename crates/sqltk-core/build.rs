use std::path::PathBuf;

use std::fs;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_visitable_impls.rs"),
        *sqltk_codegen::VISITABLE_IMPLS,
    )?;

    fs::write(
        PathBuf::from(&std::env::var("OUT_DIR").unwrap())
            .join("generated_apply_transform_impls.rs"),
        *sqltk_codegen::APPLY_TRANSFORM_IMPLS,
    )?;

    Ok(())
}
