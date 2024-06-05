use std::{
    marker::PhantomData,
    ops::{ControlFlow, Deref},
};

use sqlparser::ast::{Cte, Expr, Query, TableAlias};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{
    model::Annotate,
    model::Projection,
    model::ResolutionError,
    model::ScopeOps,
    model::{ExprSource, NamedRelation},
    model::{SqlIdent, UnquotedIdent},
    SchemaOps,
};

#[derive(Debug)]
pub struct ImportFromCte<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for ImportFromCte<'ast, State>
where
    State:
        ScopeOps + Annotate<'ast, Expr, ExprSource> + Annotate<'ast, Query, Projection> + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for ImportFromCte<'ast, State>
where
    State:
        ScopeOps + Annotate<'ast, Expr, ExprSource> + Annotate<'ast, Query, Projection> + SchemaOps,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        if let Some(node) = node.downcast_ref::<Cte>() {
            let Cte {
                alias:
                    TableAlias {
                        name: alias,
                        columns,
                    },
                query,
                ..
            } = node;

            if !columns.is_empty() {
                return self.break_with_error(ResolutionError::Unimplemented);
            }

            match state.get_annotation(query.deref()) {
                Ok(projection) => match state.add_relation(
                    NamedRelation {
                        name: SqlIdent::Unquoted(UnquotedIdent::new(alias.value.clone())).into(),
                        projection: projection.clone(),
                    }
                    .into(),
                ) {
                    Ok(_) => self.continue_with_state(state),
                    Err(err) => self.break_with_error(err),
                },
                Err(err) => self.break_with_error(err.into()),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
