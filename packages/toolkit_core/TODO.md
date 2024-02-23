# sqltk

[X] Walk Statement type to discover all reachable AST nodes

[ ] Write missing cargo doc

[ ] Improve the README

[ ] Remove dependency on cargo-expand (it just invokes cargo with some specific options)

[ ] Visitor implementation that normalises all identifiers (tables and columns) to a fully-qualified form (might    require schema) knowledge to work in all cases)

[ ] Visitor implementation that tracks provenance of projected columns:

    - Sometimes not statically resolvable due to not having schema information available
    - The visitor should have an associated "resolver" trait such that the user can provide an implementation that has Knowledge of the schema.