mod publish_cte;
mod publish_insert;
mod publish_table_factor;

use std::fmt::Debug;

use publish_cte::*;
use publish_insert::*;
use publish_table_factor::*;
use sqlparser::ast::{Expr, Query, SetExpr};
use sqltk::{
    generalise,
    prelude::{Node, Visitable, VisitorControlFlow},
    Visitor, VisitorStack,
};

use crate::{
    model::Annotates, node_path::NodePathOps, model::Projection,
    model::ResolutionError, model::ScopeOps, model::Source,
    SchemaOps,
};

#[derive(Default, Debug)]
pub struct PublishRelationsIntoScope<'ast, State>
where
    State: 'ast
        + Debug
        + ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Query, Projection>
        + NodePathOps<'ast>
        + SchemaOps,
{
    stack: VisitorStack<'ast, State, ResolutionError>,
}

impl<'ast, State: Debug> PublishRelationsIntoScope<'ast, State>
where
    State: 'ast
        + Debug
        + ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Query, Projection>
        + NodePathOps<'ast>
        + SchemaOps,
{
    pub fn new() -> Self {
        let mut stack = VisitorStack::new();
        stack.push(generalise(PublishTableFactor));
        stack.push(generalise(PublishInsert));
        stack.push(generalise(PublishCte));
        Self { stack }
    }
}

impl<'ast, State> Visitor<'ast, State, ResolutionError> for PublishRelationsIntoScope<'ast, State>
where
    State: 'ast
        + Debug
        + ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Query, Projection>
        + NodePathOps<'ast>
        + SchemaOps,
{
    fn enter<N: 'static>(
        &self,
        node: &'ast N,
        state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError>
    where
        &'ast N: Into<Node<'ast>>,
    {
        self.stack.enter(node, state)
    }

    fn exit<N: 'static>(
        &self,
        node: &'ast N,
        state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError>
    where
        &'ast N: Into<Node<'ast>>,
    {
        self.stack.exit(node, state)
    }
}
