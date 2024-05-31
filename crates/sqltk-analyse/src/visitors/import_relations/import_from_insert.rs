use std::{marker::PhantomData, ops::ControlFlow, rc::Rc};

use sqlparser::ast::{Expr, Insert, Query, SetExpr};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{
    model::Annotate,
    model::Projection,
    model::ResolutionError,
    model::ScopeOps,
    model::{CanonicalIdent, SqlIdent},
    model::{NamedRelation, ExprSource},
    SchemaOps,
};

#[derive(Debug, Clone)]
pub struct ImportFromInsert<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for ImportFromInsert<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, ExprSource>
        + Annotate<'ast, SetExpr, Projection>
        + Annotate<'ast, Query, Projection>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for ImportFromInsert<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, ExprSource>
        + Annotate<'ast, SetExpr, Projection>
        + Annotate<'ast, Query, Projection>
        + SchemaOps,
{
    type Error = ResolutionError;
    type State = State;

    // Most nodes that bring relations into scope use `exit` but in
    // Insert's case we need to use `enter` because:
    //
    // 1. there's no AST node to hang an `exit` from except ObjectName, and
    // ObjectName is used in contexts where it *should not* bring anything in
    // to scope (it is not only used to identify tables).
    //
    // 2. Child nodes of the Insert need to resolve identifiers in the
    // context of the scope, so on_exit would be too late.
    fn enter<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        if let Some(node) = node.downcast_ref::<Insert>() {
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
                    .map_err(ResolutionError::from)
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
                Ok(_) => self.continue_with_state(state),
                Err(err) => self.break_with_error(err),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
