use crate::{AstNode, DisplayType};
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

// TODO: better name: Tagger ?
pub struct NodeIdSequence {
    next_id: usize,
}

impl Default for NodeIdSequence {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeIdSequence {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn next_node<'ast, T: AstNode<'ast>>(&mut self, ast_node: &'ast T) -> Node<'ast, T> {
        Node::new(self.next_id(), ast_node)
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

// TODO: better name for Node: Tagged ?
// TODO: Then rename AstNode => Node ?
#[derive(Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Node<'ast, T: AstNode<'ast> + ?Sized> {
    id: usize,
    ast_node: &'ast T,
}

impl<'ast, T: AstNode<'ast>> Node<'ast, T> {
    pub fn new(id: usize, ast_node: &'ast T) -> Self {
        Self { id, ast_node }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn inner(&self) -> &'ast T {
        self.ast_node
    }
}

impl<'ast, T: AstNode<'ast>> Deref for Node<'ast, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ast_node
    }
}

impl<'ast, T: AstNode<'ast>> Clone for Node<'ast, T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            ast_node: self.ast_node,
        }
    }
}

impl<'ast, T: AstNode<'ast>> Display for Node<'ast, T>
where
    DisplayType<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (ID: {})", DisplayType::<T>::new(), self.id)
    }
}
