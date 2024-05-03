//! Types and functions for provenance analysis.

use sqltk::prelude::*;

use crate::{
    annotate_sources::AnnotateSource,
    annotations::Annotates,
    lexical_scope::{LexicalScope, LexicalScopeOps},
    model::resolution_error::ResolutionError,
    node_path::{NodePath, NodePathOps},
    projection_annotation::ProjectionAnnotation,
    source_annotation::SourceAnnotation,
    SchemaOps,
};

use std::convert::Infallible;
use thiserror::Error;

mod state;

/// Returns a [`Visitor`] that performs provenance analysis of a SQL AST.
pub fn new_provenance_visitor<'ast, State: 'ast>() -> impl Visitor<'ast, State, ProvenanceError>
where
    State: LexicalScopeOps<'ast>
        + Annotates<'ast, Expr, SourceAnnotation>
        // + Annotates<'ast, SelectItem, SourceAnnotation>
        + Annotates<'ast, Expr, ProjectionAnnotation>
        + Annotates<'ast, Query, ProjectionAnnotation>
        + Annotates<'ast, SetExpr, ProjectionAnnotation>
        + Annotates<'ast, Select, ProjectionAnnotation>
        + SchemaOps
        + NodePathOps<'ast>,
{
    let mut visitor = VisitorStack::<State, ProvenanceError>::new();

    visitor.push(NodePath::track());

    // #[cfg(test)]
    // visitor.push(NodePath::log_top_entry());

    visitor.push(Statement::on_enter(flow::infallible::modify_state(
        |state: &mut State| state.push_scope(),
    )));

    visitor.push(Query::on_enter(flow::infallible::modify_state(
        |state: &mut State| state.push_scope(),
    )));

    // visitor.push(LexicalScope::push_and_pop_scope_for_statements());
    // visitor.push(LexicalScope::push_and_pop_scope_for_subqueries());
    visitor.push(LexicalScope::bring_table_factor_into_scope());
    visitor.push(LexicalScope::bring_cte_into_scope());

    visitor.push(AnnotateSource::annotate_expr_with_source());
    visitor.push(AnnotateSource::annotate_set_expr_with_projection());
    visitor.push(AnnotateSource::annotate_select_with_projection());
    visitor.push(AnnotateSource::annotate_query_with_projection());

    visitor.push(Statement::on_exit(flow::infallible::modify_state(
        |state: &mut State| state.pop_scope(),
    )));

    visitor.push(Query::on_exit(flow::infallible::modify_state(
        |state: &mut State| state.pop_scope(),
    )));

    visitor
}

/// Errors that can be returned during provenance analysis.
#[derive(Debug, Error)]
pub enum ProvenanceError {
    #[error("Could not resolve identifier")]
    ResolutionError(#[from] ResolutionError),
}

/// Necessary to support being able to build a `VisitorStack` instance.
///
/// Have a read about Rust's [`Infallible`] enum if you need to understand how
/// this works.
impl From<Infallible> for ProvenanceError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

#[cfg(test)]
mod tests {

    use core::ops::Deref;
    use std::rc::Rc;

    use crate::{
        make_schema, new_provenance_visitor,
        projection_annotation::{Projection, ProjectionAnnotation},
        schema::Table,
        source_annotation::{SourceAnnotation, SourceAnnotationItem, TableColumn},
    };
    use bigdecimal::BigDecimal;
    use sqltk::prelude::*;

    use super::state::ProvenanceState;

    fn parse_sql(sql: &str) -> Vec<Statement> {
        let dialect = GenericDialect {};
        Parser::parse_sql(&dialect, sql).unwrap()
    }

    #[test]
    fn select_one_column_from_one_table() {
        let schema = make_schema! {
            users (
                id
                email
                first_name
            )
        };

        let statements = parse_sql("select id from users;");

        let state = ProvenanceState {
            schema,
            ..Default::default()
        };

        let visitor = new_provenance_visitor();

        let state = statements.evaluate(&visitor, state).unwrap();
        let projection = state
            .projection_annotations
            .values_for_key_type::<Query>()
            .collect::<Vec<_>>();

        assert_eq!(
            projection[0].deref(),
            &ProjectionAnnotation::Query(Projection {
                columns: vec![(
                    Rc::new(SourceAnnotation::single(SourceAnnotationItem::TableColumn(
                        TableColumn::new("users".into(), "id".into())
                    ))),
                    Some("id".into())
                )]
            })
        );
    }

    #[test]
    fn select_columns_from_multiple_tables() {
        let schema = make_schema! {
            users (
                id
                email
                first_name
            )
            todo_lists (
                id
                name
                owner_id
                created_at
                updated_at
            )
        };

        let statements = parse_sql(
            "select u.id from users as u inner join todo_lists as tl on tl.owner_id = u.id;",
        );

        let state = ProvenanceState {
            schema,
            ..Default::default()
        };

        let visitor = new_provenance_visitor();

        let state = statements.evaluate(&visitor, state).unwrap();
        let projection = state
            .projection_annotations
            .values_for_key_type::<Query>()
            .collect::<Vec<_>>();

        assert_eq!(projection.len(), 1);

        assert_eq!(
            projection[0].deref(),
            &ProjectionAnnotation::Query(Projection {
                columns: vec![(
                    Rc::new(SourceAnnotation::single(SourceAnnotationItem::TableColumn(
                        TableColumn::new("users".into(), "id".into())
                    ))),
                    None
                )]
            })
        );
    }

    #[test]
    fn select_columns_from_subquery() {
        let schema = make_schema! {
            users (
                id
                email
                first_name
            )
            todo_lists (
                id
                name
                owner_id
                created_at
                updated_at
            )
            todo_list_items (
                id
                description
                owner_id
                created_at
                updated_at
            )
        };

        let statements = parse_sql(
            r#"
                select
                    u.id as user_id,
                    tli.id as todo_list_item_id,
                    tli.description as todo_list_item_description
                from
                    users as u
                inner join (
                    select
                        id,
                        owner_id,
                        description
                    from
                        todo_list_items
                ) as tli on tli.owner_id = u.id;
            "#,
        );

        let state = ProvenanceState {
            schema,
            ..Default::default()
        };

        let visitor = new_provenance_visitor();

        let state = statements.evaluate(&visitor, state).unwrap();
        let projection = state.statement_provenance(&statements[0]).unwrap();

        assert_eq!(projection.columns.len(), 3);

        assert_eq!(
            &projection.columns[0].0,
            &Rc::new(SourceAnnotation::single(SourceAnnotationItem::TableColumn(
                TableColumn::new("users".into(), "id".into())
            )))
        );

        assert_eq!(
            &projection.columns[1].0,
            &Rc::new(SourceAnnotation::single(SourceAnnotationItem::TableColumn(
                TableColumn::new("todo_list_items".into(), "id".into())
            )))
        );

        assert_eq!(
            &projection.columns[2].0,
            &Rc::new(SourceAnnotation::single(SourceAnnotationItem::TableColumn(
                TableColumn::new("todo_list_items".into(), "description".into())
            )))
        );
    }

    #[test]
    fn select_columns_from_correlated_subquery() {
        let schema = make_schema! {
            film (
                id
                title
                length
                rating
            )
        };

        let statements = parse_sql(
            r#"
            select f.id, f.title, f.length, f.rating
            from film f
            where length > (
                select avg(length)
                from film
                where rating = f.rating
            );
            "#,
        );

        let state = ProvenanceState {
            schema,
            ..Default::default()
        };

        let visitor = new_provenance_visitor();

        let state = statements.evaluate(&visitor, state).unwrap();
        let projection = state.statement_provenance(&statements[0]).unwrap();

        assert_eq!(projection.columns.len(), 4);

        assert_eq!(
            projection.columns[0].0.deref(),
            &SourceAnnotation::single(SourceAnnotationItem::TableColumn(TableColumn::new(
                "film".into(),
                "id".into()
            )))
        );

        assert_eq!(
            projection.columns[1].0.deref(),
            &SourceAnnotation::single(SourceAnnotationItem::TableColumn(TableColumn::new(
                "film".into(),
                "title".into()
            )))
        );

        assert_eq!(
            projection.columns[2].0.deref(),
            &SourceAnnotation::single(SourceAnnotationItem::TableColumn(TableColumn::new(
                "film".into(),
                "length".into()
            )))
        );

        assert_eq!(
            projection.columns[3].0.deref(),
            &SourceAnnotation::single(SourceAnnotationItem::TableColumn(TableColumn::new(
                "film".into(),
                "rating".into()
            )))
        );
    }

    #[test]
    fn select_columns_from_cte() {
        let schema = make_schema! {};

        let statements = parse_sql(
            r#"
                with some_cte as (
                    select 123 as id
                )
                select id from some_cte;
            "#,
        );

        let state = ProvenanceState {
            schema,
            ..Default::default()
        };

        let visitor = new_provenance_visitor();

        let state = statements.evaluate(&visitor, state).unwrap();
        let projection = state.statement_provenance(&statements[0]).unwrap();

        assert_eq!(projection.columns.len(), 1);

        assert_eq!(
            projection.columns[0].0.deref(),
            &SourceAnnotation::single(SourceAnnotationItem::Value(Value::Number(
                BigDecimal::from(123),
                false
            )))
        );
    }
}
