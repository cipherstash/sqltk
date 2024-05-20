use std::{marker::PhantomData, ops::Deref};

use sqlparser::ast::{Cte, Expr, Query, TableAlias};
use sqltk::{flow, Visitable, Visitor, VisitorControlFlow};

use crate::{
    model::Annotate,
    model::Projection,
    model::ResolutionError,
    model::ScopeOps,
    model::{NamedRelation, Source},
    model::{SqlIdent, UnquotedIdent},
    SchemaOps,
};

#[derive(Debug)]
pub struct ImportFromCte<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for ImportFromCte<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, Source>
        + Annotate<'ast, Query, Projection>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for ImportFromCte<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, Source>
        + Annotate<'ast, Query, Projection>
        + SchemaOps,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
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
                return flow::error(ResolutionError::Unimplemented);
            }

            match state.expect_annotation(query.deref()) {
                Ok(projection) => match state.add_relation(
                    NamedRelation {
                        name: SqlIdent::Unquoted(UnquotedIdent::new(alias.value.clone())).into(),
                        projection: projection.clone(),
                    }
                    .into(),
                ) {
                    Ok(_) => flow::cont(state),
                    Err(err) => flow::error(err),
                },
                Err(err) => flow::error(err.into()),
            }
        } else {
            flow::cont(state)
        }
    }
}
