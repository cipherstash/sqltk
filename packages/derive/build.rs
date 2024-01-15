use std::path::PathBuf;

use sqltk_codegen::Codegen;

fn main() {
    let codegen = Codegen::new();

    let dest_file =
        PathBuf::from(&std::env::var("OUT_DIR").unwrap()).join("generated_node_list.json");

    codegen.generate_node_list_file(&dest_file);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
