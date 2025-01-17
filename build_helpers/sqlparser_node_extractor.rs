use cargo_metadata::Package;
use parser::SqlParserAstAnalyser;
use std::process::Command;

use super::meta::SqlParserMeta;

use super::*;

pub fn extract(sqlparser_pkg: &Package) -> SqlParserMeta {
    let output = Command::new("cargo")
        .arg("expand")
        .arg("--locked")
        .arg("--frozen")
        .arg("--offline")
        .arg("--features")
        .arg("visitor, bigdecimal")
        .current_dir(
            sqlparser_pkg
                .manifest_path
                .parent()
                .expect("could not get parent directory of sqlparser's Cargo.toml"),
        )
        .output()
        .expect("Failed to execute command");

    let syntax_tree = syn::parse_file(
        &String::from_utf8(output.stdout).expect("Could not parse output of cargo-expand as UTF8"),
    )
    .expect("Failed to parse sqlparser source code");

    SqlParserAstAnalyser::parse_mod(&syntax_tree.items)
}
