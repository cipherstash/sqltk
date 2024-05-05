use std::rc::Rc;

use sqlparser::ast::{Expr, Insert, Query, SetExpr};
use sqltk::{flow, SpecializedVisitor, VisitorControlFlow};

use crate::{
    model::Annotates,
    model::Projection,
    model::ResolutionError,
    model::ScopeOps,
    model::{CanonicalIdent, SqlIdent},
    model::{NamedRelation, Source},
    node_path::NodePathOps,
    SchemaOps,
};

#[derive(Default, Debug, Clone)]
pub struct PublishInsert;

impl<'ast, State: 'ast> SpecializedVisitor<'ast, Insert, State, ResolutionError> for PublishInsert
where
    State: ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Query, Projection>
        + NodePathOps<'ast>
        + SchemaOps,
{
    // Most nodes that bring relations into scope use on_exit but in
    // Insert's case we need on_enter because:
    //
    // 1. there's no AST node to hang an on_exit from except ObjectName, and
    // ObjectName is used in contexts where it *should not* bring anything in
    // to scope (it is not only used to identify tables).
    //
    // 2. Child nodes of the Insert need to resolve identifiers in the
    // context of the scope, so on_exit would be too late.
    fn enter(
        &self,
        node: &'ast Insert,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        let Insert {
            table_name,
            table_alias,
            ..
        } = node;

        let name = SqlIdent::from(table_name.0.last().unwrap());
        let record_as = match table_alias {
            Some(alias) => SqlIdent::from(alias),
            None => SqlIdent::Canonical(CanonicalIdent::from(
                node.table_name.0.last().unwrap().clone(),
            )),
        };

        let result = state.resolve_relation(&name).or_else(|_| {
            state
                .get_schema()
                .resolve_table(&name)
                .map_err(&ResolutionError::from)
                .and_then(|table| {
                    state.add_relation(
                        NamedRelation {
                            name: record_as.into(),
                            projection: Rc::new(Projection::from(&table)),
                        }
                        .into(),
                    )
                })
        });

        match result {
            Ok(_) => flow::cont(state),
            Err(err) => flow::error(err.into(), state),
        }
    }
}
