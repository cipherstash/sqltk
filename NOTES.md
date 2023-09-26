
// Statement Mapping Logical Steps

1. Canonicalise identifiers

column_name
table_name.column_name
schema_name.table_name.column_name

column_name as foo
table_name.column_name as foo
schema_name.table_name.column_name as foo

# Identify any AST node - how?

- no unique ID on a node
- can't rely on memory address (things move)
- can't rely on Eq
  - AST is a DAG
  - two Expr nodes can be *identical* (and thus compare as Eq) but have different parents

# NodeIdentityVisitor

- path from root (current node being visited has a unique path)

<!-- enum Node {
  Statement(&'ast Statement),
  Expr(&'ast Expr),
} -->

- let stack: Vec<&dyn AstNode>


# Ability to attach metadata to any AST node

MetadataStore

# CanonicaliseVisitor

-> stores canonical identifier whenever it sees identifier in Expr

# Provenance

Identifier "foo" comes from schema.table.column


struct VisitorA;
struct VisitorB;
struct VisitorC;
struct VisitorD;

#[derive(Pipeline)]
#[pipeline(VisitorA => VisitorB => VisitorC => VisitorD)]
struct StatementMapper;

trait Produces {
  type Output;
}

trait Requires<T> {}

trait Init {
  fn init(someting: T) -> Self;
}