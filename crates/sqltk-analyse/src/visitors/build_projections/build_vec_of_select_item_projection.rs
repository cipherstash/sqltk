use std::{marker::PhantomData, ops::{ControlFlow, Deref}, rc::Rc};

use sqlparser::ast::SelectItem;
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{
    model::{Annotate, Projection, ResolutionError, ScopeOps}, AnnotateMut, ColumnWithOptionalAlias, SelectItemSource
};

#[derive(Debug)]
pub struct BuildVecOfSelectItemProjection<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildVecOfSelectItemProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, SelectItem, SelectItemSource>
        + AnnotateMut<'ast, Vec<SelectItem>, Projection>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildVecOfSelectItemProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, SelectItem, SelectItemSource>
        + AnnotateMut<'ast, Vec<SelectItem>, Projection>,
{
    type State = State;
    type Error = ResolutionError;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        if let Some(items) = node.downcast_ref::<Vec<SelectItem>>() {
            let result = items
                .iter()
                .map(|item| {
                    state
                        .get_annotation(item)
                })
                .collect::<Result<Vec<_>, _>>();


            match result {
                Ok(sources) => {
                    let mut all_columns: Vec<Rc<ColumnWithOptionalAlias>> = Vec::new();
                    for source in sources.iter() {
                        match source.deref() {
                            SelectItemSource::ColumnWithOptionalAlias(column) => {
                                all_columns.push(column.clone())
                            }
                            SelectItemSource::ResolvedWildcard(columns) => {
                                all_columns.extend(columns.clone());
                            }
                        }
                    }
                    let projection = Rc::new(Projection { columns: all_columns });
                    state.set_annotation(items, projection);
                    self.continue_with_state(state)
                }
                Err(err) => self.break_with_error(err.into()),
            }

        } else {
            self.continue_with_state(state)
        }
    }
}
