use sqltk::{
    nav_visit,
    sqlparser::ast::{Expr, Statement},
    Node, Visitor, VisitorControlFlow,
};

pub struct MyVisitor;

impl<'ast> Visitor<'ast, Expr> for MyVisitor {
    fn enter(&mut self, _node: Node<'ast, Expr>) -> VisitorControlFlow {
        nav_visit()
    }

    fn exit(&mut self, _node: Node<'ast, Expr>) -> VisitorControlFlow {
        nav_visit()
    }
}

impl<'ast> Visitor<'ast, Statement> for MyVisitor {
    fn enter(&mut self, _node: Node<'ast, Statement>) -> VisitorControlFlow {
        nav_visit()
    }

    fn exit(&mut self, _node: Node<'ast, Statement>) -> VisitorControlFlow {
        nav_visit()
    }
}

fn main() {
    println!("Hello, world!");
}
