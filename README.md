# sqltk

`sqltk` is a toolkit for analysis and transformation of SQL statements, built on top of [`sqlparser`](https://crates.io/crates/sqlparser).

## Features

- A generalised `Visitor` trait and implementations for all `sqlparser` AST node types.

`sqlparser`'s `Visitor` implementation only contains callbacks for a handful of AST node types.

In contrast, `sqltk`'s implementation will invoke `Visitor::enter` and `Visitor::exit` for _all_ `sqlparser` node types.

Additionally, `sqltk` traverses the AST in an order that is useful for semantic analysis  - specifically any node that might be _referred to_ by another node will be visited _before_ a node that might refer to it.

This means your `Visitor` implementations can safely assume that any semantic dependencies of the node being visited have already been visited.

For example, in a `SELECT` statement the `FROM` clause will be visited before the projection or the `WHERE` clause etc.

The analysis that determines AST traversal order happens at compile time (see `packages/sqltk-codegen`).

## Getting started

`sqltk-codegen` requires `cargo-expand` to be installed.

1. Install `cargo-expand`

`$ cargo install cargo-expand`

  > NOTE: `cargo-expand` invokes Rust *nightly* to do its job. Therefore a nightly Rust toolchain must be installed. However, `sqltk`'s generated code does not require a nightly compiler.

2. Add `sqltk` to your Cargo project

`$ cargo add sqltk`

## A tour of the cargo workspace

### sqltk

The main lib crate.  Where all logic & non-generated trait implementations are maintained.

### sqltk-codegen

Analyses `sqlparser` source code and generates:

- A serialized blob of metadata about `sqlparser` AST nodes
- Analyzes the `sqlparser` AST in order to determine an AST traversal order for single-pass semantic analysis workloads.
- Generates the `Visitable` trait implementation for all AST node types
- Generates the `Transformer` trait implementation for all AST node types

## About

`sqltk` is maintained by CipherStash and is a core component of [Proxy](https://cipherstash.com/products/cipherstash-proxy), our encryption-in-use database proxy.