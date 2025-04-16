use parser::SqlParserAstAnalyser;
use std::path::Path;
use std::process::Command;

use super::meta::SqlParserMeta;

use super::*;

pub fn extract() -> SqlParserMeta {
    let sqlparser_features = vec!["visitor", "bigdecimal"];

    let sql_parser_dir = Path::new(
        std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str()
    )
        .join("..")
        .join("sqltk-parser");

    // TODO: instead of depending on cargo-expand, just invoke rustc the same
    // way that cargo-expand does and remove a dependency.
    // change to this command: `cargo +nightly rustc -- -Zunstable-options -Zunpretty=expanded`
    let output = Command::new("cargo")
        .arg("expand")
        .arg("--features")
        .arg(sqlparser_features.join(","))
        .current_dir(sql_parser_dir)
        .output()
        .expect("Failed to execute command");

    let syntax_tree = syn::parse_file(
        &String::from_utf8(output.stdout).expect("Could not parse output of cargo-expand as UTF8"),
    )
    .expect("Failed to parse sqlparser source code");

    SqlParserAstAnalyser::parse_mod(&syntax_tree.items)
}
