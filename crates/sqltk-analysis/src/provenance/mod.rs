//! Types and functions for provenance analysis.

use sqltk::prelude::*;

use crate::{
    annotate_sources::AnnotateSource,
    model::Annotates,
    model::Provenance,
    model::ResolutionError,
    model::ScopeOps,
    model::Source,
    model::{Projection, ProjectionColumn},
    node_path::{NodePath, NodePathOps},
    publish_relations::PublishRelationsIntoScope,
    update_stack::UpdateStack,
    SchemaOps,
};

use std::fmt::Debug;
use std::{convert::Infallible, rc::Rc};
use thiserror::Error;

mod state;

use derive_more::Deref;
pub use state::*;

#[derive(Debug, Deref)]
pub struct ProvenanceAnalyzer<'ast, State: 'ast>
where
    State: Debug
        + ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + Annotates<'ast, Expr, Projection>
        + Annotates<'ast, Query, Projection>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Select, Projection>
        + Annotates<'ast, Statement, Provenance>
        + SchemaOps
        + NodePathOps<'ast>,
{
    stack: VisitorStack<'ast, State, ProvenanceError>,
}

impl<'ast, State> Visitor<'ast, State, ProvenanceError> for ProvenanceAnalyzer<'ast, State>
where
    State: Debug
        + ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + Annotates<'ast, Expr, Projection>
        + Annotates<'ast, Query, Projection>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Select, Projection>
        + Annotates<'ast, Statement, Provenance>
        + SchemaOps
        + NodePathOps<'ast>,
{
    fn enter<N: 'static>(
        &self,
        node: &'ast N,
        state: State,
    ) -> VisitorControlFlow<'ast, State, ProvenanceError>
    where
        &'ast N: Into<Node<'ast>>,
    {
        self.stack.enter(node, state)
    }

    fn exit<N: 'static>(
        &self,
        node: &'ast N,
        state: State,
    ) -> VisitorControlFlow<'ast, State, ProvenanceError>
    where
        &'ast N: Into<Node<'ast>>,
    {
        self.stack.exit(node, state)
    }
}

impl<'ast, State: 'ast> ProvenanceAnalyzer<'ast, State>
where
    State: Debug
        + ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + Annotates<'ast, Expr, Projection>
        + Annotates<'ast, Query, Projection>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Select, Projection>
        + Annotates<'ast, Statement, Provenance>
        + SchemaOps
        + NodePathOps<'ast>,
{
    pub fn new() -> Self {
        let mut stack = VisitorStack::<State, ProvenanceError>::new();
        stack.push(NodePath::track());

        // NOTE: uncomment the two lines below for (very noisy) debugging.
        // #[cfg(test)]
        // stack.push(NodePath::log_top_entry());

        #[cfg(test)]
        stack.push(AnyNode::on_enter(|_, state: State| {
            if let Some(entry) = state.peek_path_entry() {
                eprintln!("{:indent$}ENTER: {}", "", entry, indent = entry.depth);
                // state.dump_scope();
            }
            flow::infallible::cont(state)
        }));

        stack.push(UpdateStack);

        stack.push(PublishRelationsIntoScope::new());

        stack.push(AnnotateSource::annotate_expr_with_source());
        stack.push(AnnotateSource::annotate_set_expr_with_projection());
        stack.push(AnnotateSource::annotate_select_with_projection());
        stack.push(AnnotateSource::annotate_query_with_projection());
        stack.push(AnnotateSource::annotate_select_item_with_source());
        stack.push(AnnotateSource::annotate_statement_with_projection());

        #[cfg(test)]
        stack.push(AnyNode::on_exit(|_, state: State| {
            if let Some(entry) = state.peek_path_entry() {
                eprintln!("{:indent$}EXIT: {}", "", entry, indent = entry.depth);
                // state.dump_scope();
            }
            flow::infallible::cont(state)
        }));

        Self { stack }
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
    use std::rc::Rc;

    use crate::{
        make_schema,
        model::Annotates,
        model::Provenance,
        model::{CanonicalIdent, SqlIdent, Table},
        model::{InsertProvenance, SelectProvenance},
        model::{Projection, ProjectionColumn},
        model::{Source, SourceItem, TableColumn},
        ProvenanceAnalyzer,
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

        let visitor = ProvenanceAnalyzer::new();

        match statements.evaluate(&visitor, state) {
            Ok(state) => match state.expect_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let SelectProvenance { projection } = provenance.deref();
                    assert_eq!(
                        projection.deref(),
                        &Projection {
                            columns: vec![ProjectionColumn {
                                source: Source::single(SourceItem::TableColumn(
                                    user_id_column.clone()
                                ))
                                .into(),
                                alias: Some(SqlIdent::canonical("id").into())
                            }
                            .into()]
                        }
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

        let visitor = ProvenanceAnalyzer::new();

        match statements.evaluate(&visitor, state) {
            Ok(state) => match state.expect_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let SelectProvenance { projection } = provenance.deref();
                    assert_eq!(projection.deref().columns.len(), 1);
                    assert_eq!(
                        projection.deref(),
                        &Projection {
                            columns: vec![ProjectionColumn {
                                source: Source::single(SourceItem::TableColumn(TableColumn::new(
                                    Rc::clone(&users_table),
                                    Rc::clone(&user_id_column),
                                )))
                                .into(),
                                alias: None,
                            }
                            .into()]
                        }
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

        let visitor = ProvenanceAnalyzer::new();

        match statements.evaluate(&visitor, state) {
            Ok(state) => match state.expect_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let SelectProvenance { projection } = provenance.deref();
                    assert_eq!(projection.columns.len(), 3);

                    assert_eq!(
                        &projection.columns[0].source,
                        &Rc::new(Source::single(SourceItem::TableColumn(
                            user_id_column.clone()
                        )))
                    );

                    assert_eq!(
                        &projection.columns[1].source,
                        &Rc::new(Source::single(SourceItem::TableColumn(
                            todo_list_items_id_column.clone()
                        )))
                    );

                    assert_eq!(
                        &projection.columns[2].source,
                        &Rc::new(Source::single(SourceItem::TableColumn(
                            todo_list_items_description_column.clone()
                        )))
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

        let visitor = ProvenanceAnalyzer::new();

        match statements.evaluate(&visitor, state) {
            Ok(state) => match state.expect_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let SelectProvenance { projection } = provenance.deref();
                    assert_eq!(projection.columns.len(), 4);

                    assert_eq!(
                        &projection.columns[0].source,
                        &Rc::new(Source::single(SourceItem::TableColumn(
                            films_id_column.clone()
                        )))
                    );

                    assert_eq!(
                        &projection.columns[1].source,
                        &Rc::new(Source::single(SourceItem::TableColumn(
                            films_title_column.clone()
                        )))
                    );

                    assert_eq!(
                        &projection.columns[2].source,
                        &Rc::new(Source::single(SourceItem::TableColumn(
                            films_length_column.clone()
                        )))
                    );

                    assert_eq!(
                        &projection.columns[3].source,
                        &Rc::new(Source::single(SourceItem::TableColumn(
                            films_rating_column.clone()
                        )))
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

        let visitor = ProvenanceAnalyzer::new();

        match statements.evaluate(&visitor, state) {
            Ok(state) => match state.expect_annotation(&statements[0]).as_deref() {
                Ok(Provenance::Select(provenance)) => {
                    let SelectProvenance { projection } = provenance.deref();
                    assert_eq!(projection.columns.len(), 1);

                    assert_eq!(
                        &projection.columns[0].source,
                        &Rc::new(Source::single(SourceItem::Value(Value::Number(
                            BigDecimal::from(123),
                            false
                        ))))
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

        let visitor = ProvenanceAnalyzer::new();

        match statements.evaluate(&visitor, state) {
            Ok(state) => match state.expect_annotation(&statements[0]).as_deref() {
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

                        assert_eq!(projection.columns.len(), 1);

                        assert_eq!(
                            &projection.columns[0].source,
                            &Rc::new(Source::single(SourceItem::TableColumn(
                                films_id_column.clone()
                            )))
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
