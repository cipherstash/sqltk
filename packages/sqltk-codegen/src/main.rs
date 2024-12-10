use std::path::PathBuf;

mod codegen;
mod generics;
mod meta;
mod parser;
mod reachability;
mod sqlparser_node_extractor;
mod transformable_trait_impls;
mod visitable_trait_impls;

use codegen::*;

fn main() -> std::io::Result<()> {
    println!(
        "The current directory is {}",
        std::env::current_dir().unwrap().display()
    );

    let codegen = Codegen::new();

    std::fs::create_dir_all(output_path()).unwrap();

    codegen.generate_visitable_impls(&output_path().join("visitable_impls.rs"), None);

    codegen.generate_apply_transform_impls(&output_path().join("transformable_impls.rs"));

    Ok(())
}

fn output_path() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("packages")
        .join("sqltk")
        .join("src")
        .join("generated")
}
