# sqltk

`sqltk` is a toolkit for analyzing and transforming SQL statements.

It uses the AST & parser from the [`sqlparser`](https://crates.io/crates/sqlparser) crate but replaces `sqlparser`'s `Visit` and `Visitor` traits with its own.

 `sqltk` provides a replacement [Visitor](https://en.wikipedia.org/wiki/Visitor_design_pattern) implementation that addresses shortcomings of the implementation provided by `sqlparser`.

`sqlparser`'s Visitor does not provide callbacks for every AST node type - which means it is not useful for any workload which requires exhaustive AST analysis. In contrast, the implementation provided by `sqltk` is *exhaustive* - as in, every node in the `sqlparser` abstract syntax tree (AST) is traversed and _reported on_ to a Visitor.

Additionally, `sqltk` traverses the AST in an order that is useful for semantic analysis (rather than the "order that AST node fields appear in the Rust source" order used by `sqlparser`). What this means is that any node that can be _referred to_ by another node will be visited _before_ the node that refers to it.

For example, in a `SELECT` statement the `FROM` clause will be visited before the projection or the `WHERE` clause etc.

This means that all of the information required to perform semantic analysis of the currently visited node will be available - avoiding the need for ad-hoc additional passes/traversals of the AST.

## Composable Visitor Pipelines

A Visitor implementation should maintain one piece of state to keep code modular, reusable, and easy to test.

But analysing SQL is non-trivial and can require maintainence of a number of pieces of independent state.

To make this manageable, `sqltk` provides the ability to compose multiple Visitors into a *pipeline*.

State flows from root to leaf on calls to `Visitor::enter` - and conversely from leaf to root on calls to `Visitor::exit`.

Any state published by a visitor is made available to visitors in the pipeline.

## Getting started

This crate analyses the source of `sqlparser` in order to generate `Node` implementations.

It does this by running `cargo expand` and consuming the output. Note that `cargo-expand` is not a library Rust crate and cannot be installed automatically by Cargo.

1. Install `cargo-expand`

`$ cargo install cargo-expand`

  > NOTE: `cargo-expand` invokes Rust *nightly* to do its job. Therefore a nightly Rust toolchain must be installed. However, `sqltk`'s generated code does not require a nightly compiler.

2. Add `sqltk` to your Cargo project

`$ cargo add sqltk`

## About

`sqltk` is maintained by CipherStash and is a core component of [Tandem](https://cipherstash.com/products/tandem), our encryption-in-use database proxy.
