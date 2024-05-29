use std::{
    marker::PhantomData,
    ops::{ControlFlow, Deref},
    rc::Rc,
};

use sqlparser::ast::{Expr, Select, SelectItem};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{
    model::{Annotate, Projection, ResolutionError, ScopeOps, SourceItem},
    AnnotateMut, ProjectionColumn, SchemaOps,
};

#[derive(Debug)]
pub struct BuildSelectProjection<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildSelectProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, SourceItem>
        + Annotate<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + AnnotateMut<'ast, Select, Projection>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildSelectProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, SourceItem>
        + Annotate<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + AnnotateMut<'ast, Select, Projection>
        + SchemaOps,
{
    type State = State;
    type Error = ResolutionError;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        if let Some(select) = node.downcast_ref::<Select>() {
            let Select {
                projection: select_items,
                ..
            } = select;

            let result: Result<Vec<_>, _> = select_items
                .iter()
                .map(|item| state.get_annotation(item))
                .collect();

            let result: Result<Vec<Rc<ProjectionColumn>>, _> = result.map(|items| {
                items
                    .iter()
                    .flat_map(|item| item.deref())
                    .map(|item| (*item).clone())
                    .collect::<Vec<_>>()
            });

            match result {
                Ok(columns) => {
                    state.set_annotation(select, Projection::Columns(columns));
                    self.continue_with_state(state)
                }
                Err(err) => self.break_with_error(err.into()),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
