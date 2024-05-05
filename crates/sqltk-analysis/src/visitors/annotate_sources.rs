//! Traits and types used used to annotate [`Expr`] nodes with their [`Source`].s

use core::marker::PhantomData;
use core::ops::Deref;
use std::rc::Rc;

use crate::model::Annotates;
use crate::model::ResolutionError;
use crate::model::ScopeOps;
use crate::model::{CanonicalIdent, SqlIdent};
use crate::model::{ColumnWritten, InsertProvenance, Provenance, SelectProvenance};
use crate::model::{Projection, ProjectionColumn};
use crate::model::{Source, SourceItem};
use crate::node_path::NodePathOps;
use crate::SchemaOps;

use sqltk::prelude::{Ident, Query};
use sqltk::{flow, VisitorControlFlow};
use sqltk::{Visitable, Visitor};

use sqlparser::ast::{
    Expr, Function, Insert, ListAggOnOverflow, Select, SelectItem, SetExpr, Statement,
};

/// Type that provides functions for building [`Visitor`] implementations that
/// can annotate [`Expr`] and [`SelectItem`] nodes with their [`Source`].
pub struct AnnotateSource<'ast, State: 'ast>(PhantomData<&'ast State>);

impl<'ast, State: 'ast> AnnotateSource<'ast, State>
where
    State: ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + Annotates<'ast, Expr, Projection>
        + Annotates<'ast, Query, Projection>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Select, Projection>
        + Annotates<'ast, Statement, Provenance>
        + NodePathOps<'ast>
        + SchemaOps,
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
    pub fn annotate_select_item_with_source() -> impl Visitor<'ast, State, ResolutionError> {
        SelectItem::on_exit(&Self::annotate_select_item_with_projection_column)
    }

    pub fn annotate_set_expr_with_projection() -> impl Visitor<'ast, State, ResolutionError> {
        SetExpr::on_exit(&Self::annotate_set_expr)
    }

    pub fn annotate_statement_with_projection() -> impl Visitor<'ast, State, ResolutionError> {
        Statement::on_exit(&Self::annotate_statement)
    }

    fn annotate_select_item_with_projection_column(
        node: &'ast SelectItem,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        let result: Vec<Result<Rc<ProjectionColumn>, ResolutionError>> = match node {
            SelectItem::UnnamedExpr(expr) => match expr {
                Expr::Identifier(ident) => vec![state
                    .expect_annotation(expr)
                    .map(|source: Rc<Source>| {
                        ProjectionColumn::new(source.clone(), Some(SqlIdent::from(ident).into()))
                            .into()
                    })
                    .map_err(ResolutionError::from)],
                expr => vec![state
                    .expect_annotation(expr)
                    .map(|source: Rc<Source>| ProjectionColumn::new(source.clone(), None).into())
                    .map_err(ResolutionError::from)],
            },
            SelectItem::ExprWithAlias { expr, alias } => vec![state
                .expect_annotation(expr)
                .map(|source: Rc<Source>| {
                    ProjectionColumn::new(source.clone(), Some(SqlIdent::from(alias).into())).into()
                })
                .map_err(ResolutionError::from)]
            .into(),
            SelectItem::QualifiedWildcard(obj_name, _) => {
                let idents: Vec<SqlIdent> = obj_name.0.iter().map(SqlIdent::from).collect();
                match state.resolve_qualified_wildcard(idents.as_slice()) {
                    Ok(projection) => {
                        Vec::from_iter(projection.columns.clone().into_iter().map(|c| Ok(c)))
                    }
                    Err(err) => vec![Err(err.into())],
                }
            }
            SelectItem::Wildcard(_) => match state.resolve_wildcard() {
                Ok(projection) => Vec::from_iter(projection.columns.iter().map(|c| Ok(c.clone()))),
                Err(err) => vec![Err(err)],
            },
        };

        let result: Result<Vec<Rc<ProjectionColumn>>, ResolutionError> =
            result.into_iter().collect();

        match result {
            Ok(columns) => {
                state.add_annotation(node, columns);
                flow::cont(state)
            }
            Err(err) => flow::error(err, state),
        }
    }

    fn annotate_select(
        node: &'ast Select,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        let Select {
            projection: select_items,
            ..
        } = node;

        let result: Result<Vec<Rc<Vec<Rc<ProjectionColumn>>>>, ResolutionError> = select_items
            .iter()
            .map(|item| {
                state
                    .expect_annotation(item)
                    .map_err(|err| ResolutionError::from(err))
            })
            .collect();

        let result: Result<Vec<Rc<ProjectionColumn>>, ResolutionError> = result.map(|items| {
            items
                .into_iter()
                .map(|item| item.deref().clone().into_iter().collect::<Vec<_>>())
                .flatten()
                .collect()
        });

        match result {
            Ok(columns) => {
                state.add_annotation(node, Projection { columns });
                flow::cont(state)
            }
            Err(err) => flow::error(err, state),
        }
    }

    /// Resolves and records the [`Source`] for an [`Expr`] node.
    fn annotate_expr(
        node: &'ast Expr,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
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
                state.add_annotation(node, Source::single(SourceItem::Value(value.clone())));
                flow::cont(state)
            }
            Expr::IntroducedString { introducer, value } => {
                state.add_annotation(
                    node,
                    Source::single(SourceItem::IntroducedString(
                        introducer.clone(),
                        value.clone(),
                    )),
                );
                flow::cont(state)
            }
            Expr::TypedString { data_type, value } => {
                state.add_annotation(
                    node,
                    Source::single(SourceItem::TypedString(data_type.clone(), value.clone())),
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
                state.add_annotation(
                    node,
                    Source::single(SourceItem::FunctionCall {
                        ident: SqlIdent::from(
                            name.0.last().expect(
                                "A sqlparser ObjectName to have at least one identifier part",
                            ),
                        ),

                        arg_sources: vec![], // TODO: capture the Sources for the function args
                    }),
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
                let result: Result<Rc<Source>, ResolutionError> =
                    match <State as Annotates<'_, Query, Projection>>::expect_annotation(
                        &state,
                        query.deref(),
                    ) {
                        Ok(projection) => {
                            let Projection { columns } = projection.deref();
                            if let Some((projection_column, rest)) = columns.split_first() {
                                let merged = rest.iter().fold(
                                    projection_column.source.clone(),
                                    |acc, projection_column| {
                                        Source::merge(&acc, &projection_column.source)
                                    },
                                );
                                Ok(<State as Annotates<'_, Expr, Source>>::add_annotation(
                                    &mut state, node, merged,
                                ))
                            } else {
                                Err(ResolutionError::InvalidSubqueryExpr(query.clone()))
                            }
                        }
                        Err(err) => Err(ResolutionError::ExpectedProjectionAnnotation(err)),
                    };

                match result {
                    Ok(_) => flow::cont(state),
                    Err(err) => flow::error(err, state),
                }
            }
            Expr::ArraySubquery(query) => {
                let result: Result<Rc<Source>, ResolutionError> =
                    match state.expect_annotation(query.deref()) {
                        Ok(projection) => {
                            let Projection { columns } = projection.deref();
                            if let Some((projection_column, rest)) = columns.split_first() {
                                let merged = rest.iter().fold(
                                    projection_column.source.clone(),
                                    |acc, projection_column| {
                                        Source::merge(&acc, &projection_column.source)
                                    },
                                );
                                Ok(state.add_annotation(node, merged))
                            } else {
                                Err(ResolutionError::InvalidArraySubqueryExpr(query.clone()))
                            }
                        }
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
                    if let Some((projection_column, rest)) = projection.columns.split_first() {
                        let merged = rest.iter().fold(
                            projection_column.source.clone(),
                            |acc, projection_column| Source::merge(&acc, &projection_column.source),
                        );
                        <State as Annotates<'_, Expr, Source>>::add_annotation(
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
                    .resolve_qualified_wildcard(
                        Vec::from_iter(wildcard.0.iter().map(SqlIdent::from)).as_slice(),
                    )
                    .map(|projection| {
                        if let Some((projection_column, rest)) = projection.columns.split_first() {
                            let merged = rest.iter().fold(
                                projection_column.source.clone(),
                                |acc, projection_column| {
                                    Source::merge(&acc, &projection_column.source)
                                },
                            );
                            <State as Annotates<'_, Expr, Source>>::add_annotation(
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
        let resolved = state.resolve_ident(&ident.into());
        match resolved {
            Ok(source) => {
                <State as Annotates<'_, Expr, Source>>::add_annotation(&mut state, expr, source);
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
        let resolved = state.resolve_compound_ident(
            Vec::from_iter(compound_ident.iter().map(SqlIdent::from)).as_slice(),
        );
        match resolved {
            Ok(source) => {
                <State as Annotates<'_, Expr, Source>>::add_annotation(&mut state, expr, source);
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
                <State as Annotates<'_, Expr, Source>>::expect_annotation(&state, *expr).map(
                    |source| {
                        acc.push(source);
                        acc
                    },
                )
            });

        match sources {
            Ok(sources) => {
                let (first, rest) = sources
                    .split_first()
                    .expect("there should be a source for every Expr");
                let merged = rest
                    .iter()
                    .fold(first.clone(), |acc, source| Source::merge(&acc, source));
                <State as Annotates<'_, Expr, Source>>::add_annotation(&mut state, node, merged);
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
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        match state.expect_annotation(query.body.deref()) {
            Ok(projection_annotation) => {
                state.add_annotation(query, projection_annotation);
                flow::cont(state)
            }
            Err(err) => flow::error(err.into(), state),
        }
    }

    fn annotate_statement(
        statement: &'ast Statement,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        let result: Result<State, (ResolutionError, State)> = match statement {
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
                    Ok(state)
                }
                Err(err) => Err((err.into(), state)),
            },
            Statement::Insert(Insert {
                table_name,
                columns,
                source,
                returning,
                ..
            }) => {
                let table: Result<_, ResolutionError> = state
                    .get_schema()
                    .resolve_table(&SqlIdent::from(table_name.0.last().unwrap()))
                    .map_err(|err| err.into());

                let source: Result<Option<Rc<Projection>>, ResolutionError> = match source {
                    Some(source) => state
                        .expect_annotation(source.deref())
                        .map(|projection| projection.clone())
                        .map(Some)
                        .map_err(ResolutionError::from),
                    None => Ok(None),
                };

                let columns_written: Result<Vec<ColumnWritten>, ResolutionError> = match source {
                    Ok(Some(projection)) => columns
                        .clone()
                        .into_iter()
                        .map(|ident| CanonicalIdent::from(ident.value))
                        .zip(projection.columns.iter())
                        .map(|(ident, column)| {
                            Ok(ColumnWritten {
                                column: ident.into(),
                                data: column.source.clone(),
                            })
                        })
                        .collect::<Result<Vec<_>, _>>(),
                    Ok(None) => Ok(vec![]),
                    Err(err) => Err(err.into()),
                };

                let returning: Result<Option<Rc<Projection>>, ResolutionError> = match returning {
                    Some(select_items) => {
                        match select_items
                            .iter()
                            .map(|item| state.expect_annotation(item))
                            .collect::<Result<Vec<Rc<Vec<Rc<ProjectionColumn>>>>, _>>()
                        {
                            Ok(projection_columns) => {
                                if projection_columns.len() > 0 {
                                    Ok(Some(
                                        Projection {
                                            columns: projection_columns
                                                .iter()
                                                .map(|rc| rc.deref())
                                                .flatten()
                                                .map(|source| source.clone())
                                                .collect(),
                                        }
                                        .into(),
                                    ))
                                } else {
                                    Ok(None)
                                }
                            }
                            Err(err) => Err(err.into()),
                        }
                    }
                    None => Ok(None),
                };

                match (table, columns_written, returning) {
                    (Ok(into_table), Ok(columns_written), Ok(returning)) => {
                        state.add_annotation(
                            statement,
                            Provenance::Insert(
                                InsertProvenance {
                                    into_table,
                                    columns_written,
                                    returning,
                                }
                                .into(),
                            ),
                        );
                        Ok(state)
                    }
                    (table, columns_written, returning) => {
                        match table.and(columns_written).and(returning) {
                            Ok(_) => Ok(state),
                            Err(err) => Err((err.into(), state)),
                        }
                    }
                }
            }
            // Statement::Update {
            //     table,
            //     assignments,
            //     from,
            //     selection,
            //     returning,
            // } => todo!(),
            // Statement::Delete(Delete {
            //     tables,
            //     from,
            //     using,
            //     selection,
            //     returning,
            //     ..
            // }) => todo!(),
            _ => Ok(state),
        };

        match result {
            Ok(state) => flow::cont(state),
            Err((err, state)) => flow::error(err.into(), state),
        }
    }

    fn annotate_set_expr(
        set_expr: &'ast SetExpr,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        match set_expr {
            SetExpr::Select(select) => match state.expect_annotation(select.deref()) {
                // Simply clone the annotation from the SetExpr to the Query.
                Ok(projection) => {
                    state.add_annotation(set_expr, projection);
                    flow::cont(state)
                }
                Err(err) => flow::error(err.into(), state),
            },
            SetExpr::Query(query) => match state.expect_annotation(query.deref()) {
                // Simply clone the annotation from the SetExpr to the Query.
                Ok(projection) => {
                    state.add_annotation(set_expr, projection);
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
                    Ok((left_projection, right_projection)) => {
                        let merged_columns = left_projection
                            .columns
                            .iter()
                            .zip(right_projection.columns.clone())
                            .map(|(left, right)| {
                                Rc::new(ProjectionColumn::new(
                                    Source::merge(&left.source, &right.source),
                                    left.alias.clone(),
                                ))
                            })
                            .collect::<Vec<_>>();

                        state.add_annotation(
                            set_expr,
                            Projection {
                                columns: merged_columns,
                            },
                        );

                        flow::cont(state)
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
                        Rc::new(ProjectionColumn::new(
                            Rc::new(Source::single(SourceItem::ColumnOfValues)),
                            None,
                        ))
                    })
                    .collect::<Vec<_>>();

                state.add_annotation(
                    set_expr,
                    Projection {
                        columns: projection_columns,
                    },
                );

                flow::cont(state)
            }
            SetExpr::Insert(_statement) => {
                // TODO!
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
