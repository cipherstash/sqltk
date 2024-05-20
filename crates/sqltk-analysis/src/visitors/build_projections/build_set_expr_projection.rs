use std::{marker::PhantomData, ops::Deref, rc::Rc};

use sqlparser::ast::{Expr, Query, Select, SetExpr};
use sqltk::{flow, Visitable, Visitor, VisitorControlFlow};

use crate::{
    model::{Annotate, Projection, ResolutionError, ScopeOps, Source},
    ProjectionColumn, SchemaOps, SourceItem,
};

#[derive(Debug)]
pub struct BuildSetExprProjection<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildSetExprProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, Source>
        + Annotate<'ast, Select, Projection>
        + Annotate<'ast, SetExpr, Projection>
        + Annotate<'ast, Query, Projection>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildSetExprProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, Source>
        + Annotate<'ast, Select, Projection>
        + Annotate<'ast, SetExpr, Projection>
        + Annotate<'ast, Query, Projection>
        + SchemaOps,
{
    type State = State;
    type Error = ResolutionError;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        if let Some(set_expr) = node.downcast_ref::<SetExpr>() {
            match set_expr {
                SetExpr::Select(select) => match state.expect_annotation(select.deref()) {
                    // Simply clone the annotation from the SetExpr to the Query.
                    Ok(projection) => {
                        state.add_annotation(set_expr, projection);
                        flow::cont(state)
                    }
                    Err(err) => flow::error(err.into()),
                },
                SetExpr::Query(query) => match state.expect_annotation(query.deref()) {
                    // Simply clone the annotation from the SetExpr to the Query.
                    Ok(projection) => {
                        state.add_annotation(set_expr, projection);
                        flow::cont(state)
                    }
                    Err(err) => flow::error(err.into()),
                },
                SetExpr::SetOperation {
                    op: _,
                    set_quantifier: _,
                    left,
                    right,
                } => {
                    // In SQL a semantically valid set operation (UNION, INTERSECT
                    // etc) is an operator where the left and right operands are
                    // both subqueries which have projections containing the same
                    // number of columns on each side.  However, the SQL *grammar*
                    // does not enforce that there are the same number of columns.
                    // In the case that the number of columns is different, the
                    // combined projection will have a number of columns equal to
                    // the length of the shortest of left or right. But this is
                    // academic, because Postgres will reject the query when it is
                    // executed.
                    //
                    // Named columns (not just expression) from the right hand side
                    // are ignored. The left hand side wins.
                    let result = state
                        .expect_annotation(left.deref())
                        .and_then(|left| Ok((left, state.expect_annotation(right.deref())?)));

                    match result {
                        Ok((left_projection, right_projection)) => {
                            state.add_annotation(
                                set_expr,
                                Projection::Concatenated(vec![
                                    left_projection.clone(),
                                    right_projection.clone(),
                                ]),
                            );

                            flow::cont(state)
                        }
                        Err(err) => flow::error(err.into()),
                    }
                }
                SetExpr::Values(values) => {
                    let number_of_columns = values
                        .rows
                        .first()
                        .expect("Parse would have failed if there were no rows")
                        .len();

                    let projection_columns = (0..number_of_columns)
                        .map(|_| {
                            Rc::new(ProjectionColumn::new(
                                Rc::new(Source::single(SourceItem::ColumnOfValues)),
                                None,
                            ))
                        })
                        .collect::<Vec<_>>();

                    state.add_annotation(set_expr, Projection::Columns(projection_columns));

                    flow::cont(state)
                }
                SetExpr::Insert(_statement) => {
                    // TODO: get projection from statement.
                    // We might need the concept of an empty projection
                    flow::error(
                        ResolutionError::UnsupportedInsertOrUpdateInSubqueryExpressionPosition(
                            Box::new(set_expr.clone()),
                        )
                    )
                }
                SetExpr::Update(_statement) => flow::error(
                    // TODO: get projection from statement.
                    // We might need the concept of an empty projection
                    ResolutionError::UnsupportedInsertOrUpdateInSubqueryExpressionPosition(
                        Box::new(set_expr.clone()),
                    )
                ),
                SetExpr::Table(_table) => flow::error(
                    // TODO: implement this
                    ResolutionError::UnsupportedTableKeywordInSubqueryExpressionPosition(Box::new(
                        set_expr.clone(),
                    ))
                ),
            }
        } else {
            flow::cont(state)
        }
    }
}
