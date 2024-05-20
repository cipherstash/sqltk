use std::{marker::PhantomData, ops::Deref};

use sqlparser::ast::{Query, Statement};
use sqltk::{
    prelude::{flow, VisitorControlFlow},
    Visitable, Visitor,
};

use crate::{Annotate, Projection, Provenance, ResolutionError, SelectProvenance};

#[derive(Debug, Clone)]
pub struct BuildSelectProvenance<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildSelectProvenance<'ast, State>
where
    State: Annotate<'ast, Statement, Provenance> + Annotate<'ast, Query, Projection>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}


impl<'ast, State> Visitor<'ast> for BuildSelectProvenance<'ast, State>
where
    State: Annotate<'ast, Statement, Provenance> + Annotate<'ast, Query, Projection>,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        if let Some(statement) = node.downcast_ref::<Statement>() {
            match statement {
                Statement::Query(query) => match state.expect_annotation(query.deref()) {
                    Ok(projection) => {
                        state.add_annotation(
                            statement,
                            Provenance::Select(
                                SelectProvenance {
                                    projection: projection.clone(),
                                }
                                .into(),
                            ),
                        );
                        flow::cont(state)
                    }
                    Err(err) => flow::error(err.into()),
                },
                _ => flow::cont(state),
            }
        } else {
            flow::cont(state)
        }
    }
}
