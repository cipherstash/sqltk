use std::{
    marker::PhantomData,
    ops::{ControlFlow, Deref},
    rc::Rc,
};

use sqlparser::ast::{Expr, Select, SelectItem};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{
    model::{Annotate, ExprSource, Projection, ResolutionError, ScopeOps},
    AnnotateMut, ColumnWithOptionalAlias, SchemaOps, SelectItemSource,
};

#[derive(Debug)]
pub struct BuildSelectProjection<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildSelectProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, ExprSource>
        + Annotate<'ast, SelectItem, SelectItemSource>
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
        + Annotate<'ast, Expr, ExprSource>
        + Annotate<'ast, SelectItem, SelectItemSource>
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

            match result {
                Ok(sources) => {
                    let mut all_columns: Vec<Rc<ColumnWithOptionalAlias>> = Vec::new();
                    for source in sources {
                        match source.deref() {
                            SelectItemSource::ColumnWithOptionalAlias(column) => {
                                all_columns.push(column.clone())
                            }
                            SelectItemSource::ResolvedWildcard(columns) => {
                                all_columns.extend(columns.clone());
                            }
                        }
                    }
                    state.set_annotation(
                        select,
                        Projection {
                            columns: all_columns,
                        },
                    );
                    self.continue_with_state(state)
                }
                Err(err) => self.break_with_error(err.into()),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
