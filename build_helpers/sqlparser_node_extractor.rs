use parser::SqlParserAstAnalyser;
use std::path::PathBuf;
use std::process::Command;

use super::meta::SqlParserMeta;

use super::*;

pub fn extract(sqlparser_path: &PathBuf, sqlparser_features: &[String]) -> SqlParserMeta {
    let output = Command::new("rustup")
        .env_remove("RUSTC")
        // .env("RUSTUP_TOOLCHAIN", "nightly")
        .arg("run")
        .arg("nightly")
        .arg("cargo")
        .arg("rustc")
        .arg("--profile=check")
        .arg("--features=visitor,bigdecimal")
        .arg(format!("--features={}", sqlparser_features.join(",")))
        .arg("--")
        .arg("-Zunpretty=expanded")
        .current_dir(sqlparser_path)
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
        &String::from_utf8(output.stdout).expect("Could not parse output of macro-expanded Rust code"),
    )
    .expect("Failed to parse sqlparser source code");

    SqlParserAstAnalyser::parse_mod(&syntax_tree.items)
}
