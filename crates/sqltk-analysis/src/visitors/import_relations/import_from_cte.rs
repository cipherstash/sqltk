use std::ops::Deref;

use sqlparser::ast::{Cte, Expr, Query, TableAlias};
use sqltk::{flow, SpecializedVisitor, VisitorControlFlow};

use crate::{
    model::Annotates,
    model::Projection,
    model::ResolutionError,
    model::ScopeOps,
    model::{NamedRelation, Source},
    model::{SqlIdent, UnquotedIdent},
    SchemaOps,
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct ImportFromCte;

impl<'ast, State: 'ast> SpecializedVisitor<'ast, Cte, State, ResolutionError> for ImportFromCte
where
    State: ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, Query, Projection>
        + SchemaOps,
{
    fn exit(
        &self,
        node: &'ast Cte,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        let Cte {
            alias: TableAlias {
                name: alias,
                columns,
            },
            query,
            ..
        } = node;

        if !columns.is_empty() {
            return flow::error(ResolutionError::Unimplemented, state);
        }

        match state.expect_annotation(query.deref()) {
            Ok(projection) => match state.add_relation(
                NamedRelation {
                    name: SqlIdent::Unquoted(UnquotedIdent::new(alias.value.clone())).into(),
                    projection: projection.clone(),
                }
                .into(),
            ) {
                Ok(_) => flow::cont(state),
                Err(err) => flow::error(err, state),
            },
            Err(err) => flow::error(err.into(), state),
        }
    }
}
