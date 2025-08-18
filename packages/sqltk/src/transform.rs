use std::any::Any;

use crate::Visitable;

/// Trait for producing a new AST node from an existing one by applying edits.
pub trait Transform<'ast> {
    /// The error type returned by this [`Transform`].
    type Error;

    /// Applies edits to an AST node.
    ///
    /// `target_node` is an owned node to which the edits will be applied. Because AST transformation happens bottom-up,
    /// child nodes of `target_node` may have already been edited - which implies that `&target_node == original_node` does
    /// not hold true in general.
    ///
    /// `original_node` is a read-only reference to the node without any edits applied.
    ///
    /// `node_path` is a slice containing references to all of the ancestors of `original_node`. This is useful for
    /// performing contextual transformations (rather than just type-based) such as transforming an `Expr` in a
    /// projection differently to an `Expr` in a `WHERE` clause.
    ///
    /// Transformations are applied from the leaf nodes towards to the root node. Therefore any edits to child nodes of
    /// `target_node` will have already been applied therefore care must be taken by implementors of this trait to not undo
    /// edits that have already been applied.
    ///
    /// Returns `Ok(target_node)` when transformation is successful, else `Err(Self::Error)`.
    ///
    /// An error during transformation will immediatly terminatate the entire transformation process of the AST.
    fn transform<N: Visitable>(
        &mut self,
        node_path: &NodePath<'ast>,
        target_node: N,
    ) -> Result<N, Self::Error>;

    fn check_postcondition(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Default)]
pub struct NodePath<'ast> {
    node_path: Vec<&'ast dyn Any>,
}

impl<'ast> NodePath<'ast> {
    // Used only in generated code
    #[doc(hidden)]
    pub fn push(&mut self, node: &'ast dyn Any) {
        self.node_path.push(node);
    }

    // Used only in generated code
    #[doc(hidden)]
    pub fn pop(&mut self) {
        self.node_path.pop();
    }

    /// Returns the current node if the path has at least 1 node in it.
    ///
    /// `source_node` is an alias for [`Self::last_1`].
    pub fn source_node_as<N: Visitable>(&self) -> Option<(&'ast N,)> {
        self.last_1_as()
    }

    /// Returns the current node if the path has at least 1 node in it.
    ///
    /// `last_1` is an alias for [`Self::source_node`].
    pub fn last_1_as<N: Visitable>(&self) -> Option<(&'ast N,)> {
        Some((self.nth_last_as(0)?,))
    }

    /// Returns the last 2 nodes in the path. The leftmost node in the tuple is the shallowest ancestor and the
    /// rightmost node is the current node.
    ///
    /// Returns `Some(nodes)` on success, `None` if there are less than 2 nodes in the path.
    pub fn last_2_as<B, A>(&self) -> Option<(&'ast B, &'ast A)>
    where
        B: Visitable,
        A: Visitable,
    {
        Some((self.nth_last_as(1)?, self.nth_last_as(0)?))
    }

    /// Returns the last 3 nodes in the path. The leftmost node in the tuple is the shallowest ancestor and the
    /// rightmost node is the current node.
    ///
    /// Returns `Some(nodes)` on success, `None` if there are less than 3 nodes in the path.
    pub fn last_3_as<C, B, A>(&self) -> Option<(&'ast C, &'ast B, &'ast A)>
    where
        C: Visitable,
        B: Visitable,
        A: Visitable,
    {
        Some((
            self.nth_last_as(2)?,
            self.nth_last_as(1)?,
            self.nth_last_as(0)?,
        ))
    }

    /// Returns the last 4 nodes in the path. The leftmost node in the tuple is the shallowest ancestor and the
    /// rightmost node is the current node.
    ///
    /// Returns `Some(nodes)` on success, `None` if there are less than 4 nodes in the path.
    pub fn last_4_as<D, C, B, A>(&self) -> Option<(&'ast D, &'ast C, &'ast B, &'ast A)>
    where
        D: Visitable,
        C: Visitable,
        B: Visitable,
        A: Visitable,
    {
        Some((
            self.nth_last_as(3)?,
            self.nth_last_as(2)?,
            self.nth_last_as(1)?,
            self.nth_last_as(0)?,
        ))
    }

    /// Returns the last 5 nodes in the path. The leftmost node in the tuple is the shallowest ancestor and the
    /// rightmost node is the current node.
    ///
    /// Returns `Some(nodes)` on success, `None` if there are less than 4 nodes in the path.
    pub fn last_5_as<E, D, C, B, A>(&self) -> Option<(&'ast E, &'ast D, &'ast C, &'ast B, &'ast A)>
    where
        E: Visitable,
        D: Visitable,
        C: Visitable,
        B: Visitable,
        A: Visitable,
    {
        Some((
            self.nth_last_as(4)?,
            self.nth_last_as(3)?,
            self.nth_last_as(2)?,
            self.nth_last_as(1)?,
            self.nth_last_as(0)?,
        ))
    }

    pub fn nth_last_as<N: Visitable>(&self, nth: usize) -> Option<&'ast N> {
        let len = self.node_path.len();
        if len > nth {
            self.node_path
                .get((self.node_path.len() - 1) - nth)
                .and_then(|node| node.downcast_ref::<N>())
        } else {
            None
        }
    }
}

/// Trait for applying a [`Transform`] to an AST to produce an edited version.
///
/// Implementations are provided for all AST node types in `sqltk-parser`.
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
        self.apply_transform_with_path(transform, &mut NodePath::default())
    }

    /// Recursively applies `transform` to `self` returning `Ok(Self)` when
    /// successful or `Err(T::Error)` if unsuccessful.
    ///
    /// The `transform` is applied depth-first before finally being applied to
    /// `self`.
    fn apply_transform_with_path<T>(
        &'ast self,
        transform: &mut T,
        node_path: &mut NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>;
}

#[cfg(test)]
mod tests {
    use sqltk_parser::{
        ast::{Assignment, Expr, Ident, SelectItem},
        dialect::PostgreSqlDialect,
        parser::{Parser, ParserError},
    };

    use crate::NodePath;

    use super::*;
    use std::convert::Infallible;

    #[test]
    fn direct_invocation_of_transform() {
        pub struct UpcaseStrings;

        impl<'ast> Transform<'ast> for UpcaseStrings {
            type Error = Infallible;

            fn transform<N: Visitable>(
                &mut self,
                _: &NodePath<'ast>,
                mut target_node: N,
            ) -> Result<N, Self::Error> {
                if let Some(string) = target_node.downcast_mut::<String>() {
                    string.make_ascii_uppercase();
                }
                Ok(target_node)
            }
        }

        let a = String::from("hello");
        assert_eq!(
            UpcaseStrings.transform(&NodePath::default(), a.clone()),
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
                _: &NodePath<'ast>,
                mut target_node: N,
            ) -> Result<N, Self::Error> {
                if let Some(ident) = target_node.downcast_mut::<Ident>() {
                    ident.value.make_ascii_uppercase();
                }
                Ok(target_node)
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
                _: &NodePath<'ast>,
                mut target_node: N,
            ) -> Result<N, Self::Error> {
                if let Some(items) = target_node.downcast_mut::<Vec<SelectItem>>() {
                    items.reverse();
                }
                Ok(target_node)
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
                _: &NodePath<'ast>,
                mut target_node: N,
            ) -> Result<N, Self::Error> {
                if let Some(items) = target_node.downcast_mut::<Vec<SelectItem>>() {
                    let select_item = parser_for("(now() - users.dob) as age")
                        .unwrap()
                        .parse_select_item()
                        .unwrap();
                    items.insert(1, select_item);
                }
                Ok(target_node)
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
                node_path: &NodePath<'ast>,
                target_node: N,
            ) -> Result<N, Self::Error> {
                if let Some((_assignment, expr)) = node_path.last_2_as::<Assignment, Expr>() {
                    if !matches!(expr, Expr::Value(_)) {
                        return Err(OnlyValuesInUpdate);
                    }
                }

                Ok(target_node)
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
