# sqltk

`sqltk` is a toolkit for semantic analysis and transformation of SQL statements.

It uses the AST & parser from the [`sqlparser`](https://crates.io/crates/sqlparser) crate but replaces `sqlparser`'s `Visit` and `Visitor` traits with its own.

`sqlparser`'s Visitor does not provide callbacks for every AST node type - which means it is not useful for any workload which requires exhaustive and/or artbitrary AST analysis. In contrast, the implementation provided by `sqltk` is *exhaustive* - as in, every node in the `sqlparser` abstract syntax tree (AST) is traversed and _reported on_ to a Visitor.

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

### sqltk

The main crate.  Where all logic & non-generated trait & type definitions are maintained. Generated types and traits are consumed from upstream and re-exported.

### sqltk-codegen

Analyses `sqlparser` source code and generates:

- A serialized blob of metadata about `sqlparser` AST nodes
- Analyzes the `sqlparser` AST in order to determine an AST traversal order for single-pass semantic analysis workloads.
- Generates the `Visitable` trait implementation for all AST node types

Depends on `sqltk_meta` for serialization/derialization of data.
Depends on `sqltk_syn_helpers` for manipulating `syn` types.

### sqltk-analysis

Provides utilities for performing semantic analysis of SQL statements.

## About

`sqltk` is maintained by CipherStash and is a core component of [Tandem](https://cipherstash.com/products/tandem), our encryption-in-use database proxy.
