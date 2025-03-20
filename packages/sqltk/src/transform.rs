use std::any::Any;

use crate::Visitable;

/// Trait for producing a new AST node from an existing one by applying edits.
pub trait Transform<'ast> {
    /// The error type returned by this [`Transform`].
    type Error;

    /// Applies edits to an AST node.
    ///
    /// `new_node` is an owned node to which the edits will be applied. Because AST transformation happens bottom-up,
    /// child nodes of `new_node` may have already been edited - which implies that `&new_node == original_node` does
    /// not hold true in general.
    ///
    /// `original_node` is a read-only reference to the node without any edits applied.
    ///
    /// `context` is a slice containing references to all of the ancestors of `original_node`. This is useful for
    /// performing contextual transformations (rather than just type-based) such as transforming an `Expr` in a
    /// projection differently to an `Expr` in a `WHERE` clause.
    ///
    /// Transformations are applied from the leaf nodes towards to the root node. Therefore any edits to child nodes of
    /// `new_node` will have already been applied therefore care must be taken by implementors of this trait to not undo
    /// edits that have already been applied.
    ///
    /// Returns `Ok(new_node)` when transformation is successful, else `Err(Self::Error)`.
    ///
    /// An error during transformation will immediatly terminatate the entire transformation process of the AST.
    fn transform<N: Visitable>(
        &mut self,
        new_node: N,
        original_node: &'ast N,
        context: &Context<'ast>,
    ) -> Result<N, Self::Error>;
}

#[derive(Default)]
pub struct Context<'ast> {
    context: Vec<&'ast dyn Any>,
}

impl<'ast> Context<'ast> {
    // Used only in generated code
    #[doc(hidden)]
    pub fn push(&mut self, node: &'ast dyn Any) {
        self.context.push(node);
    }

    // Used only in generated code
    #[doc(hidden)]
    pub fn pop(&mut self) {
        self.context.pop();
    }

    pub fn nth_last_as<T: 'static>(&self, nth: usize) -> Option<&'ast T> {
        if self.context.is_empty() {
            None
        } else {
            self.context
                .get(self.context.len() - 1 - nth)
                .and_then(|node| node.downcast_ref::<T>())
        }
    }

    pub fn last_as<T: 'static>(&self) -> Option<&'ast T> {
        self.nth_last_as(0)
    }
}

/// Trait for applying a [`Transform`] to an AST to produce an edited version.
///
/// Implementations are provided for all AST node types in `sqlparser`.
pub trait Transformable<'ast>
where
    Self: Sized,
{
    /// Recursively applies `transform` to `self` returning `Ok(Self)` when
    /// successful or `Err(T::Error)` if unsuccessful.
    ///
    /// The `transform` is applied depth-first before finally being applied to
    /// `self`.
    fn apply_transform<T>(&'ast self, transform: &mut T) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        self.apply_transform_with_path(transform, &mut Context::default())
    }

    /// Recursively applies `transform` to `self` returning `Ok(Self)` when
    /// successful or `Err(T::Error)` if unsuccessful.
    ///
    /// The `transform` is applied depth-first before finally being applied to
    /// `self`.
    fn apply_transform_with_path<T>(
        &'ast self,
        transform: &mut T,
        context: &mut Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>;
}

#[cfg(test)]
mod tests {
    use sqlparser::{
        ast::{Assignment, Expr, Ident, SelectItem},
        dialect::PostgreSqlDialect,
        parser::{Parser, ParserError},
    };

    use crate::Context;

    use super::*;
    use std::convert::Infallible;

    #[test]
    fn direct_invocation_of_transform() {
        pub struct UpcaseStrings;

        impl<'ast> Transform<'ast> for UpcaseStrings {
            type Error = Infallible;

            fn transform<N: Visitable>(
                &mut self,
                mut new_node: N,
                _: &'ast N,
                _: &Context<'ast>,
            ) -> Result<N, Self::Error> {
                if let Some(string) = new_node.downcast_mut::<String>() {
                    string.make_ascii_uppercase();
                }
                Ok(new_node)
            }
        }

        let a = String::from("hello");
        assert_eq!(
            UpcaseStrings.transform(a.clone(), &a, &Context::default()),
            Ok(String::from("HELLO"))
        );
    }

    #[test]
    fn transform_sql_statement() {
        pub struct UpcaseIdents;

        impl<'ast> Transform<'ast> for UpcaseIdents {
            type Error = Infallible;

            fn transform<N: Visitable>(
                &mut self,
                mut new_node: N,
                _: &'ast N,
                _: &Context<'ast>,
            ) -> Result<N, Self::Error> {
                if let Some(ident) = new_node.downcast_mut::<Ident>() {
                    ident.value.make_ascii_uppercase();
                }
                Ok(new_node)
            }
        }

        let ast = Parser::parse_sql(
            &PostgreSqlDialect {},
            "SELECT users.name, users.email FROM users;",
        )
        .unwrap();

        match ast.apply_transform(&mut UpcaseIdents) {
            Ok(statements) => {
                let statement = statements.first().unwrap();
                assert_eq!(
                    statement.to_string(),
                    "SELECT USERS.NAME, USERS.EMAIL FROM USERS"
                );
            }
            Err(err) => panic!("Oops: {:?}", err),
        }
    }

    #[test]
    fn transform_sql_statement_reverse_vec() {
        pub struct ReverseProjectionColumns;

        impl<'ast> Transform<'ast> for ReverseProjectionColumns {
            type Error = Infallible;

            fn transform<N: Visitable>(
                &mut self,
                mut new_node: N,
                _: &'ast N,
                _: &Context<'ast>,
            ) -> Result<N, Self::Error> {
                if let Some(items) = new_node.downcast_mut::<Vec<SelectItem>>() {
                    items.reverse();
                }
                Ok(new_node)
            }
        }

        let ast = Parser::parse_sql(
            &PostgreSqlDialect {},
            "SELECT users.name, users.email FROM users;",
        )
        .unwrap();

        match ast.apply_transform(&mut ReverseProjectionColumns) {
            Ok(statements) => {
                let statement = statements.first().unwrap();
                assert_eq!(
                    statement.to_string(),
                    "SELECT users.email, users.name FROM users",
                );
            }
            Err(err) => panic!("Oops: {:?}", err),
        }
    }

    #[test]
    fn transform_sql_statement_add_select_item() {
        pub struct InsertSelectItem;

        impl<'ast> Transform<'ast> for InsertSelectItem {
            type Error = Infallible;

            fn transform<N: Visitable>(
                &mut self,
                mut new_node: N,
                _: &'ast N,
                _: &Context<'ast>,
            ) -> Result<N, Self::Error> {
                if let Some(items) = new_node.downcast_mut::<Vec<SelectItem>>() {
                    let select_item = parser_for("(now() - users.dob) as age")
                        .unwrap()
                        .parse_select_item()
                        .unwrap();
                    items.insert(1, select_item);
                }
                Ok(new_node)
            }
        }

        let ast = Parser::parse_sql(
            &PostgreSqlDialect {},
            "SELECT users.name, users.email FROM users;",
        )
        .unwrap();

        match ast.apply_transform(&mut InsertSelectItem) {
            Ok(statements) => {
                let statement = statements.first().unwrap();
                assert_eq!(
                    statement.to_string(),
                    "SELECT users.name, (now() - users.dob) AS age, users.email FROM users",
                );
            }
            Err(err) => panic!("Oops: {:?}", err),
        }
    }

    #[test]
    fn context_sensitive_transformation() {
        pub struct AssertOnlyValueExprsInUpdate;

        pub struct OnlyValuesInUpdate;

        impl<'ast> Transform<'ast> for AssertOnlyValueExprsInUpdate {
            type Error = OnlyValuesInUpdate;

            fn transform<N: Visitable>(
                &mut self,
                new_node: N,
                original_node: &'ast N,
                context: &Context<'ast>,
            ) -> Result<N, Self::Error> {
                // We are looking for Expr nodes that are immediate children of Assignment nodes. The following asserts
                // that the parent node is an Assignment and the original node is Expr.
                if let (Some(_), Some(expr)) = (
                    context.last_as::<Assignment>(),
                    original_node.downcast_ref::<Expr>(),
                ) {
                    if !matches!(expr, Expr::Value(_)) {
                        return Err(OnlyValuesInUpdate);
                    }
                }

                Ok(new_node)
            }
        }

        let ast = Parser::parse_sql(
            &PostgreSqlDialect {},
            "UPDATE users SET name = 'Bob' where name = 'BOB'",
        )
        .unwrap();

        assert!(ast
            .apply_transform(&mut AssertOnlyValueExprsInUpdate)
            .is_ok());

        let ast = Parser::parse_sql(
            &PostgreSqlDialect {},
            "UPDATE users SET name = 'Bob' || ' Smith' WHERE name = 'BOB'",
        )
        .unwrap();

        assert!(ast
            .apply_transform(&mut AssertOnlyValueExprsInUpdate)
            .is_err());
    }

    fn parser_for(sql: &str) -> Result<Parser, ParserError> {
        Parser::new(&PostgreSqlDialect {}).try_with_sql(sql)
    }
}
