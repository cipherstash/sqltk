mod build_helpers;

use build_helpers::codegen::*;
use cargo_metadata::{MetadataCommand, Package};
use std::{path::PathBuf, process::Command};
use tempfile::Builder;

fn main() -> std::io::Result<()> {
    match std::env::var("DOCS_RS") {
        // assume cargo-expand is available on docs.rs.
        // docs.rs does not allow network access in build.rs.
        Ok(_) => {},
        Err(_) => ensure_cargo_expand_is_installed(),
    }

    if let Some(sqlparser_pkg) = locate_sqlparser_dep() {
        let codegen = Codegen::new(sqlparser_pkg);

        if !output_path().exists() {
            std::fs::create_dir(output_path()).unwrap();
        }

        codegen.generate_visitable_impls(&output_path().join("visitable_impls.rs"), None);
        codegen.generate_transformable_impls(&output_path().join("transformable_impls.rs"));
    }

    Ok(())
}

fn output_path() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR to be present")).join("generated")
}

fn ensure_cargo_expand_is_installed() {
    let is_installed = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).contains("cargo-expand"))
        .unwrap_or(false);

    // Install cargo-expand if not installed
    if !is_installed {
        println!("cargo:warning=cargo-expand is not installed. Installing it now...");
        let status = Command::new("cargo")
            .arg("install")
            .arg("cargo-expand")
            .status()
            .expect("Failed to execute cargo install");

        if !status.success() {
            panic!("Failed to install cargo-expand");
        }
    } else {
        println!("cargo:warning=cargo-expand is already installed.");
    }
}

fn locate_sqlparser_dep() -> Option<Package> {
    let temp_dir = Builder::new()
        .prefix("sqltk-source")
        .tempdir_in(std::env::var("OUT_DIR").expect("OUT_DIR not set"))
        .expect("could not make temp dir");

    let src_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let cargo_toml = PathBuf::from(src_dir).join("Cargo.toml");
    let target_file = temp_dir.path().join("Cargo.toml");

    std::fs::copy(&cargo_toml, &target_file).unwrap_or_else(|_| {
        panic!(
            "cannot copy from {} to {}",
            cargo_toml.to_string_lossy(),
            target_file.to_string_lossy()
        )
    });

    // Fetch the metadata of the current project in order to discover where the local copy of sqlparser lives.
    // We run this command in a temporary directory because it always wants to change Cargo.lock which would force use
    // to publish with `--no-verify`. This way, we isolate any changes to the temp dir and they are discarded.
    let metadata = MetadataCommand::new()
        .current_dir(temp_dir.as_ref())
        .exec()
        .expect("Failed to fetch cargo metadata");

    // Iterate over dependencies to find the one you're interested in
    if let Some(sql_parser_pkg) = metadata.packages.iter().find(|p| p.name == "sqlparser") {
        println!("cargo:rerun-if-changed={}", sql_parser_pkg.manifest_path);
        println!(
            "Dependency 'sqlparser' is located at: {}",
            sql_parser_pkg.manifest_path
        );

        return Some(sql_parser_pkg.clone());
    };

    None
}
