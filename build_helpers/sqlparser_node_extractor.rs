use cargo_metadata::Package;
use parser::SqlParserAstAnalyser;
use std::process::Command;

use super::meta::SqlParserMeta;

use super::*;

pub fn extract(sqlparser_pkg: &Package) -> SqlParserMeta {
    let output = Command::new("rustup")
        .env_remove("RUSTC")
        // .env("RUSTUP_TOOLCHAIN", "nightly")
        .arg("run")
        .arg("nightly")
        .arg("cargo")
        .arg("rustc")
        .arg("--profile=check")
        .arg("--features=visitor,bigdecimal")
        .arg("--")
        .arg("-Zunpretty=expanded")
        .current_dir(
            sqlparser_pkg
                .manifest_path
                .parent()
                .expect("could not get parent directory of sqlparser's Cargo.toml"),
        )
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        println!(
            "cargo:message=STDERR: {}",
            String::from_utf8(output.stderr.clone()).unwrap()
        );
    }

    let content = String::from_utf8(output.stdout.clone()).unwrap();
    assert!(!content.trim().is_empty());

    let syntax_tree = syn::parse_file(
        &String::from_utf8(output.stdout).expect("Could not parse output of cargo-expand as UTF8"),
    )
    .expect("Failed to parse sqlparser source code");

    SqlParserAstAnalyser::parse_mod(&syntax_tree.items)
}
