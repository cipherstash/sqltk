use sqlparser::ast::{Expr, Statement};
use sqltk::{nav_visit, Node, Visitor, VisitorControlFlow};

// #[derive(UniversalVisitor)]
// #[universal_visitor(visits(Expr, Statement))]
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
