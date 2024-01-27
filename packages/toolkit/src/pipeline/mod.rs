//! # Pipelines
//!
//! A [`Pipeline`] allows composition of multiple [`crate::Visitor`]
//! implementations to facilitate build sophisticated AST analysis and
//! transformation workflows from small, isolated and unit testable pieces.
//!
//! All `Pipeline` implementations are also [`VisitorDispatch`] implementations
//! so that they can be passed to [`crate::AstNode::accept`].
//!
//! ## Building a pipeline
//!
//! A `Pipeline` is created by calling [`build`] which is passed a tuple of
//! [`Stage`] implementations as a generic parameter.
//!

mod dispatch_impls;
mod pipeline_impls;
mod scope;
mod stage;
mod tuples_of_stages;

pub use scope::*;
pub use stage::*;
pub use tuples_of_stages::*;

use crate::VisitorDispatch;
use std::marker::PhantomData;

pub trait PipelineInit<'ast, 'scope: 'ast>
where
    Self: Sized,
{
    fn new(scope: RootScope) -> Result<Self, InitializeError>;
}

pub trait PipelineDispatch<'ast, 'scope: 'ast>: VisitorDispatch<'ast> {}

pub struct Pipeline<'ast, 'scope: 'ast, Stages: TupleOfStages<'ast>> {
    stages: Stages,
    scope: RootScope,
    _ast: PhantomData<&'ast ()>,
    _scope: PhantomData<&'scope ()>,
}

impl<'ast, 'scope: 'ast, Stages: TupleOfStages<'ast>> Pipeline<'ast, 'scope, Stages> {
    pub fn get<T: 'static>(&self) -> Result<ReadOnly<T>, UnknownItemError> {
        self.scope.get::<T>()
    }
}

pub fn build<'ast, 'scope: 'ast, Stages: TupleOfStages<'ast>>(
    scope: RootScope,
) -> Result<Pipeline<'ast, 'scope, Stages>, InitializeError>
where
    Pipeline<'ast, 'scope, Stages>: PipelineInit<'ast, 'scope> + PipelineDispatch<'ast, 'scope>,
{
    Pipeline::<'ast, 'scope, Stages>::new(scope)
}

#[cfg(test)]
mod test {
    use sqlparser::{ast::Expr, dialect::GenericDialect, parser::Parser};
    use sqltk_derive::VisitorDispatch;

    use crate::{
        dispatch::AssumeNotImplemented,
        nav_visit,
        pipeline::{self, InitializeError, ReadOnly, ReadWrite, RootScope, Scope, Stage},
        AstNode, Node, Visitor, VisitorControlFlow,
    };

    #[test]
    fn basic() {
        #[derive(Debug, Eq, PartialEq)]
        struct ExprsBalanced(bool);

        #[derive(VisitorDispatch)]
        struct BalancedExprsCheck {
            expr_enter_count: ReadOnly<ExprEnterCount>,
            expr_exit_count: ReadOnly<ExprExitCount>,
            exprs_balanced: ReadWrite<ExprsBalanced>,
        }

        impl<'ast, 'scope> Stage<'ast, 'scope> for BalancedExprsCheck {
            fn init_exit(scope: &mut impl Scope<'scope>) -> Result<Self, InitializeError> {
                scope
                    .import::<ExprEnterCount>()
                    .import::<ExprExitCount>()
                    .export(ExprsBalanced(true))
                    .resolve()
                    .map(|(expr_enter_count, expr_exit_count, exprs_balanced)| Self {
                        expr_enter_count,
                        expr_exit_count,
                        exprs_balanced,
                    })
                    .map_err(|_| InitializeError)
            }
        }

        impl<'ast> Visitor<'ast, Expr> for BalancedExprsCheck {
            fn enter(&mut self, _: Node<'ast, Expr>) -> VisitorControlFlow {
                self.exprs_balanced.get_mut().0 = false;
                nav_visit()
            }

            fn exit(&mut self, _: Node<'ast, Expr>) -> VisitorControlFlow {
                self.exprs_balanced.get_mut().0 =
                    self.expr_enter_count.get().0 == self.expr_exit_count.get().0;
                nav_visit()
            }
        }

        struct ExprEnterCount(usize);
        struct ExprExitCount(usize);

        #[derive(VisitorDispatch)]
        struct ExprCounter {
            expr_enter_count: ReadWrite<ExprEnterCount>,
            expr_exit_count: ReadWrite<ExprExitCount>,
        }

        impl<'ast, 'scope> Stage<'ast, 'scope> for ExprCounter {
            fn init_enter(scope: &mut impl Scope<'scope>) -> Result<(), InitializeError> {
                scope.export(ExprEnterCount(0)).export(ExprExitCount(0));

                Ok(())
            }

            fn init_exit(scope: &mut impl Scope<'scope>) -> Result<Self, InitializeError> {
                scope
                    .import_owned::<ExprEnterCount>()
                    .import_owned::<ExprExitCount>()
                    .resolve()
                    .map(|(expr_enter_count, expr_exit_count)| Self {
                        expr_enter_count,
                        expr_exit_count,
                    })
                    .map_err(|_| InitializeError)
            }
        }

        impl<'ast> Visitor<'ast, Expr> for ExprCounter {
            fn enter(&mut self, _: Node<'ast, Expr>) -> VisitorControlFlow {
                self.expr_enter_count.get_mut().0 += 1;
                nav_visit()
            }

            fn exit(&mut self, _: Node<'ast, Expr>) -> VisitorControlFlow {
                self.expr_exit_count.get_mut().0 += 1;
                nav_visit()
            }
        }

        if let Ok(mut pipeline) =
            pipeline::build::<(BalancedExprsCheck, ExprCounter)>(RootScope::default())
        {
            let dialect = GenericDialect {};

            let sql = "SELECT a, b, 123, myfunc(b) \
                       FROM table_1 \
                       WHERE a > b AND b < 100 \
                       ORDER BY a DESC, b";

            let ast = Parser::parse_sql(&dialect, sql).unwrap();

            ast.accept(&mut pipeline);

            if let Ok(expr_balanced) = pipeline.get::<ExprsBalanced>() {
                assert_eq!(*expr_balanced.get(), ExprsBalanced(true));
            } else {
                assert!(false, "Could not read result from scope")
            }
        } else {
            assert!(false, "Pipeline construction failed")
        };
    }
}
