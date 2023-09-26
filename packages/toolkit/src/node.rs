use crate::AstNode;

pub struct NodeBuilder {
    next_id: usize,
}

impl NodeBuilder {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn new_node<'ast, T: 'ast + AstNode<'ast>>(
        &mut self,
        ast_node: &'ast T,
    ) -> Node<'ast, T> {
        Node::new(self.next_id(), ast_node)
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Node<'ast, T: 'ast + AstNode<'ast> + ?Sized> {
    id: usize,
    ast_node: &'ast T,
}

impl<'ast, T: 'ast + AstNode<'ast>> Node<'ast, T> {
    pub fn new(id: usize, ast_node: &'ast T) -> Self {
        Self { id, ast_node }
    }
}

impl<'ast, T: 'ast + AstNode<'ast>> Clone for Node<'ast, T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            ast_node: self.ast_node,
        }
    }
}