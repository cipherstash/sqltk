# sqltk

[X] Walk Statement type to discover all reachable AST nodes

[ ] Specify more useful visit order for semantic analysis:
    E.G. it would be more useful to visit tables & joins etc before a projection in  a SELECT because
    the projection depends on information from relations.
    Would defining a partial order on node types be sufficient for this?

[ ] Remove dependency on cargo-expand (it just invokes cargo with some specific options)

# Next

[ ] Visitor implementation that keeps track of the "stack" (path of current node)

[ ] Visitor implementation that normalises all identifiers (tables and columns) to a fully-qualified form (might    require schema) knowledge to work in all cases)

[ ] Visitor implementation that tracks provenance of projected columns:

    - Sometimes not statically resolvable due to not having schema information available
    - The visitor should have an associated "resolver" trait such that the user can provide an implementation that has Knowledge of the schema.

[ ] Mechanism for attaching arbitrary metadata to nodes (use path as node identifier?)

[ ] Composable Visitors that can share state (test visitors in isolation but they can be fused to do their work in a single pass)

[ ] Documentation

// https://blog.weiznich.de/blog/eurorust-non-overlapping-contains-for-hlists/