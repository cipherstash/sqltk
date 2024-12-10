# sqltk-codegen

Analyses the source of the `sqlparser` crate to gather AST type information and then generate implementations of `sqltk::Visitable` and `sqltk::Transformable`.

The generated code is checked in to `sqltk` because the analysis step depends on `cargo-expand` which complicates the installation process.

## Generating code

The code generation step should be run whenever a new version of `sqlparser` is used and/or the definitions of the `Visitable` or `Transform` traits change.

The code generation will output Rust source code in `../sqltk/src/generated`.

`cargo run sqltk-codegen` will generate the code.