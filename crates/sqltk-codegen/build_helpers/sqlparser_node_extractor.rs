use cargo_metadata::{CargoOpt, MetadataCommand};
use std::path::PathBuf;
use std::process::Command;

use super::meta::SqlParserMeta;

use super::*;

pub fn extract(sqlparser_features: Vec<String>) -> SqlParserMeta {
    let sqlparser_features = {
        let mut tmp = sqlparser_features.clone();
        tmp.push("visitor".into());
        tmp.push("bigdecimal".into());
        tmp
    };

    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .features(CargoOpt::AllFeatures)
        .exec()
        .unwrap();

    let sql_parser_dir = PathBuf::from(
        &metadata
            .packages
            .iter()
            .find(|p| p.name.starts_with("sqlparser"))
            .expect("Could not find sqlparser crate")
            .manifest_path
            .parent()
            .unwrap(),
    );

    println!(
        "cargo:rerun-if-changed={}",
        sql_parser_dir.as_os_str().to_str().unwrap()
    );

    // TODO: instead of depending on cargo-expand, just invoke rustc the same
    // way that cargo-expand does and remove a dependency.
    // change to this command: `cargo +nightly rustc -- -Zunstable-options -Zunpretty=expanded`
    //
    let output = Command::new("cargo")
        .arg("expand")
        .arg("--features")
        .arg(sqlparser_features.join(","))
        .current_dir(sql_parser_dir)
        .output()
        .expect("Failed to execute command");

    let syntax_tree = syn::parse_file(
        &String::from_utf8(output.stdout).expect("Could not parse output of cargo-expand as UTF08"),
    )
    .expect("Failed to parse sqlparser source code");

    SqlParserAstAnalyser::parse_mod(&syntax_tree.items)
}
