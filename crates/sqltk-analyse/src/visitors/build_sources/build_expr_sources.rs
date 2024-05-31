use std::{
    marker::PhantomData,
    ops::{ControlFlow, Deref},
    rc::Rc,
};

use sqlparser::ast::{
    Expr, Function, FunctionArg, FunctionArgExpr, Ident, ListAggOnOverflow, ObjectName, Query,
};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{Annotate, AnnotateMut, Projection, ResolutionError, ScopeOps, ExprSource, SqlIdent};

#[derive(Debug)]
pub struct BuildExprSources<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>)
where
    State: ScopeOps + AnnotateMut<'ast, Expr, ExprSource> + Annotate<'ast, Query, Projection>;

impl<'ast, State> Default for BuildExprSources<'ast, State>
where
    State: ScopeOps + AnnotateMut<'ast, Expr, ExprSource> + Annotate<'ast, Query, Projection>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> BuildExprSources<'ast, State>
where
    State: ScopeOps + AnnotateMut<'ast, Expr, ExprSource> + Annotate<'ast, Query, Projection>,
{
    fn resolve_ident_and_record_source(
        &self,
        expr: &'ast Expr,
        mut state: State,
        ident: &'ast Ident,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        let resolved = state.resolve_ident(&ident.into());
        match resolved {
            Ok(source) => {
                state.set_annotation(expr, source);
                self.continue_with_state(state)
            }
            Err(err) => self.break_with_error(err),
        }
    }

    fn resolve_compound_ident_and_record_source(
        &self,
        expr: &'ast Expr,
        mut state: State,
        compound_ident: &'ast [Ident],
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        let resolved = state.resolve_compound_ident(
            Vec::from_iter(compound_ident.iter().map(SqlIdent::from)).as_slice(),
        );
        match resolved {
            Ok(source) => {
                state.set_annotation(expr, source);
                self.continue_with_state(state)
            }
            Err(err) => self.break_with_error(err),
        }
    }

    fn resolve_sources(
        &self,
        node: &'ast Expr,
        exprs: &[&'ast Expr],
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        let sources = exprs
            .iter()
            .try_fold(Vec::with_capacity(exprs.len()), |mut acc, expr| {
                state.get_annotation(*expr).map(|source| {
                    acc.push(source);
                    acc
                })
            });

        match sources {
            Ok(sources) => {
                state.set_annotation(node, ExprSource::Multiple(sources));
                self.continue_with_state(state)
            }
            Err(err) => self.break_with_error(err.into()),
        }
    }

    fn resolve_function_args(
        &self,
        node: &'ast Expr,
        mut state: State,
        name: &ObjectName,
        args: &'ast [FunctionArg],
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        let sources: Result<Vec<Rc<ExprSource>>, ResolutionError> = args
            .iter()
            .map(|arg| {
                let arg_expr = match arg {
                    FunctionArg::Named {
                        name: _,
                        arg: arg_expr,
                        operator: _,
                    } => arg_expr,
                    FunctionArg::Unnamed(arg_expr) => arg_expr,
                };

                let result = match arg_expr {
                    FunctionArgExpr::Expr(expr) => {
                        state.get_annotation(expr).map_err(ResolutionError::from)
                    }
                    FunctionArgExpr::QualifiedWildcard(name) => state
                        .resolve_qualified_wildcard(
                            name.0
                                .iter()
                                .map(SqlIdent::from)
                                .collect::<Vec<_>>()
                                .as_slice(),
                        )
                        .map(|columns| Rc::new(ExprSource::ResolvedWildcard(columns.clone()))),
                    FunctionArgExpr::Wildcard => state
                        .resolve_wildcard()
                        .map(|columns| Rc::new(ExprSource::ResolvedWildcard(columns.clone()))),
                };

                result
            })
            .collect();

        match sources {
            Ok(sources) => {
                state.set_annotation(
                    node,
                    ExprSource::FunctionCall {
                        ident: SqlIdent::from(
                            name.0.last().expect(
                                "A sqlparser ObjectName to have at least one identifier part",
                            ),
                        ),
                        arg_sources: sources,
                    },
                );
                self.continue_with_state(state)
            }
            Err(err) => self.break_with_error(err),
        }
    }
}

impl<'ast, State> Visitor<'ast> for BuildExprSources<'ast, State>
where
    State: ScopeOps + AnnotateMut<'ast, Expr, ExprSource> + Annotate<'ast, Query, Projection>,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        if let Some(node) = node.downcast_ref::<Expr>() {
            match node {
                Expr::Identifier(ident) => self.resolve_ident_and_record_source(node, state, ident),
                Expr::CompoundIdentifier(idents) => {
                    self.resolve_compound_ident_and_record_source(node, state, idents)
                }
                Expr::BinaryOp { left, op: _, right } => {
                    self.resolve_sources(node, &[left, right], state)
                }
                Expr::JsonAccess {
                    left,
                    operator: _,
                    right,
                } => self.resolve_sources(node, &[left, right], state),
                Expr::CompositeAccess { expr, key: _ } => {
                    self.resolve_sources(node, &[expr], state)
                }
                Expr::IsFalse(expr) => self.resolve_sources(node, &[expr], state),
                Expr::IsNotFalse(expr) => self.resolve_sources(node, &[expr], state),
                Expr::IsTrue(expr) => self.resolve_sources(node, &[expr], state),
                Expr::IsNotTrue(expr) => self.resolve_sources(node, &[expr], state),
                Expr::IsNull(expr) => self.resolve_sources(node, &[expr], state),
                Expr::IsNotNull(expr) => self.resolve_sources(node, &[expr], state),
                Expr::IsUnknown(expr) => self.resolve_sources(node, &[expr], state),
                Expr::IsNotUnknown(expr) => self.resolve_sources(node, &[expr], state),
                Expr::IsDistinctFrom(expr_0, expr_1) => {
                    self.resolve_sources(node, &[expr_0, expr_1], state)
                }
                Expr::IsNotDistinctFrom(expr_0, expr_1) => {
                    self.resolve_sources(node, &[expr_0, expr_1], state)
                }
                Expr::InList {
                    expr,
                    list,
                    negated: _,
                } => {
                    let mut exprs: Vec<&Expr> = list.iter().collect();
                    exprs.push(expr);
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::InSubquery {
                    expr,
                    subquery: _,
                    negated: _,
                } => self.resolve_sources(node, &[expr], state),
                Expr::InUnnest {
                    expr,
                    array_expr,
                    negated: _,
                } => self.resolve_sources(node, &[expr, array_expr], state),
                Expr::Between {
                    expr,
                    negated: _,
                    low,
                    high,
                } => self.resolve_sources(node, &[expr, low, high], state),
                Expr::Like {
                    negated: _,
                    expr,
                    pattern,
                    escape_char: _,
                } => self.resolve_sources(node, &[expr, pattern], state),
                Expr::ILike {
                    negated: _,
                    expr,
                    pattern,
                    escape_char: _,
                } => self.resolve_sources(node, &[expr, pattern], state),
                Expr::SimilarTo {
                    negated: _,
                    expr,
                    pattern,
                    escape_char: _,
                } => self.resolve_sources(node, &[expr, pattern], state),
                Expr::RLike {
                    negated: _,
                    expr,
                    pattern,
                    regexp: _,
                } => self.resolve_sources(node, &[expr, pattern], state),
                Expr::AnyOp {
                    left,
                    compare_op: _,
                    right,
                } => self.resolve_sources(node, &[left, right], state),
                Expr::AllOp {
                    left,
                    compare_op: _,
                    right,
                } => self.resolve_sources(node, &[left, right], state),
                Expr::UnaryOp { op: _, expr } => self.resolve_sources(node, &[expr], state),
                Expr::Convert {
                    expr,
                    data_type: _,
                    charset: _,
                    target_before_value: _,
                } => self.resolve_sources(node, &[expr], state),
                Expr::Cast {
                    expr,
                    data_type: _,
                    format: _,
                    kind: _,
                } => self.resolve_sources(node, &[expr], state),
                Expr::AtTimeZone {
                    timestamp,
                    time_zone: _,
                } => self.resolve_sources(node, &[timestamp], state),
                Expr::Extract { field: _, expr } => self.resolve_sources(node, &[expr], state),
                Expr::Ceil { expr, field: _ } => self.resolve_sources(node, &[expr], state),
                Expr::Floor { expr, field: _ } => self.resolve_sources(node, &[expr], state),
                Expr::Position { expr, r#in } => self.resolve_sources(node, &[expr, r#in], state),
                Expr::Substring {
                    expr,
                    substring_from,
                    substring_for,
                    special: _,
                } => {
                    let mut exprs: Vec<&Expr> = Vec::new();
                    exprs.push(expr);
                    if let Some(expr) = substring_from.as_ref() {
                        exprs.push(expr.deref())
                    }
                    if let Some(expr) = substring_for.as_ref() {
                        exprs.push(expr.deref())
                    }
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::Trim {
                    expr,
                    trim_where: _,
                    trim_what,
                    trim_characters,
                } => {
                    let mut exprs: Vec<&Expr> = Vec::new();
                    exprs.push(expr);
                    if let Some(expr) = trim_what.as_ref() {
                        exprs.push(expr.deref())
                    }
                    if let Some(multiple_exprs) = trim_characters.as_ref() {
                        exprs.extend(&multiple_exprs[..])
                    }
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::Overlay {
                    expr,
                    overlay_what,
                    overlay_from,
                    overlay_for,
                } => {
                    let mut exprs: Vec<&Expr> = Vec::new();
                    exprs.push(expr);
                    exprs.push(overlay_what);
                    exprs.push(overlay_from);
                    if let Some(expr) = overlay_for.as_ref() {
                        exprs.push(expr)
                    }
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::Collate { expr, collation: _ } => self.resolve_sources(node, &[expr], state),
                Expr::Nested(expr) => self.resolve_sources(node, &[expr], state),
                Expr::Value(value) => {
                    state.set_annotation(node, ExprSource::Value(value.clone()));
                    self.continue_with_state(state)
                }
                Expr::IntroducedString { introducer, value } => {
                    state.set_annotation(
                        node,
                        ExprSource::IntroducedString(introducer.clone(), value.clone()),
                    );
                    self.continue_with_state(state)
                }
                Expr::TypedString { data_type, value } => {
                    state.set_annotation(
                        node,
                        ExprSource::TypedString(data_type.clone(), value.clone()),
                    );
                    self.continue_with_state(state)
                }
                Expr::MapAccess { column, keys } => {
                    let mut exprs: Vec<&Expr> = Vec::new();
                    exprs.push(column);
                    exprs.extend(keys[..].iter().map(|k| &k.key));
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::Function(Function { args, name, .. }) => {
                    self.resolve_function_args(node, state, name, args)
                }
                Expr::AggregateExpressionWithFilter { expr, filter } => {
                    self.resolve_sources(node, &[expr, filter], state)
                }
                Expr::Case {
                    operand,
                    conditions,
                    results,
                    else_result,
                } => {
                    let mut exprs: Vec<&Expr> = Vec::new();
                    if let Some(operand) = operand.as_ref() {
                        exprs.push(operand)
                    }
                    exprs.extend(&conditions[..]);
                    exprs.extend(&results[..]);
                    if let Some(else_result) = else_result.as_ref() {
                        exprs.push(else_result)
                    }
                    self.resolve_sources(node, &exprs[..], state)
                }
                // TODO: figure out how to record this.
                // e.g. SELECT EXISTS (SELECT ...);
                // There are subtle security implications: EXISTS only returns a
                // boolean, so naively it appears not to be leaking data to the
                // caller. But for example `SELECT EXISTS (SELECT true where
                // patient.email = "alice@example.com" and patient.is_alcoholic);`
                // leaks a fact by simply returning true/false.
                //
                // This problem is even more broad than just EXISTS. Values returned
                // in a projection have a Source (the values that they are computed
                // from) but the entire projection also has an implied filter that
                // had to be true for that row to be returned, and that filter
                // itself leaks information and we should track it. It's not a
                // Source - it's something else. I think we're missing a concept.
                //
                // That missing concept is probably: "all of the table-columns that
                // were examined by the filter in the WHERE clause"
                Expr::Exists {
                    subquery: _,
                    negated: _,
                } => self.continue_with_state(state),
                Expr::Subquery(query) => {
                    let result: Result<Rc<ExprSource>, ResolutionError> =
                        match state.get_annotation(query.deref()) {
                            Ok(projection) => Ok(state
                                .set_annotation(node, ExprSource::Projection(projection.clone()))),
                            Err(err) => Err(ResolutionError::ExpectedProjectionAnnotation(err)),
                        };

                    match result {
                        Ok(_) => self.continue_with_state(state),
                        Err(err) => self.break_with_error(err),
                    }
                }
                Expr::ArraySubquery(query) => {
                    let result: Result<Rc<ExprSource>, ResolutionError> =
                        match state.get_annotation(query.deref()) {
                            Ok(projection) => Ok(state
                                .set_annotation(node, ExprSource::Projection(projection.clone()))),
                            Err(err) => Err(ResolutionError::ExpectedProjectionAnnotation(err)),
                        };

                    match result {
                        Ok(_) => self.continue_with_state(state),
                        Err(err) => self.break_with_error(err),
                    }
                }
                Expr::ListAgg(agg) => {
                    let mut exprs: Vec<&Expr> = Vec::new();
                    exprs.push(agg.expr.deref());
                    if let Some(sep) = agg.separator.as_ref() {
                        exprs.push(sep.deref())
                    }
                    if let Some(ListAggOnOverflow::Truncate {
                        filler: Some(filler),
                        with_count: _,
                    }) = agg.on_overflow.as_ref()
                    {
                        exprs.push(filler)
                    }
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::ArrayAgg(agg) => {
                    let mut exprs: Vec<&Expr> = Vec::new();
                    exprs.push(agg.expr.deref());
                    if let Some(order_by) = agg.order_by.as_ref() {
                        exprs.extend(order_by.iter().map(|ob| &ob.expr))
                    }
                    if let Some(limit) = agg.limit.as_ref() {
                        exprs.push(limit.deref())
                    }
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::GroupingSets(grouping_sets) => {
                    let exprs: Vec<&Expr> = grouping_sets.iter().flatten().collect();
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::Cube(cube) => {
                    let exprs: Vec<&Expr> = cube.iter().flatten().collect();
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::Rollup(rollup) => {
                    let exprs: Vec<&Expr> = rollup.iter().flatten().collect();
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::Tuple(exprs) => {
                    self.resolve_sources(node, &exprs.iter().collect::<Vec<_>>()[..], state)
                }
                Expr::Struct { values, fields: _ } => {
                    self.resolve_sources(node, &values.iter().collect::<Vec<_>>()[..], state)
                }
                Expr::Named { expr, name: _ } => self.resolve_sources(node, &[expr], state),
                Expr::ArrayIndex { obj, indexes } => {
                    let mut exprs: Vec<&Expr> = Vec::new();
                    exprs.push(obj);
                    exprs.extend(&indexes[..]);
                    self.resolve_sources(node, &exprs[..], state)
                }
                Expr::Array(array) => {
                    self.resolve_sources(node, &array.elem.iter().collect::<Vec<_>>()[..], state)
                }
                Expr::Interval(interval) => self.resolve_sources(node, &[&interval.value], state),
                // NOTE: this is MySQL specific
                Expr::MatchAgainst { .. } => self.break_with_error(ResolutionError::Unimplemented),
                Expr::Wildcard => {
                    let result: Result<Rc<ExprSource>, ResolutionError> =
                        match state.resolve_wildcard() {
                            Ok(columns) => Ok(state
                                .set_annotation(node, ExprSource::ResolvedWildcard(columns.clone()))),
                            Err(err) => Err(err),
                        };

                    match result {
                        Ok(_) => self.continue_with_state(state),
                        Err(err) => self.break_with_error(err),
                    }
                }
                Expr::QualifiedWildcard(wildcard) => {
                    let result: Result<Rc<ExprSource>, ResolutionError> =
                        match state.resolve_qualified_wildcard(
                            Vec::from_iter(wildcard.0.iter().map(SqlIdent::from)).as_slice(),
                        ) {
                            Ok(columns) => Ok(state
                                .set_annotation(node, ExprSource::ResolvedWildcard(columns.clone()))),
                            Err(err) => Err(err),
                        };

                    match result {
                        Ok(_) => self.continue_with_state(state),
                        Err(err) => self.break_with_error(err),
                    }
                }
                Expr::OuterJoin(expr) => self.resolve_sources(node, &[expr], state),
                Expr::Dictionary(dictionary_field) => self.resolve_sources(
                    node,
                    dictionary_field
                        .iter()
                        .map(|df| df.value.deref())
                        .collect::<Vec<&Expr>>()
                        .as_slice(),
                    state,
                ),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
