# sqltk

`sqltk` is a toolkit for semantic analysis and transformation of SQL statements.

It uses the AST & parser from the [`sqlparser`](https://crates.io/crates/sqlparser) crate but replaces `sqlparser`'s `Visit` and `Visitor` traits with its own.

`sqlparser`'s Visitor does not provide callbacks for every AST node type - which means it is not useful for any workload which requires exhaustive AST analysis. In contrast, the implementation provided by `sqltk` is *exhaustive* - as in, every node in the `sqlparser` abstract syntax tree (AST) is traversed and _reported on_ to a Visitor.

Additionally, `sqltk` traverses the AST in an order that is useful for semantic analysis  - specifically any node that might be _referred to_ by another node will be visited _before_ a node that might refer to it.

For example, in a `SELECT` statement the `FROM` clause will be visited before the projection or the `WHERE` clause etc.

## Getting started

This crate analyses the source of `sqlparser` in order to generate `Visitable` implementations.

It does this by running `cargo expand` and consuming the output. Note that `cargo-expand` is not a library Rust crate and cannot be installed automatically by Cargo.

1. Install `cargo-expand`

`$ cargo install cargo-expand`

  > NOTE: `cargo-expand` invokes Rust *nightly* to do its job. Therefore a nightly Rust toolchain must be installed. However, `sqltk`'s generated code does not require a nightly compiler.

2. Add `sqltk` to your Cargo project

`$ cargo add sqltk`

## A tour of the cargo workspace

The breakdown into multiple crates is for these main reasons:

1. Limit the impact of full-rebuilds

There is a lot of code generation and expensive analysis of the `sqlparser` crate during the build - if we can avoid that then hacking on this codebase is a much more pleasant experience.

2. Testing derive macros

`sqltk_core` was extracted out of `sqltk` so that `sqltk_derive` could depend on the core and permit `#[derive(VisitorDispatch)]` to be tested, which solves a circular dependency issue.

3. Sharing code between different parts of the build process (keeping things DRY)

### sqltk (packages/toolkit)

The main crate. Publically re-exports the content of `sqltk_core` and `sqltk_derive`.

Does not define any types or traits itself but does contain tests that depend on `#[derive(VisitorDispatch)]`.

### sqltk_core (packages/toolkit_core)

Where all logic & non-generated trait & type definitions are maintained. Generated types and traits are consumed from upstream and re-exported.

### sqltk_derive (packages/derive)

Where the implementation of `#[derive(VisitorDispatch)]` lives.

Depends on `sqltk_core` for testing purposes and `sqltk_meta` to be able to generate `DispatchTable` trait implementations (as part of the `VisitorDispatch` derivation).

Depends on `sqltk_codegen` (for the extracted & serialized `sqlparser` AST type information) and `sqltk_syn_helpers` for some utility functions.

### sqltk_meta (packages/meta)

Defines types that represent information about AST node types (and their fields) from `sqlparser`. Also defines `serde` serializers/deserializers so that the extracted information can be persisted to avoid repeated scanning of the `sqlparser` crate during build time.

### sqltk_syn_helpers (packages/syn_helpers)

Utility functions for manipulating `syn::Type` and `syn::TypePath` values.

### sqltk_codegen (packages/codegen)

Analyses `sqlparser` source code and generates:

- A serialized blob of metadata about `sqlparser` AST nodes
- Analyzes the `sqlparser` AST in order to determine an AST traversal order for single-pass semantic analysis workloads.
- Generates the `Visitable` trait implementation for all AST node types

Depends on `sqltk_meta` for serialization/derialization of data.
Depends on `sqltk_syn_helpers` for manipulating `syn` types.

## About

`sqltk` is maintained by CipherStash and is a core component of [Tandem](https://cipherstash.com/products/tandem), our encryption-in-use database proxy.
