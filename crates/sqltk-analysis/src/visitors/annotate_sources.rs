//! Traits and types used used to annotate [`Expr`] nodes with their [`Source`].s

use core::marker::PhantomData;
use core::ops::Deref;
use std::rc::Rc;

use crate::annotations::Annotates;
use crate::lexical_scope::LexicalScopeOps;
use crate::node_path::NodePathOps;
use crate::projection_annotation::{Projection, ProjectionAnnotation};
use crate::resolution_error::ResolutionError;
use crate::source_annotation::SourceAnnotationItem;

use sqltk::prelude::{Ident, Query};
use sqltk::{flow, VisitorControlFlow};
use sqltk::{Visitable, Visitor};

use sqlparser::ast::{Expr, Function, ListAggOnOverflow, Select, SelectItem, SetExpr};
use unicase::UniCase;

use crate::model::source_annotation::SourceAnnotation;

/// Type that provides functions for building [`Visitor`] implementations that
/// can annotate [`Expr`] and [`SelectItem`] nodes with their [`Source`].
pub struct AnnotateSource<'ast, State: 'ast>(PhantomData<&'ast State>);

impl<'ast, State: 'ast> AnnotateSource<'ast, State>
where
    State: LexicalScopeOps<'ast>
        + Annotates<'ast, Expr, SourceAnnotation>
        + Annotates<'ast, Expr, ProjectionAnnotation>
        + Annotates<'ast, Query, ProjectionAnnotation>
        + Annotates<'ast, SetExpr, ProjectionAnnotation>
        + Annotates<'ast, Select, ProjectionAnnotation>
        + NodePathOps<'ast>,
{
    pub fn annotate_expr_with_source() -> impl Visitor<'ast, State, ResolutionError> {
        Expr::on_exit(&Self::annotate_expr)
    }

    pub fn annotate_select_with_projection() -> impl Visitor<'ast, State, ResolutionError> {
        Select::on_exit(&Self::annotate_select)
    }

    pub fn annotate_query_with_projection() -> impl Visitor<'ast, State, ResolutionError> {
        Query::on_exit(&Self::annotate_query)
    }

    pub fn annotate_set_expr_with_projection() -> impl Visitor<'ast, State, ResolutionError> {
        SetExpr::on_exit(&Self::annotate_set_expr)
    }

    fn annotate_select(
        node: &'ast Select,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError>
    where
        State: LexicalScopeOps<'ast>
            + Annotates<'ast, Expr, SourceAnnotation>
            + Annotates<'ast, Select, ProjectionAnnotation>
            + NodePathOps<'ast>,
    {
        let Select {
            projection: select_items,
            ..
        } = node;

        let result = select_items
            .iter()
            .flat_map(|item| match item {
                SelectItem::UnnamedExpr(expr) => match expr {
                    Expr::Identifier(ident) => vec![state
                        .expect_annotation(expr)
                        .map(|source: Rc<SourceAnnotation>| {
                            (source.clone(), Some(UniCase::new(ident.to_string())))
                        })
                        .map_err(ResolutionError::from)],
                    expr => vec![state
                        .expect_annotation(expr)
                        .map(|source: Rc<SourceAnnotation>| (source.clone(), None))
                        .map_err(ResolutionError::from)],
                },
                SelectItem::ExprWithAlias { expr, alias } => vec![state
                    .expect_annotation(expr)
                    .map(|source: Rc<SourceAnnotation>| {
                        (source.clone(), Some(UniCase::new(alias.to_string())))
                    })
                    .map_err(ResolutionError::from)],
                SelectItem::QualifiedWildcard(obj_name, _) => {
                    match state.resolve_qualified_wildcard(&obj_name.0) {
                        Ok(projection) => {
                            Vec::from_iter(projection.columns.iter().map(|c| Ok(c.clone())))
                        }
                        Err(err) => vec![Err(err)],
                    }
                }
                SelectItem::Wildcard(_) => match state.resolve_wildcard() {
                    Ok(projection) => {
                        Vec::from_iter(projection.columns.iter().map(|c| Ok(c.clone())))
                    }
                    Err(err) => vec![Err(err)],
                },
            })
            .collect::<Vec<_>>();

        let result: Result<Vec<(Rc<SourceAnnotation>, Option<UniCase<String>>)>, ResolutionError> =
            result.into_iter().collect();

        match result {
            Ok(columns) => {
                state.add_annotation(node, ProjectionAnnotation::Query(Projection { columns }));
                flow::cont(state)
            }
            Err(err) => flow::error(err, state),
        }
    }

    /// Resolves and records the [`Source`] for an [`Expr`] node.
    fn annotate_expr(
        node: &'ast Expr,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError>
    where
        State: LexicalScopeOps<'ast> + Annotates<'ast, Expr, SourceAnnotation> + NodePathOps<'ast>,
    {
        match node {
            Expr::Identifier(ident) => Self::resolve_ident_and_record_source(node, state, ident),
            Expr::CompoundIdentifier(idents) => {
                Self::resolve_compound_ident_and_record_source(node, state, idents)
            }
            Expr::BinaryOp { left, op: _, right } => {
                Self::resolve_two_sources(node, left, right, state)
            }
            Expr::JsonAccess {
                left,
                operator: _,
                right,
            } => Self::resolve_two_sources(node, left, right, state),
            Expr::CompositeAccess { expr, key: _ } => Self::resolve_one_source(node, expr, state),
            Expr::IsFalse(expr) => Self::resolve_one_source(node, expr, state),
            Expr::IsNotFalse(expr) => Self::resolve_one_source(node, expr, state),
            Expr::IsTrue(expr) => Self::resolve_one_source(node, expr, state),
            Expr::IsNotTrue(expr) => Self::resolve_one_source(node, expr, state),
            Expr::IsNull(expr) => Self::resolve_one_source(node, expr, state),
            Expr::IsNotNull(expr) => Self::resolve_one_source(node, expr, state),
            Expr::IsUnknown(expr) => Self::resolve_one_source(node, expr, state),
            Expr::IsNotUnknown(expr) => Self::resolve_one_source(node, expr, state),
            Expr::IsDistinctFrom(expr_0, expr_1) => {
                Self::resolve_two_sources(node, expr_0, expr_1, state)
            }
            Expr::IsNotDistinctFrom(expr_0, expr_1) => {
                Self::resolve_two_sources(node, expr_0, expr_1, state)
            }
            Expr::InList {
                expr,
                list,
                negated: _,
            } => {
                let mut exprs: Vec<&Expr> = list.iter().collect();
                exprs.push(expr);
                Self::resolve_sources(node, &exprs[..], state)
            }
            Expr::InSubquery {
                expr,
                subquery: _,
                negated: _,
            } => Self::resolve_one_source(node, expr, state),
            Expr::InUnnest {
                expr,
                array_expr,
                negated: _,
            } => Self::resolve_two_sources(node, expr, array_expr, state),
            Expr::Between {
                expr,
                negated: _,
                low,
                high,
            } => Self::resolve_sources(node, &[expr, low, high], state),
            Expr::Like {
                negated: _,
                expr,
                pattern,
                escape_char: _,
            } => Self::resolve_two_sources(node, expr, pattern, state),
            Expr::ILike {
                negated: _,
                expr,
                pattern,
                escape_char: _,
            } => Self::resolve_two_sources(node, expr, pattern, state),
            Expr::SimilarTo {
                negated: _,
                expr,
                pattern,
                escape_char: _,
            } => Self::resolve_two_sources(node, expr, pattern, state),
            Expr::RLike {
                negated: _,
                expr,
                pattern,
                regexp: _,
            } => Self::resolve_two_sources(node, expr, pattern, state),
            Expr::AnyOp {
                left,
                compare_op: _,
                right,
            } => Self::resolve_two_sources(node, left, right, state),
            Expr::AllOp {
                left,
                compare_op: _,
                right,
            } => Self::resolve_two_sources(node, left, right, state),
            Expr::UnaryOp { op: _, expr } => Self::resolve_one_source(node, expr, state),
            Expr::Convert {
                expr,
                data_type: _,
                charset: _,
                target_before_value: _,
            } => Self::resolve_one_source(node, expr, state),
            Expr::Cast {
                expr,
                data_type: _,
                format: _,
                kind: _,
            } => Self::resolve_one_source(node, expr, state),
            Expr::AtTimeZone {
                timestamp,
                time_zone: _,
            } => Self::resolve_one_source(node, timestamp, state),
            Expr::Extract { field: _, expr } => Self::resolve_one_source(node, expr, state),
            Expr::Ceil { expr, field: _ } => Self::resolve_one_source(node, expr, state),
            Expr::Floor { expr, field: _ } => Self::resolve_one_source(node, expr, state),
            Expr::Position { expr, r#in } => Self::resolve_sources(node, &[expr, r#in], state),
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
                Self::resolve_sources(node, &exprs[..], state)
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
                Self::resolve_sources(node, &exprs[..], state)
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
                Self::resolve_sources(node, &exprs[..], state)
            }
            Expr::Collate { expr, collation: _ } => Self::resolve_one_source(node, expr, state),
            Expr::Nested(expr) => Self::resolve_one_source(node, expr, state),
            Expr::Value(value) => {
                state.add_annotation(
                    node,
                    SourceAnnotation::single(SourceAnnotationItem::Value(value.clone())),
                );
                flow::cont(state)
            }
            Expr::IntroducedString { introducer, value } => {
                state.add_annotation(
                    node,
                    SourceAnnotation::single(SourceAnnotationItem::IntroducedString(
                        introducer.clone(),
                        value.clone(),
                    )),
                );
                flow::cont(state)
            }
            Expr::TypedString { data_type, value } => {
                state.add_annotation(
                    node,
                    SourceAnnotation::single(SourceAnnotationItem::TypedString(
                        data_type.clone(),
                        value.clone(),
                    )),
                );
                flow::cont(state)
            }
            Expr::MapAccess { column, keys } => {
                let mut exprs: Vec<&Expr> = Vec::new();
                exprs.push(column);
                exprs.extend(keys[..].iter().map(|k| &k.key));
                Self::resolve_sources(node, &exprs[..], state)
            }
            Expr::Function(Function { args: _, name, .. }) => {
                // TODO: Expr::Function - extract sources from function args

                state.add_annotation(
                    node,
                    SourceAnnotation::single(SourceAnnotationItem::FunctionCall(
                        name.to_string().into(),
                    )),
                );
                flow::cont(state)
            }
            Expr::AggregateExpressionWithFilter { expr, filter } => {
                Self::resolve_sources(node, &[expr, filter], state)
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
                Self::resolve_sources(node, &exprs[..], state)
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
            } => flow::cont(state),
            Expr::Subquery(query) => {
                let result: Result<Rc<SourceAnnotation>, ResolutionError> =
                    match <State as Annotates<'_, Query, ProjectionAnnotation>>::expect_annotation(
                        &state,
                        query.deref(),
                    ) {
                        Ok(projection_annotation) => match projection_annotation.deref() {
                            ProjectionAnnotation::Query(Projection { columns })
                            | ProjectionAnnotation::Insert {
                                returning: Some(Projection { columns }),
                                ..
                            } => {
                                if let Some(((first, _), rest)) = columns.split_first() {
                                    let merged = rest.iter().fold(
                                        first.clone(),
                                        |acc, (source_annotation, _)| {
                                            SourceAnnotation::merge(&acc, source_annotation)
                                        },
                                    );
                                    Ok(<State as Annotates<'_, Expr, SourceAnnotation>>::add_annotation(&mut state, node, merged))
                                } else {
                                    Err(ResolutionError::InvalidSubqueryExpr(query.clone()))
                                }
                            }
                            _ => Err(ResolutionError::InvalidSubqueryExpr(query.clone())),
                        },
                        Err(err) => Err(ResolutionError::ExpectedProjectionAnnotation(err)),
                    };

                match result {
                    Ok(_) => flow::cont(state),
                    Err(err) => flow::error(err, state),
                }
            }
            Expr::ArraySubquery(query) => {
                let result: Result<Rc<SourceAnnotation>, ResolutionError> =
                    match <State as Annotates<'_, Query, ProjectionAnnotation>>::expect_annotation(
                        &state,
                        query.deref(),
                    ) {
                        Ok(projection_annotation) => match projection_annotation.deref() {
                            ProjectionAnnotation::Query(Projection { columns })
                            | ProjectionAnnotation::Insert {
                                returning: Some(Projection { columns }),
                                ..
                            } => {
                                if let Some(((first, _), rest)) = columns.split_first() {
                                    let merged = rest.iter().fold(
                                        first.clone(),
                                        |acc, (source_annotation, _)| {
                                            SourceAnnotation::merge(&acc, source_annotation)
                                        },
                                    );
                                    Ok(<State as Annotates<'_, Expr, SourceAnnotation>>::add_annotation(&mut state, node, merged))
                                } else {
                                    Err(ResolutionError::InvalidArraySubqueryExpr(query.clone()))
                                }
                            }
                            _ => Err(ResolutionError::InvalidArraySubqueryExpr(query.clone())),
                        },
                        Err(err) => Err(ResolutionError::ExpectedProjectionAnnotation(err)),
                    };

                match result {
                    Ok(_) => flow::cont(state),
                    Err(err) => flow::error(err, state),
                }
            }
            Expr::ListAgg(agg) => {
                let mut exprs: Vec<&Expr> = Vec::new();
                exprs.push(agg.expr.deref());
                if let Some(sep) = agg.separator.as_ref() {
                    exprs.push(sep.deref())
                }
                if let Some(overflow) = agg.on_overflow.as_ref() {
                    if let ListAggOnOverflow::Truncate {
                        filler: Some(filler),
                        with_count: _,
                    } = overflow
                    {
                        exprs.push(filler)
                    }
                }
                Self::resolve_sources(node, &exprs[..], state)
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
                Self::resolve_sources(node, &exprs[..], state)
            }
            Expr::GroupingSets(grouping_sets) => {
                let exprs: Vec<&Expr> = grouping_sets.iter().flatten().collect();
                Self::resolve_sources(node, &exprs[..], state)
            }
            Expr::Cube(cube) => {
                let exprs: Vec<&Expr> = cube.iter().flatten().collect();
                Self::resolve_sources(node, &exprs[..], state)
            }
            Expr::Rollup(rollup) => {
                let exprs: Vec<&Expr> = rollup.iter().flatten().collect();
                Self::resolve_sources(node, &exprs[..], state)
            }
            Expr::Tuple(exprs) => {
                Self::resolve_sources(node, &exprs.iter().collect::<Vec<_>>()[..], state)
            }
            Expr::Struct { values, fields: _ } => {
                Self::resolve_sources(node, &values.iter().collect::<Vec<_>>()[..], state)
            }
            Expr::Named { expr, name: _ } => Self::resolve_one_source(node, expr, state),
            Expr::ArrayIndex { obj, indexes } => {
                let mut exprs: Vec<&Expr> = Vec::new();
                exprs.push(obj);
                exprs.extend(&indexes[..]);
                Self::resolve_sources(node, &exprs[..], state)
            }
            Expr::Array(array) => {
                Self::resolve_sources(node, &array.elem.iter().collect::<Vec<_>>()[..], state)
            }
            Expr::Interval(interval) => Self::resolve_one_source(node, &interval.value, state),
            // NOTE: this is MySQL specific
            Expr::MatchAgainst { .. } => flow::error(ResolutionError::Unimplemented, state),
            Expr::Wildcard => {
                let result = state.resolve_wildcard().map(|projection| {
                    if let Some(((first, _), rest)) = projection.columns.split_first() {
                        let merged = rest.iter().fold(first.clone(), |acc, (source, _)| {
                            SourceAnnotation::merge(&acc, source)
                        });
                        <State as Annotates<'_, Expr, SourceAnnotation>>::add_annotation(
                            &mut state, node, merged,
                        );
                        Ok(())
                    } else {
                        Err(ResolutionError::UnresolvableWildcard(Box::new(
                            node.clone(),
                        )))
                    }
                });

                match result {
                    Ok(_) => flow::cont(state),
                    Err(err) => flow::error(err, state),
                }
            }
            Expr::QualifiedWildcard(wildcard) => {
                let result = state
                    .resolve_qualified_wildcard(&wildcard.0)
                    .map(|projection| {
                        if let Some(((first, _), rest)) = projection.columns.split_first() {
                            let merged = rest.iter().fold(first.clone(), |acc, (source, _)| {
                                SourceAnnotation::merge(&acc, source)
                            });
                            <State as Annotates<'_, Expr, SourceAnnotation>>::add_annotation(
                                &mut state, node, merged,
                            );
                            Ok(())
                        } else {
                            Err(ResolutionError::UnresolvableWildcard(Box::new(
                                node.clone(),
                            )))
                        }
                    });

                match result {
                    Ok(_) => flow::cont(state),
                    Err(err) => flow::error(err, state),
                }
            }
            Expr::OuterJoin(expr) => Self::resolve_one_source(node, expr, state),
            Expr::Dictionary(dictionary_field) => Self::resolve_sources(
                node,
                dictionary_field
                    .iter()
                    .map(|df| df.value.deref())
                    .collect::<Vec<&Expr>>()
                    .as_slice(),
                state,
            ),
        }
    }
    fn resolve_ident_and_record_source(
        expr: &'ast Expr,
        mut state: State,
        ident: &'ast Ident,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        let resolved = state.resolve_ident(ident);
        match resolved {
            Ok(source) => {
                <State as Annotates<'_, Expr, SourceAnnotation>>::add_annotation(
                    &mut state, expr, source,
                );
                flow::cont(state)
            }
            Err(err) => flow::error(err, state),
        }
    }

    fn resolve_compound_ident_and_record_source(
        expr: &'ast Expr,
        mut state: State,
        compound_ident: &'ast [Ident],
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        let resolved = state.resolve_compound_ident(compound_ident);
        match resolved {
            Ok(source) => {
                <State as Annotates<'_, Expr, SourceAnnotation>>::add_annotation(
                    &mut state, expr, source,
                );
                flow::cont(state)
            }
            Err(err) => flow::error(err, state),
        }
    }

    fn resolve_sources(
        node: &'ast Expr,
        exprs: &[&'ast Expr],
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        let sources = exprs
            .iter()
            .try_fold(Vec::with_capacity(exprs.len()), |mut acc, expr| {
                <State as Annotates<'_, Expr, SourceAnnotation>>::expect_annotation(&state, *expr)
                    .map(|source| {
                        acc.push(source);
                        acc
                    })
            });

        match sources {
            Ok(sources) => {
                let (first, rest) = sources
                    .split_first()
                    .expect("there should be a source for every Expr");
                let merged = rest.iter().fold(first.clone(), |acc, source| {
                    SourceAnnotation::merge(&acc, source)
                });
                <State as Annotates<'_, Expr, SourceAnnotation>>::add_annotation(
                    &mut state, node, merged,
                );
                flow::cont(state)
            }
            Err(err) => flow::error(err.into(), state),
        }
    }

    fn resolve_two_sources(
        node: &'ast Expr,
        expr_0: &'ast Expr,
        expr_1: &'ast Expr,
        state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        Self::resolve_sources(node, &[expr_0, expr_1], state)
    }

    fn resolve_one_source(
        node: &'ast Expr,
        expr: &'ast Expr,
        state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        Self::resolve_sources(node, &[expr], state)
    }

    fn annotate_query(
        query: &'ast Query,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError>
    where
        State: LexicalScopeOps<'ast> + Annotates<'ast, SetExpr, ProjectionAnnotation>,
    {
        match state.expect_annotation(query.body.deref()) {
            Ok(projection_annotation) => {
                state.add_annotation(query, projection_annotation);
                flow::cont(state)
            }
            Err(err) => flow::error(err.into(), state),
        }
    }

    fn annotate_set_expr(
        set_expr: &'ast SetExpr,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError>
    where
        State: LexicalScopeOps<'ast>
            + Annotates<'ast, Query, ProjectionAnnotation>
            + Annotates<'ast, SetExpr, ProjectionAnnotation>
            + Annotates<'ast, Select, ProjectionAnnotation>,
    {
        match set_expr {
            SetExpr::Select(select) => match state.expect_annotation(select.deref()) {
                // Simply clone the annotation from the SetExpr to the Query.
                Ok(projection_annotation) => {
                    state.add_annotation(set_expr, projection_annotation);
                    flow::cont(state)
                }
                Err(err) => flow::error(err.into(), state),
            },
            SetExpr::Query(query) => match state.expect_annotation(query.deref()) {
                // Simply clone the annotation from the SetExpr to the Query.
                Ok(projection_annotation) => {
                    state.add_annotation(set_expr, projection_annotation);
                    flow::cont(state)
                }
                Err(err) => flow::error(err.into(), state),
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
                    Ok((left_annotation, right_annotation)) => {
                        if let (Some(left_projection), Some(right_projection)) =
                            (left_annotation.projection(), right_annotation.projection())
                        {
                            let merged_columns = left_projection
                                .columns
                                .iter()
                                .zip(right_projection.columns.clone())
                                .map(|((left_source, left_ident), (right_source, _))| {
                                    (
                                        SourceAnnotation::merge(left_source, &right_source),
                                        left_ident.clone(),
                                    )
                                })
                                .collect::<Vec<_>>();

                            state.add_annotation(
                                set_expr,
                                ProjectionAnnotation::Query(Projection {
                                    columns: merged_columns,
                                }),
                            );

                            flow::cont(state)
                        } else {
                            flow::error(
                                ResolutionError::IncompatibleOperandsForSetOperation(
                                    left.clone(),
                                    right.clone(),
                                ),
                                state,
                            )
                        }
                    }
                    Err(err) => flow::error(err.into(), state),
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
                        (
                            Rc::new(SourceAnnotation::single(
                                SourceAnnotationItem::ColumnOfValues,
                            )),
                            None,
                        )
                    })
                    .collect::<Vec<_>>();

                state.add_annotation(
                    set_expr,
                    ProjectionAnnotation::Query(Projection {
                        columns: projection_columns,
                    }),
                );

                flow::cont(state)
            }
            SetExpr::Insert(_statement) => {
                // if let Statement::Insert(Insert {
                //     columns, source, returning, ..
                // }) = statement
                // {
                // }
                flow::error(
                    ResolutionError::UnsupportedInsertOrUpdateInSubqueryExpressionPosition(
                        Box::new(set_expr.clone()),
                    ),
                    state,
                )
            }
            // TODO: implement this
            SetExpr::Update(_statement) => flow::error(
                ResolutionError::UnsupportedInsertOrUpdateInSubqueryExpressionPosition(Box::new(
                    set_expr.clone(),
                )),
                state,
            ),
            // TODO: implement this
            SetExpr::Table(_table) => flow::error(
                ResolutionError::UnsupportedTableKeywordInSubqueryExpressionPosition(Box::new(
                    set_expr.clone(),
                )),
                state,
            ),
        }
    }
}
