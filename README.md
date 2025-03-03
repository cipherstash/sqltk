# sqltk

`sqltk` is a toolkit for analysis and transformation of SQL statements, built on top of [`sqlparser`](https://crates.io/crates/sqlparser).

## Features

- A comprehensive `Visitor` trait and implementations for all `sqlparser` AST node types.

- A `Transform` trait for rewriting ASTs (`sqltk` does not provide a `VisitorMut` trait).

### Comprehensive Visitor trait with more useful AST traversal order

`sqlparser`'s `Visitor` implementation only contains callbacks for a handful of AST node types.

In contrast, `sqltk`'s implementation will invoke `Visitor::enter` and `Visitor::exit` for _all_ `sqlparser` node types.

Additionally, `sqltk` traverses the AST in an order that is useful for semantic analysis  - specifically any node that might be _referred to_ by another node will be visited _before_ a node that might refer to it.

This means your `Visitor` implementations can safely assume that any semantic dependencies of the node being visited have already been visited.

For example, in a `SELECT` statement the `FROM` clause will be visited before the projection or the `WHERE` clause etc.

The analysis that determines AST traversal order happens at compile time (see `packages/sqltk-codegen`).

### Transform trait

The `Transform` trait contains a single method imaginitively named `transform`. Which takes a reference to the *original* AST node and an owned clone of the node as arguments. Edits are applied to the owned node and returned in a `Result`.

The reason for this existence of this trait is so that metadata about nodes (from a previous analysis step) which inform the transformation process can be held in the type that implements `Transform`. These will be regular Rust shared references to AST nodes (and therefore read-only). Which would *prevent* mutation of the nodes in-place because Rust will not allow coexistence of `&node` and `&mut node`.

`sqlparser`'s `VisitorMut::visit_mut` method accepts a `&mut node` argument, thus preventing coexistance of references to nodes in another data structure - which rules out the use of some patterns for associating metadata with those nodes.

Transformation begins at the leaf nodes of the AST (AKA depth-first) and ends at the root node.

## Getting started

Add `sqltk` to your Cargo project

`$ cargo add sqltk`

If you plan to hack on `sqltk` itself then you will need to install `cargo-expand` if you plan on running the code generator.

`$ cargo install cargo-expand`

  > NOTE: `cargo-expand` invokes Rust *nightly* to do its job. Therefore a nightly Rust toolchain must be installed. However, `sqltk`'s generated code does not require a nightly compiler.

## `sqltk-codegen`

Analyses `sqlparser` source code and generates:

- Analyzes the `sqlparser` AST in order to determine an AST traversal order for single-pass semantic analysis workloads
- Generates the `Visitable` trait implementations for all AST node types
- Generates the `Transformer` trait implementations for all AST node types
- Generates the `Semantic` trait implementations

To update:

```bash
# Run the code generation
(cd packages/sqltk-codegen && cargo run sqltk-codegen)

# Commit the changes
git add packages/sqltk-codegen/packages/
git commit -m 'updates from running `cargo run sqltk-codegen`'
```

You will need to do this whenever:

- You are updating sqlparser, and
- Any AST handling in sqlparser has changed

## About

`sqltk` is maintained by CipherStash and is a core component of [Proxy](https://cipherstash.com/products/cipherstash-proxy), our encryption-in-use database proxy.
