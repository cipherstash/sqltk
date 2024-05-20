use std::{marker::PhantomData, rc::Rc};

use sqlparser::ast::SelectItem;
use sqltk::{flow, Visitable, Visitor, VisitorControlFlow};

use crate::{
    model::{Annotate, Projection, ResolutionError, ScopeOps},
    ProjectionColumn,
};

// TODO: remove this entire module.

#[derive(Debug)]
pub struct BuildVecOfSelectItemProjection<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildVecOfSelectItemProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + Annotate<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildVecOfSelectItemProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + Annotate<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>,
{
    type State = State;
    type Error = ResolutionError;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        if let Some(items) = node.downcast_ref::<Vec<SelectItem>>() {
            let result = items
                .iter()
                .map(|item| {
                    state
                        .expect_annotation(item)
                        .map(|columns| columns.iter().cloned().collect::<Vec<_>>())
                })
                .collect::<Result<Vec<_>, _>>();

            let result = result
                .map(|all_columns| all_columns.into_iter().flatten().collect::<Vec<_>>())
                .map(|columns| Rc::new(Projection::Columns(columns)))
                .map_err(ResolutionError::from);

            match result {
                Ok(projection) => {
                    state.add_annotation(items, projection);
                    flow::cont(state)
                }
                Err(err) => flow::error(err.into()),
            }
        } else {
            flow::cont(state)
        }
    }
}
