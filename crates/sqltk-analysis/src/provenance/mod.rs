//! Types and functions for provenance analysis.

use sqltk::prelude::*;

use crate::{
    annotate_sources::{AnnotateSource, SourceAnnotationOps},
    lexical_scope::{LexicalScope, LexicalScopeOps},
    model::resolution_error::ResolutionError,
    node_path::{NodePath, NodePathOps},
    SchemaOps,
};

use std::convert::Infallible;
use thiserror::Error;

mod state;

/// Returns a [`Visitor`] that performs provenance analysis of a SQL AST.
pub fn new_provenance_visitor<'ast, State: 'ast>() -> impl Visitor<'ast, State, ProvenanceError>
where
    State: LexicalScopeOps<'ast> + SourceAnnotationOps<'ast> + SchemaOps + NodePathOps<'ast>,
{
    let mut visitor = VisitorStack::<State, ProvenanceError>::new();

    visitor.push(NodePath::track());

    #[cfg(test)]
    visitor.push(NodePath::log_top_entry());

    visitor.push(LexicalScope::reset_for_each_statement());
    visitor.push(LexicalScope::bring_tables_into_scope());

    visitor.push(AnnotateSource::annotate_expr_with_source());
    visitor.push(AnnotateSource::annotate_select_item_with_source());

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

    use crate::{
        make_schema, new_provenance_visitor,
        schema::{Column, ColumnType, Table},
        sources::{ColumnRef, Source, SourceItem, TableColumn},
    };
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
                id:uuid
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

        match statements.evaluate(&visitor, state) {
            Ok(state) => {
                let projection = state
                    .source_annotations
                    .values_for_key_type::<SelectItem>()
                    .collect::<Vec<_>>();

                assert_eq!(
                    projection[0].deref(),
                    &Source::single(SourceItem::TableColumn(TableColumn::new(
                        "users".into(),
                        ColumnRef::Identifier("id".into())
                    )))
                );
            }
            Err(err) => assert!(false, "{:?}", err),
        };
    }

    #[test]
    fn select_columns_from_multiple_tables() {
        let schema = make_schema! {
            users (
                id:uuid
                email
                first_name
            )
            todo_lists (
                id:uuid
                name
                owner_id:uuid
                created_at:timestamp
                updated_at:timestamp
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

        match statements.evaluate(&visitor, state) {
            Ok(state) => {
                let projection = state
                    .source_annotations
                    .values_for_key_type::<SelectItem>()
                    .collect::<Vec<_>>();

                assert_eq!(projection.len(), 1);

                assert_eq!(
                    projection[0].deref(),
                    &Source::single(SourceItem::TableColumn(TableColumn::new(
                        "users".into(),
                        ColumnRef::Identifier("id".into())
                    )))
                );
            }
            Err((err, _state)) => assert!(false, "{:?}", err),
        };
    }

    #[test]
    fn select_columns_from_subquery() {
        let schema = make_schema! {
            users (
                id:uuid
                email
                first_name
            )
            todo_lists (
                id:uuid
                name
                owner_id:uuid
                created_at:timestamp
                updated_at:timestamp
            )
            todo_list_items (
                id:uuid
                description
                owner_id:uuid
                created_at:timestamp
                updated_at:timestamp
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
            "#
        );

        let state = ProvenanceState {
            schema,
            ..Default::default()
        };

        let visitor = new_provenance_visitor();

        match statements.evaluate(&visitor, state) {
            Ok(state) => {
                let projection = state.statement_provenance(&statements[0]).unwrap();

                assert_eq!(projection.len(), 3);

                assert_eq!(
                    projection[0],
                    Source::single(SourceItem::TableColumn(TableColumn::new(
                        "users".into(),
                        ColumnRef::Identifier("id".into())
                    )))
                );

                assert_eq!(
                    projection[1],
                    Source::single(SourceItem::TableColumn(TableColumn::new(
                        "todo_list_items".into(),
                        ColumnRef::Identifier("id".into())
                    )))
                );

                assert_eq!(
                    projection[2],
                    Source::single(SourceItem::TableColumn(TableColumn::new(
                        "todo_list_items".into(),
                        ColumnRef::Identifier("description".into())
                    )))
                );
            }
            Err((err, _state)) => assert!(false, "{:?}", err),
        };
    }
}
