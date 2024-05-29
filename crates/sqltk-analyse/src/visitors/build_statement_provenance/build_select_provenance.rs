use std::{
    marker::PhantomData,
    ops::{ControlFlow, Deref},
};

use sqlparser::ast::{Query, Statement};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{Annotate, AnnotateMut, Projection, Provenance, ResolutionError, SelectProvenance};

#[derive(Debug, Clone)]
pub struct BuildSelectProvenance<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildSelectProvenance<'ast, State>
where
    State: AnnotateMut<'ast, Statement, Provenance> + Annotate<'ast, Query, Projection>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildSelectProvenance<'ast, State>
where
    State: AnnotateMut<'ast, Statement, Provenance> + Annotate<'ast, Query, Projection>,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        if let Some(statement) = node.downcast_ref::<Statement>() {
            match statement {
                Statement::Query(query) => match state.get_annotation(query.deref()) {
                    Ok(projection) => {
                        state.set_annotation(
                            statement,
                            Provenance::Select(
                                SelectProvenance {
                                    projection: projection.clone(),
                                }
                                .into(),
                            ),
                        );
                        self.continue_with_state(state)
                    }
                    Err(err) => self.break_with_error(err.into()),
                },
                _ => self.continue_with_state(state),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
