use std::{error::Error, fmt::Debug};

use crate::Visitable;

/// Trait for producing a new AST node from an existing one by applying edits.
pub trait Transform<'ast> {
    /// The error type returned by this [`Transform`].
    type Error: Error + Debug;

    /// Applies edits to an AST node.
    ///
    /// `original_node` is a read-only reference to the node without any edits applied.
    ///
    /// `new_node` is an owned node to which the edits will be applied.
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
        original_node: &'ast N,
        new_node: N,
    ) -> Result<N, Self::Error>;
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
        T: Transform<'ast>;
}

#[cfg(test)]
mod tests {
    use sqlparser::{
        ast::{Ident, SelectItem},
        dialect::PostgreSqlDialect,
        parser::{Parser, ParserError},
    };

    use super::*;
    use std::convert::Infallible;

    #[test]
    fn direct_invocation_of_transform() {
        pub struct UpcaseStrings;

        impl<'ast> Transform<'ast> for UpcaseStrings {
            type Error = Infallible;

            fn transform<N: Visitable>(
                &mut self,
                _: &'ast N,
                mut new_node: N,
            ) -> Result<N, Self::Error> {
                if let Some(string) = new_node.downcast_mut::<String>() {
                    string.make_ascii_uppercase();
                }
                Ok(new_node)
            }
        }

        let a = String::from("hello");
        assert_eq!(
            UpcaseStrings.transform(&a, a.clone()),
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
                _: &'ast N,
                mut new_node: N,
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
                _: &'ast N,
                mut new_node: N,
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
                _: &'ast N,
                mut new_node: N,
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

    fn parser_for(sql: &str) -> Result<Parser, ParserError> {
        Parser::new(&PostgreSqlDialect {}).try_with_sql(sql)
    }
}
