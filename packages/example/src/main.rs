use sqlparser::ast::{Expr, Statement};
use sqltk::{Visitor, VisitorControlFlow, nav_visit, Node};

// #[derive(UniversalVisitor)]
// #[universal_visitor(visits(Expr, Statement))]
pub struct MyVisitor;


impl<'ast> Visitor<'ast, Expr> for MyVisitor {
    fn enter(&self, _node: Node<'ast, Expr>) -> VisitorControlFlow {
        nav_visit()
    }

    fn exit(&self, _node: Node<'ast, Expr>) -> VisitorControlFlow {
        nav_visit()
    }
}

impl<'ast> Visitor<'ast, Statement> for MyVisitor {
    fn enter(&self, _node: Node<'ast, Statement>) -> VisitorControlFlow {
        nav_visit()
    }

    fn exit(&self, _node: Node<'ast, Statement>) -> VisitorControlFlow {
        nav_visit()
    }
}

fn main() {
    println!("Hello, world!");
}

