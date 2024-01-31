use crate::{Visitable, DisplayType};
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

    pub fn next_node<'ast, T: Visitable<'ast>>(&mut self, visitable: &'ast T) -> Node<'ast, T> {
        Node::new(self.next_id(), visitable)
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

// TODO: better name for Node: Tagged ?
// TODO: Then rename Visitable => Node ?
#[derive(Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Node<'ast, T: Visitable<'ast> + ?Sized> {
    id: usize,
    visitable: &'ast T,
}

impl<'ast, T: Visitable<'ast>> Node<'ast, T> {
    pub fn new(id: usize, visitable: &'ast T) -> Self {
        Self { id, visitable }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn inner(&self) -> &'ast T {
        self.visitable
    }
}

impl<'ast, T: Visitable<'ast>> Deref for Node<'ast, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.visitable
    }
}

impl<'ast, T: Visitable<'ast>> Clone for Node<'ast, T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            visitable: self.visitable,
        }
    }
}

impl<'ast, T: Visitable<'ast>> Display for Node<'ast, T>
where
    DisplayType<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (ID: {})", DisplayType::<T>::new(), self.id)
    }
}
