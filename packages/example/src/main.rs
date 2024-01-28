use std::ops::ControlFlow;

use sqltk::{
    sqlparser::ast::{Expr, Statement},
    EnterControlFlow, ExitControlFlow, Navigation, Node, Visitor,
};

pub struct MyVisitor;

impl<'ast> Visitor<'ast, Expr> for MyVisitor {
    fn enter(&mut self, _node: Node<'ast, Expr>) -> EnterControlFlow {
        ControlFlow::Continue(Navigation::Visit)
    }

    fn exit(&mut self, _node: Node<'ast, Expr>) -> ExitControlFlow {
        ControlFlow::Continue(())
    }
}

impl<'ast> Visitor<'ast, Statement> for MyVisitor {
    fn enter(&mut self, _node: Node<'ast, Statement>) -> EnterControlFlow {
        ControlFlow::Continue(Navigation::Visit)
    }

    fn exit(&mut self, _node: Node<'ast, Statement>) -> ExitControlFlow {
        ControlFlow::Continue(())
    }
}

fn main() {
    println!("Hello, world!");
}
