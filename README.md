# sqlparser-ast-toolkit

This crate is an extension for the [`sqlparser`](https://crates.io/crates/sqlparser) crate to support semantic analysis and transformation of arbitrary SQL.

At its core, `sqlparser-ast-toolkit` provides a replacement [Visitor](https://en.wikipedia.org/wiki/Visitor_design_pattern) implementation that addresses a shortcoming of the implementation provided by `sqlparser`.

`sqlparser`'s Visitor does not provide callbacks for every node type - which means it is not useful for any workload which requires whole-of-AST analysis.

The implementation provided by `sqlparser-ast-tookit` is *exhaustive* - as in, every node in the `sqlparser` abstract syntax tree (AST) is traversed and _reported on_ to a Visitor.

## Designed to support semantic analysis of SQL

The order that AST nodes are ctraversed has been carefully designed to support ease of semantic analysis. In essence, this means that `FROM` clauses, and `JOIN` clauses for example are visited *before* any expressions that could reference them are visited.

This strategy can succinctly stated as "__forward references are visited last__" which means that tables, columns and sub-selects have been resolved prior to any expression that makes use of them.

## Composable Visitor Pipelines

A Visitor implementation should maintain one piece of state to keep code modular, reusable, and easy to test.

But analysing SQL is non-trivial and can require management of a number of pieces of independent state.

To make this manageable, `sqlparser-ast-toolkit` provides the ability to compose multiple Visitors into a *pipeline*.

State flows down the pipeline uni-directionally - and successive Visitors in the pipeline can opt-in to what state they consume from the previous steps.

Any state published by a visitor is made available to subsequent steps.

## Included Visitor implementations

`NodeStack`

Keeps track of the path to the current node. Subsequent visitors can request the path of the current node.

`NodeMetadata<T>`

Associates arbitrary metadata with AST nodes, using a `NodePath` to uniquely identify individual nodes.

`Canonicalize`

Maintains the expanded (fully-qualified) identifiers for all identifier nodes. Centralises identifier comparison logic and makes checking if two different identifiers are refering to the same value easy.

`Transform`

`sqlparser-ast-toolkit` provides no mechanism to mutate the AST during traversal. Instead, this crate provides a means to describe edits as values and to collect those edits during traversal.

The `Transform` visitor can build a new AST by traversing the existing AST and copying it to a new one while applying edits.

This does not require an additional traversal of the AST.

## Getting started

This crate analyses the source of `sqlparser` in order to generate `Node` implementations.

It does this by running `cargo expand` and consuming the output. Note that `cargo-expand` is not a library Rust crate and cannot be installed automatically by Cargo.

1. Install `cargo-expand`

`$ cargo install cargo-expand`

  > NOTE: `cargo-expand` invokes Rust *nightly* to do its job. Therefore a nightly Rust toolchain must be installed. However, `sqlparser-ast-toolkit`'s generated code does not require a nightly compiler.

2. Add `sqlparser-ast-toolkit` to your Cargo project

`$ cargo add sqlparser-ast-tookit`

3. Define a Visitor

```rust
use sqltk::{VisitorDispatch, Visitor};
use sqlparser::ast::Expr;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

#[derive(VisitorDispatch)]
pub struct ExprCounter {
  pub count: usize
}

impl Visitor<Expr> for ExprCounter {
  fn enter(&mut self, expr: &Expr) -> VisitorControlFlow {
    self.count +=1;
    ControlFlow::Continue(Navigation::Visit)
  }

  fn exit(&mut self, expr: &Expr) -> VisitorControlFlow {
    ControlFlow::Continue(Navigation::Visit)
  }
}

let sql = "SELECT (1 + 2) * 3;";
let dialect = GenericDialect {};
let ast = Parser::parse_sql(&dialect, sql).unwrap();

let visitor = ExprCounter { count: 0 };

ast.accept(&visitor);

println!("There were {} Expr nodes in the statement!", visitor.count);
```

## License

## Bugs & contributions

## FAQ

### Q: are there any plans to include the `sqlparser-ast-toolkit` functionality in `sqlparser` itself?

As authors of `sqlparser-ast-toolkit`, we are not opposed to the idea (assuming of course that the `sqlparser` maintainers would consider it).

However, we ([CipherStash](https://cipherstash.com)) depend heavily on this codebase and it's still in its infancy. We would like to retain the freedom of ownership over this functionality until we consider it stable.

Making breaking changes will be more difficult after it is included upstream.

## Misc.

`sqlparser-ast-toolkit` is maintained by CipherStash and is a core component of [Tandem](https://docs.cipherstash.com/getting-started/step3.html), our encryption-in-use database proxy.