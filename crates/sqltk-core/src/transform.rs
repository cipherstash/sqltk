use std::{error::Error, fmt::Debug};

/// Trait for producing a new AST node from an existing one by applying edits.
pub trait Transform<'ast> {
    /// An reference to a value of this type is passed to
    /// [`Transform::transform`].  There are no constraints on what type it
    /// can be but it is typically used for storing information about which
    /// edits to apply to particular nodes in the AST.
    type Context;

    /// The error type returned by this [`Transform`].
    type Error: Error + Debug;

    /// Takes ownership of `new_node` and applies zero or more edits and returns
    /// the edited node.  The `original_node` is passed by reference and can be
    /// used to lookup information about what edits to perform from `context`.
    ///
    /// Edits are applied depth-first which means that child nodes of `new_node`
    /// will have already have had their edits applied. It is therefore usually
    /// fine to make modifications to arbitrarily nested child nodes of
    /// `new_node` because no more transformers will be executed on its
    /// children.
    ///
    /// However, nothing prevents a transformer running on the *parent* node of
    /// the current node from making arbitrarily nested modifications that
    /// clobber any edits made by this transformer.
    fn transform<N: 'static + Debug>(
        &self,
        original_node: &'ast N,
        new_node: N,
        context: &Self::Context,
    ) -> Result<N, Self::Error>;
}

/// Trait for applying a [`Transform`] to an AST to produce an edited version.
///
/// Implementations are provided for all AST node types in `sqlparser`.
pub trait ApplyTransform<'ast>
where
    Self: Sized + Debug,
{
    /// Recursively applies `transform` to `self` returning `Ok(Self)` when
    /// successful or `Err(T::Error)` if unsuccessful.
    ///
    /// The `transform` is applied depth-first before finally being applied to
    /// `self`.
    fn apply_transform<T>(
        &'ast self,
        transform: &T,
        context: &T::Context,
    ) -> Result<Self, T::Error>
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
    use std::{any::Any, convert::Infallible};

    #[test]
    fn direct_invocation_of_transform() {
        pub struct UpcaseStrings;

        impl<'ast> Transform<'ast> for UpcaseStrings {
            type Context = ();
            type Error = Infallible;

            fn transform<N: 'static + Debug>(
                &self,
                _original_node: &'ast N,
                mut new_node: N,
                _: &Self::Context,
            ) -> Result<N, Self::Error> {
                if let Some(string) = (&mut new_node as &mut dyn Any).downcast_mut::<String>() {
                    string.make_ascii_uppercase();
                }
                Ok(new_node)
            }
        }

        let a = String::from("hello");
        assert_eq!(
            UpcaseStrings.transform(&a, a.clone(), &()),
            Ok(String::from("HELLO"))
        );
    }

    #[test]
    fn transform_sql_statement() {
        pub struct UpcaseIdents;

        impl<'ast> Transform<'ast> for UpcaseIdents {
            type Context = ();
            type Error = Infallible;

            fn transform<N: 'static + Debug>(
                &self,
                _original_node: &'ast N,
                mut new_node: N,
                _: &Self::Context,
            ) -> Result<N, Self::Error> {
                if let Some(ident) = (&mut new_node as &mut dyn Any).downcast_mut::<Ident>() {
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

        match ast.apply_transform(&UpcaseIdents, &()) {
            Ok(statements) => {
                let statement = statements.get(0).unwrap();
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
            type Context = ();
            type Error = Infallible;

            fn transform<N: 'static + Debug>(
                &self,
                _original_node: &'ast N,
                mut new_node: N,
                _: &Self::Context,
            ) -> Result<N, Self::Error> {
                if let Some(items) =
                    (&mut new_node as &mut dyn Any).downcast_mut::<Vec<SelectItem>>()
                {
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

        match ast.apply_transform(&ReverseProjectionColumns, &()) {
            Ok(statements) => {
                let statement = statements.get(0).unwrap();
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
            type Context = ();
            type Error = Infallible;

            fn transform<N: 'static + Debug>(
                &self,
                _original_node: &'ast N,
                mut new_node: N,
                _: &Self::Context,
            ) -> Result<N, Self::Error> {
                if let Some(items) =
                    (&mut new_node as &mut dyn Any).downcast_mut::<Vec<SelectItem>>()
                {
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

        match ast.apply_transform(&InsertSelectItem, &()) {
            Ok(statements) => {
                let statement = statements.get(0).unwrap();
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
