mod build_helpers;

use build_helpers::codegen::*;
use cargo_metadata::MetadataCommand;
use std::{env, path::PathBuf, process::ExitCode};
use tempfile::Builder;

fn main() -> ExitCode {
    let path = output_path();

    if let Some((sqlparser_path, sqlparser_features)) = resolve_sqlparser_dependency() {
        let codegen = Codegen::new(sqlparser_path, sqlparser_features);

        if !output_path().exists() {
            std::fs::create_dir(&path).unwrap();
        }

        codegen.generate_visitable_impls(&path.join("visitable_impls.rs"), None);
        codegen.generate_transformable_impls(&path.join("transformable_impls.rs"));

        return ExitCode::SUCCESS;
    }

    println!("cargo:warning=failed to resolve sqlparser dependency");
    ExitCode::FAILURE
}

fn output_path() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR to be present")).join("generated")
}

/// Resolves the *concrete* sqlparser version and enabled sqlparser features.
///
/// The concrete version must be precisely known because sqltk generates trait implementations from sqlparser's source,
/// which have to compile successfully.
///
/// This will *not* (necessarily) be the version of specified in sqltk's Cargo.toml - but a version that satisfies
/// some-crate's and sqltk's mutual semver constraints.
///
/// Imagine this dependency tree:
///
/// some-crate
///     sqlparser
///     sqktk
///         sqlparser
///
/// Both some-crate and sqltk depend on sqlparser. This will find the post-cargo-dependency-resolution version of
/// sqlparser that sqltk depends on, which *should* be precisely the same version as what some-crate depends on, so long
/// as they are semver compatible.
fn resolve_sqlparser_dependency() -> Option<(PathBuf, Vec<String>)> {
    // When publishing sqltk we need to jump through some hoops to avoid writing to OUT_DIR
    // otherwise `cargo publish` (or `cargo package`) will produce verification error.
    let metadata = if std::env::var("SQLTK_PUBLISH").is_ok() {
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
        MetadataCommand::new()
            .current_dir(temp_dir.as_ref())
            .other_options(vec!["--offline".into()])
            .exec()
            .expect("Failed to fetch cargo metadata")
    } else {
        MetadataCommand::new()
            .current_dir(env::var("CARGO_MANIFEST_DIR").unwrap())
            .other_options(vec!["--offline".into()])
            .exec()
            .expect("Failed to fetch cargo metadata")
    };

    if let Some(resolve) = metadata.resolve {
        for node in resolve.nodes {
            if let Some(sqlparser_pkg) = metadata
                .packages
                .iter()
                .find(|p| p.id == node.id && p.name == "sqlparser")
            {
                println!("Dependency: {}", sqlparser_pkg.name);
                println!("Enabled Features: {:?}", node.features);
                println!("cargo:rerun-if-changed={}", sqlparser_pkg.manifest_path);
                return Some((
                    sqlparser_pkg.manifest_path.parent().unwrap().into(),
                    node.features.clone(),
                ));
            }
        }
        return None;
    }

    None
}
