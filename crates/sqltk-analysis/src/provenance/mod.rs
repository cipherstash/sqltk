//! Types and functions for provenance analysis.

use sqltk::Visitor;

use crate::build_projection_columns::BuildProjectionColumns;
use crate::build_projections::BuildProjections;
use crate::build_statement_provenance::BuildStatementProvenance;
use crate::trace_expr_sources::TraceExprSources;
use crate::{
    import_relations::ImportRelations, model::Provenance, model::ResolutionError,
    update_stack::UpdateStack,
};

use core::marker::PhantomData;
use std::convert::Infallible;
use std::fmt::Debug;
use thiserror::Error;

mod state;

pub use state::*;

#[derive(Debug, Visitor)]
#[visitor(
    error_ty = ProvenanceError,
    children = [
        UpdateStack,
        ImportRelations,
        TraceExprSources,
        BuildProjectionColumns,
        BuildProjections,
        BuildStatementProvenance,
    ]
)]
pub struct ProvenanceAnalyser<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>)
where
    State: Debug + ProvenanceStateBounds<'ast>;

impl<'ast, State> Default for ProvenanceAnalyser<'ast, State>
where
    State: Debug + ProvenanceStateBounds<'ast>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
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
    use pretty_assertions::assert_eq;
    use std::rc::Rc;

    use crate::{
        make_schema,
        model::{
            Annotate, CanonicalIdent, InsertProvenance, Projection, ProjectionColumn, Provenance,
            SelectProvenance, Source, SourceItem, SqlIdent, Table, TableColumn,
        },
        DeleteProvenance, ProvenanceAnalyser,
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
            tables: {
                users: {
                    id (PK),
                    email,
                    first_name,
                }
            }
        };

        let user_id_column = schema
            .resolve_table_column(&SqlIdent::unquoted("users"), &SqlIdent::unquoted("id"))
            .unwrap();

        let statements = parse_sql("select id from users;");

        let state = ProvenanceState::new(schema);

        match statements.evaluate(&ProvenanceAnalyser::default(), state) {
            Ok(state) => match state.get_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let SelectProvenance { projection } = provenance.deref();
                    assert_eq!(
                        projection.deref(),
                        &Projection::Columns(vec![ProjectionColumn {
                            source: Source::single(SourceItem::TableColumn(user_id_column.clone()))
                                .into(),
                            alias: Some(SqlIdent::unquoted("id").into())
                        }
                        .into()])
                    );
                }
                Ok(_) => {
                    assert!(false, "Wrong Provenance variant")
                }
                Err(err) => {
                    assert!(false, "Error retrieving Provenance: {:#?}", err)
                }
            },
            Err(err) => {
                assert!(false, "Error during AST evaluation: {:#?}", err)
            }
        }
    }

    #[test]
    fn select_columns_from_multiple_tables() {
        let schema = make_schema! {
            tables: {
                users: {
                    id (PK),
                    email,
                    first_name,
                }
                todo_lists: {
                    id (PK),
                    name,
                    owner_id,
                    created_at,
                    updated_at,
                }
            }
        };

        let users_table = schema.resolve_table(&SqlIdent::unquoted("users")).unwrap();
        let user_id_column = users_table.get_column(&SqlIdent::unquoted("id")).unwrap();

        let statements = parse_sql(
            "select u.id from users as u inner join todo_lists as tl on tl.owner_id = u.id;",
        );

        let state = ProvenanceState::new(schema);

        match statements.evaluate(&ProvenanceAnalyser::default(), state) {
            Ok(state) => match state.get_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let SelectProvenance { projection } = provenance.deref();
                    assert_eq!(
                        projection.deref(),
                        &Projection::Columns(vec![ProjectionColumn {
                            source: Source::single(SourceItem::TableColumn(TableColumn::new(
                                Rc::clone(&users_table),
                                Rc::clone(&user_id_column),
                            )))
                            .into(),
                            alias: Some(SqlIdent::unquoted("id").into())
                        }
                        .into()])
                    );
                }
                Ok(_) => {
                    assert!(false, "Wrong Provenance variant")
                }
                Err(err) => {
                    assert!(false, "Error retrieving Provenance: {:#?}", err)
                }
            },
            Err(err) => {
                assert!(false, "Error during AST evaluation: {:#?}", err)
            }
        }
    }

    #[test]
    fn select_columns_from_subquery() {
        let schema = make_schema! {
            tables: {
                users: {
                    id,
                    email,
                    first_name,
                }
                todo_lists: {
                    id,
                    name,
                    owner_id,
                    created_at,
                    updated_at,
                }
                todo_list_items: {
                    id,
                    description,
                    owner_id,
                    created_at,
                    updated_at,
                }
            }
        };

        let user_id_column = schema
            .resolve_table_column(&SqlIdent::unquoted("users"), &SqlIdent::unquoted("id"))
            .unwrap();
        let todo_list_items_id_column = schema
            .resolve_table_column(
                &SqlIdent::unquoted("todo_list_items"),
                &SqlIdent::unquoted("id"),
            )
            .unwrap();
        let todo_list_items_description_column = schema
            .resolve_table_column(
                &SqlIdent::unquoted("todo_list_items"),
                &SqlIdent::unquoted("description"),
            )
            .unwrap();

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

        let state = ProvenanceState::new(schema);

        match statements.evaluate(&ProvenanceAnalyser::default(), state) {
            Ok(state) => match state.get_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let projection = provenance.projection.deref();

                    assert_eq!(
                        projection,
                        &Projection::Columns(vec![
                            ProjectionColumn {
                                source: Source::single(SourceItem::TableColumn(
                                    user_id_column.clone()
                                ))
                                .into(),
                                alias: Some(SqlIdent::unquoted("user_id").into())
                            }
                            .into(),
                            ProjectionColumn {
                                source: SourceItem::TableColumn(todo_list_items_id_column.clone())
                                    .into(),
                                alias: Some(SqlIdent::unquoted("todo_list_item_id").into())
                            }
                            .into(),
                            ProjectionColumn {
                                source: SourceItem::TableColumn(
                                    todo_list_items_description_column.clone()
                                )
                                .into(),
                                alias: Some(
                                    SqlIdent::unquoted("todo_list_item_description").into()
                                )
                            }
                            .into(),
                        ])
                    );
                }
                Ok(_) => {
                    assert!(false, "Wrong Provenance variant")
                }
                Err(err) => {
                    assert!(false, "Error retrieving Provenance: {:#?}", err)
                }
            },
            Err(err) => {
                assert!(false, "Error during AST evaluation: {:#?}", err)
            }
        }
    }

    #[test]
    fn select_columns_from_correlated_subquery() {
        let schema = make_schema! {
            tables: {
                films: {
                    id,
                    title,
                    length,
                    rating,
                }
            }
        };

        let films_id_column = schema
            .resolve_table_column(&SqlIdent::unquoted("films"), &SqlIdent::unquoted("id"))
            .unwrap();
        let films_title_column = schema
            .resolve_table_column(&SqlIdent::unquoted("films"), &SqlIdent::unquoted("title"))
            .unwrap();
        let films_length_column = schema
            .resolve_table_column(&SqlIdent::unquoted("films"), &SqlIdent::unquoted("length"))
            .unwrap();
        let films_rating_column = schema
            .resolve_table_column(&SqlIdent::unquoted("films"), &SqlIdent::unquoted("rating"))
            .unwrap();

        let statements = parse_sql(
            r#"
            select f.id, f.title, f.length, f.rating
            from films f
            where length > (
                select avg(length)
                from films
                where rating = f.rating
            );
        "#,
        );

        let state = ProvenanceState::new(schema);

        match statements.evaluate(&ProvenanceAnalyser::default(), state) {
            Ok(state) => match state.get_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let projection = provenance.projection.deref();

                    assert_eq!(
                        projection,
                        &Projection::Columns(vec![
                            ProjectionColumn {
                                source: Source::single(SourceItem::TableColumn(
                                    films_id_column.clone()
                                ))
                                .into(),
                                alias: Some(SqlIdent::unquoted("id").into())
                            }
                            .into(),
                            ProjectionColumn {
                                source: Source::single(SourceItem::TableColumn(
                                    films_title_column.clone()
                                ))
                                .into(),
                                alias: Some(SqlIdent::unquoted("title").into())
                            }
                            .into(),
                            ProjectionColumn {
                                source: Source::single(SourceItem::TableColumn(
                                    films_length_column.clone()
                                ))
                                .into(),
                                alias: Some(SqlIdent::unquoted("length").into())
                            }
                            .into(),
                            ProjectionColumn {
                                source: Source::single(SourceItem::TableColumn(
                                    films_rating_column.clone()
                                ))
                                .into(),
                                alias: Some(SqlIdent::unquoted("rating").into())
                            }
                            .into(),
                        ])
                    );
                }
                Ok(_) => {
                    assert!(false, "Wrong Provenance variant")
                }
                Err(err) => {
                    assert!(false, "Error retrieving Provenance: {:#?}", err)
                }
            },
            Err(err) => {
                assert!(false, "Error during AST evaluation: {:#?}", err)
            }
        }
    }

    #[test]
    fn select_columns_from_cte() {
        let schema = make_schema! { name: "public" };

        let statements = parse_sql(
            r#"
                with some_cte as (
                    select 123 as id
                )
                select id from some_cte;
            "#,
        );

        let state = ProvenanceState::new(schema);

        match statements.evaluate(&ProvenanceAnalyser::default(), state) {
            Ok(state) => match state.get_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let projection = provenance.projection.deref();

                    assert_eq!(
                        projection,
                        &Projection::Columns(vec![ProjectionColumn {
                            source: Source::single(SourceItem::Value(Value::Number(
                                BigDecimal::from(123),
                                false
                            )))
                            .into(),
                            alias: Some(SqlIdent::unquoted("id").into())
                        }
                        .into(),])
                    );
                }
                Ok(_) => {
                    assert!(false, "Wrong Provenance variant")
                }
                Err(err) => {
                    assert!(false, "Error retrieving Provenance: {:#?}", err)
                }
            },
            Err(err) => {
                assert!(false, "Error during AST evaluation: {:#?}", err)
            }
        }
    }

    #[test]
    fn basic_insert() {
        let schema = make_schema! {
            tables: {
                films: {
                    id,
                    title,
                    length,
                    rating,
                }
            }
        };

        let statements = parse_sql(
            r#"
            insert into films (title, length, rating)
                values ('Star Wars', '2 hours', '10/10')
                returning id;
        "#,
        );

        let films_id_column = schema
            .resolve_table_column(&SqlIdent::unquoted("films"), &SqlIdent::unquoted("id"))
            .unwrap();

        let state = ProvenanceState::new(schema);

        match statements.evaluate(&ProvenanceAnalyser::default(), state) {
            Ok(state) => match state.get_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Insert(provenance)) => {
                    if let InsertProvenance {
                        into_table,
                        columns_written,
                        returning: Some(projection),
                    } = provenance.deref()
                    {
                        assert_eq!(into_table.name.deref(), &CanonicalIdent::from("films"));

                        assert_eq!(columns_written.len(), 3);
                        assert_eq!(
                            columns_written[0].column.deref(),
                            &CanonicalIdent::from("title")
                        );
                        assert_eq!(
                            columns_written[0].data.deref(),
                            &Source::single(SourceItem::ColumnOfValues)
                        );

                        assert_eq!(
                            columns_written[1].column.deref(),
                            &CanonicalIdent::from("length")
                        );
                        assert_eq!(
                            columns_written[1].data.deref(),
                            &Source::single(SourceItem::ColumnOfValues)
                        );

                        assert_eq!(&*columns_written[2].column, &CanonicalIdent::from("rating"));
                        assert_eq!(
                            columns_written[2].data.deref(),
                            &Source::single(SourceItem::ColumnOfValues)
                        );

                        let projection = projection.deref();

                        assert_eq!(
                            projection,
                            &Projection::Columns(vec![ProjectionColumn {
                                source: Source::single(SourceItem::TableColumn(
                                    films_id_column.clone()
                                ))
                                .into(),
                                alias: Some(SqlIdent::unquoted("id").into())
                            }
                            .into(),])
                        );
                    } else {
                        assert!(false, "expected Some(projection)")
                    }
                }
                Ok(_) => {
                    assert!(false, "Wrong Provenance variant")
                }
                Err(err) => {
                    assert!(false, "Error retrieving Provenance: {:#?}", err)
                }
            },
            Err(err) => {
                assert!(false, "Error during AST evaluation: {:#?}", err)
            }
        }
    }

    #[test]
    fn basic_delete() {
        let schema = make_schema! {
            tables: {
                films: {
                    id,
                    title,
                    length,
                    rating,
                }
            }
        };

        let statements = parse_sql("delete from films where id = 123 returning id;");

        let films_id_column = schema
            .resolve_table_column(&SqlIdent::unquoted("films"), &SqlIdent::unquoted("id"))
            .unwrap();

        let state = ProvenanceState::new(schema);

        match statements.evaluate(&ProvenanceAnalyser::default(), state) {
            Ok(state) => match state.get_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Delete(provenance)) => {
                    if let DeleteProvenance {
                        from_table,
                        returning: Some(projection),
                    } = provenance.deref()
                    {
                        assert_eq!(from_table.name.deref(), &CanonicalIdent::from("films"));

                        let projection = projection.deref();

                        assert_eq!(
                            projection,
                            &Projection::Columns(vec![ProjectionColumn {
                                source: Source::single(SourceItem::TableColumn(
                                    films_id_column.clone()
                                ))
                                .into(),
                                alias: Some(SqlIdent::unquoted("id").into())
                            }
                            .into(),])
                        );
                    } else {
                        assert!(false, "expected Some(projection)")
                    }
                }
                Ok(_) => {
                    assert!(false, "Wrong Provenance variant")
                }
                Err(err) => {
                    assert!(false, "Error retrieving Provenance: {:#?}", err)
                }
            },
            Err(err) => {
                assert!(false, "Error during AST evaluation: {:#?}", err)
            }
        }
    }
}
