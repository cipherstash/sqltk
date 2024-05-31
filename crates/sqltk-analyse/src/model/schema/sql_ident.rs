use core::mem;
use std::hash::{Hash, Hasher};
use std::{fmt::Debug, ops::Deref, rc::Rc};

use derive_more::Display;
use sqlparser::ast::Ident;
use unicase::UniCase;

/// Syntactically, SQL identifiers come in two forms: quoted and unquoted. An
/// additional third kind of identifier is a "canonical" (non-syntactic)
/// identifier, e.g. one from the database itself.
///
/// The `SqlIdent` enum represents all three forms.
///
/// All identifiers loaded from a database [`crate::model::schema::Schema`] are
/// modelled as canonical.  Identifiers in SQL statements are either quoted or
/// unquoted: never canonical.
///
/// For an "official" explanation of how SQL identifiers work (at least with
/// respect to Postgres), see
/// [https://www.postgresql.org/docs/14/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS].
///
/// SQL is wild, hey!
#[derive(Debug, Clone, Eq, PartialOrd, Ord, Display)]
pub enum SqlIdent {
    /// A identifier without quotes.
    ///
    /// Compared case-insensitively with schema identifiers and identifiers
    /// defined in SQL statements such as projection and column aliases.
    #[display(fmt = "{}", _0)]
    Unquoted(UnquotedIdent),

    /// A identifier with quotes. The `String` is the identifier without quotes,
    /// The specific quote character used is not part of the variant.
    ///
    /// Compared case-sensitively with schema identifiers and identifiers
    /// defined in SQL statements such as projection and column aliases.
    #[display(fmt = "\"{}\"", _0)]
    Quoted(String),

    /// An identifier from the database.
    ///
    /// It is always case-sensitive and could be the name of a schema, table,
    /// view, column etc.
    ///
    /// Note that the `String` can be a mixed case string may contain whitespace
    /// so if used in a SQL statement might require quoting.
    Canonical(CanonicalIdent),
}

// This manual Hash implementation is required to prevent a clippy error:
// "error: you are deriving `Hash` but have implemented `PartialEq` explicitly"
impl Hash for SqlIdent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
        match self {
            SqlIdent::Unquoted(x) => x.hash(state),
            SqlIdent::Quoted(x) => x.hash(state),
            SqlIdent::Canonical(x) => x.hash(state),
        }
    }
}

#[derive(Debug, Display, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct UnquotedIdent {
    #[display(fmt = "{}", _0)]
    inner: UniCase<String>,
}

impl UnquotedIdent {
    pub fn new(ident: String) -> Self {
        Self {
            inner: UniCase::<String>::new(ident),
        }
    }
}

impl From<&str> for UnquotedIdent {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}

impl From<&Ident> for UnquotedIdent {
    fn from(ident: &Ident) -> Self {
        Self::new(ident.value.clone())
    }
}

#[derive(Debug, Display, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct QuotedIdent {
    #[display(fmt = "{}", _0)]
    inner: String,
}

impl PartialEq for SqlIdent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SqlIdent::Unquoted(me), SqlIdent::Unquoted(other)) => me == other,
            (SqlIdent::Unquoted(me), SqlIdent::Quoted(other)) => {
                me.inner == UniCase::<&str>::from(other.as_str())
            }
            (SqlIdent::Unquoted(_), SqlIdent::Canonical(other)) => self == other,
            (SqlIdent::Quoted(me), SqlIdent::Unquoted(other)) => {
                other.inner == UniCase::<&str>::from(me.as_str())
            }
            (SqlIdent::Quoted(me), SqlIdent::Quoted(other)) => other == me,
            (SqlIdent::Quoted(me), SqlIdent::Canonical(other)) => &other.0 == me,
            (SqlIdent::Canonical(me), SqlIdent::Unquoted(other)) => {
                UniCase::<&str>::from(me.0.as_str()) == other.inner
            }
            (SqlIdent::Canonical(me), SqlIdent::Quoted(other)) => other == &me.0,
            (SqlIdent::Canonical(me), SqlIdent::Canonical(other)) => other == me,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
pub struct CanonicalIdent(pub String);

impl From<String> for CanonicalIdent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Ident> for CanonicalIdent {
    fn from(value: Ident) -> Self {
        Self(value.value.clone())
    }
}

impl From<&str> for CanonicalIdent {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl SqlIdent {
    /// Convenience for constructing a `SqlIdent::Canonical(_)` from a `&str`.
    pub fn canonical(ident: &str) -> Self {
        SqlIdent::Canonical(ident.into())
    }

    /// Convenience for constructing a `SqlIdent::Quoted(_)` from a `&str`.
    pub fn quoted(ident: &str) -> Self {
        SqlIdent::Quoted(ident.into())
    }

    /// Convenience for constructing a `SqlIdent::Unquoted(_)` from a `&str`.
    pub fn unquoted(ident: &str) -> Self {
        SqlIdent::Unquoted(ident.into())
    }

    /// Finds a unique `SqlIdent` (the needle) in an [`Iterator<Item = &T`>]
    /// (the haystack) where `T` is implements [`Identifiable`].
    ///
    /// Returns `Ok(Some(target))` if a single unique match is found.
    /// Returns `Ok(None)` if no match is found.
    /// Returns `Err(AmbiguousMatchError)` if multiple matches are found.
    pub fn try_find_unique<T, I>(
        needle: &SqlIdent,
        haystack: &mut I,
    ) -> Result<Option<Rc<T>>, AmbiguousMatchError>
    where
        T: 'static + Named + Debug,
        I: Iterator<Item = Rc<T>>,
    {
        match (
            haystack.find(|item| item.name().eq(needle)),
            haystack.find(|item| item.name().eq(needle)),
        ) {
            (Some(result), None) => Ok(Some(Rc::clone(&result))),
            (Some(_), Some(_)) => Err(AmbiguousMatchError(needle.clone())),
            _ => Ok(None),
        }
    }

    /// Like [`Self::try_find_unique`] but it is also an error when no match is
    /// found at all.
    pub fn find_unique<T, I>(
        needle: &SqlIdent,
        haystack: &mut I,
    ) -> Result<Rc<T>, FindUniqueMatchError>
    where
        T: 'static + Named + Debug,
        I: Iterator<Item = Rc<T>>,
    {
        match Self::try_find_unique(needle, haystack) {
            Ok(Some(found)) => Ok(found),
            Ok(None) => Err(FindUniqueMatchError::NotFound(needle.clone())),
            Err(AmbiguousMatchError(_)) => Err(FindUniqueMatchError::Ambiguous(needle.clone())),
        }
    }
}

#[derive(
    Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, derive_more::Display,
)]
pub struct AmbiguousMatchError(pub SqlIdent);

#[derive(Debug, thiserror::Error, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum FindUniqueMatchError {
    #[error("Ambiguous identifier match (multiple matches in scope)")]
    Ambiguous(SqlIdent),

    #[error("Identifier not found in scope")]
    NotFound(SqlIdent),
}

impl From<CanonicalIdent> for SqlIdent {
    fn from(value: CanonicalIdent) -> Self {
        SqlIdent::Canonical(value)
    }
}

impl From<sqlparser::ast::Ident> for SqlIdent {
    fn from(ident: sqlparser::ast::Ident) -> Self {
        if ident.quote_style.is_some() {
            Self::Quoted(ident.value)
        } else {
            Self::Unquoted(UnquotedIdent::from(ident.value.as_str()))
        }
    }
}

impl From<&sqlparser::ast::Ident> for SqlIdent {
    fn from(ident: &sqlparser::ast::Ident) -> Self {
        if ident.quote_style.is_some() {
            Self::Quoted(ident.value.clone())
        } else {
            Self::Unquoted(UnquotedIdent::from(ident.value.as_str()))
        }
    }
}

impl PartialEq<SqlIdent> for Option<Rc<SqlIdent>> {
    fn eq(&self, other: &SqlIdent) -> bool {
        match self {
            Some(me) => me.deref().eq(other),
            None => false,
        }
    }
}

impl PartialEq<CanonicalIdent> for SqlIdent {
    fn eq(&self, canonical: &CanonicalIdent) -> bool {
        match self {
            SqlIdent::Unquoted(me) => me.inner == UniCase::<&str>::from(canonical.0.as_str()),
            SqlIdent::Quoted(me) => me.as_str() == canonical.0.as_str(),
            SqlIdent::Canonical(me) => me.0 == canonical.0,
        }
    }
}

impl PartialEq<SqlIdent> for CanonicalIdent {
    fn eq(&self, other: &SqlIdent) -> bool {
        match other {
            SqlIdent::Unquoted(unquoted) => {
                UniCase::<&str>::from(self.0.as_str()) == unquoted.inner
            }
            SqlIdent::Quoted(quoted) => self.0.as_str() == quoted.as_str(),
            SqlIdent::Canonical(canonical) => self == canonical,
        }
    }
}

/// Trait for database entities (schemas, tables, columns etc) that can be
/// uniquely identified via a `&str` with some parent context.
pub trait Named {
    type Name: PartialEq<SqlIdent>;

    fn name(&self) -> &Self::Name;
}

macro_rules! impl_named {
    ($ty:ty, $field:ident, $id_ty:ty) => {
        impl Named for $ty {
            type Name = $id_ty;

            fn name(&self) -> &Self::Name {
                &self.$field
            }
        }
    };
}

use crate::Column;
use crate::NamedRelation;
use crate::ColumnWithOptionalAlias;
use crate::Schema;
use crate::Table;

impl_named!(Schema, name, CanonicalIdent);
impl_named!(Table, name, CanonicalIdent);
impl_named!(Column, name, CanonicalIdent);
impl_named!(NamedRelation, name, SqlIdent);
impl_named!(ColumnWithOptionalAlias, alias, Option<Rc<SqlIdent>>);

impl<'a, Other: 'a + Named, T> Named for &'a T
where
    Self: Deref<Target = Other>, // where
                                 //     <T as Named>::Name: PartialEq<SqlIdent>,
{
    type Name = <Other as Named>::Name;

    fn name(&self) -> &Self::Name {
        self.deref().name()
    }
}

// impl<T: Named> Named for &Rc<T>
// where
//     <T as Named>::Name: PartialEq<SqlIdent>,
// {
//     type Name = <T as Named>::Name;

//     fn name(&self) -> &Self::Name {
//         (**self).name()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(SqlIdent::canonical("Foo"), CanonicalIdent::from("Foo"), true; "Canonical: both mixed case equal")]
    #[test_case(SqlIdent::canonical("foo"), CanonicalIdent::from("Foo"), false; "Canonical: different case not equal")]
    #[test_case(SqlIdent::canonical("Foo"), CanonicalIdent::from("foo"), false; "Canonical: different case not equal (swapped lhs & rhs)")]
    #[test_case(SqlIdent::canonical("Foo"), CanonicalIdent::from("Bar"), false; "Canonical: different identifiers are different")]
    #[test_case(SqlIdent::quoted("foo"), CanonicalIdent::from("foo"), true ; "Quoted: both lowercase equal")]
    #[test_case(SqlIdent::quoted("Foo"), CanonicalIdent::from("Foo"), true; "Quoted: both mixed case equal")]
    #[test_case(SqlIdent::quoted("foo"), CanonicalIdent::from("Foo"), false; "Quoted: mixed case different")]
    #[test_case(SqlIdent::quoted("Foo"), CanonicalIdent::from("foo"), false; "Quoted: different case not equal (swapped lhs & rhs)")]
    #[test_case(SqlIdent::quoted("Foo"), CanonicalIdent::from("Bar"), false; "Quoted: different identifiers are different")]
    #[test_case(SqlIdent::unquoted("foo"), CanonicalIdent::from("foo"), true; "Unquoted: both lowercase equal")]
    #[test_case(SqlIdent::unquoted("Foo"), CanonicalIdent::from("Foo"), true; "Unquoted: both mixed case equal")]
    #[test_case(SqlIdent::unquoted("foo"), CanonicalIdent::from("Foo"), true; "Unquoted: mixed case different")]
    #[test_case(SqlIdent::unquoted("Foo"), CanonicalIdent::from("foo"), true; "Unquoted: different case not equal (swapped lhs & rhs)")]
    #[test_case(SqlIdent::unquoted("Foo"), CanonicalIdent::from("Bar"), false; "Unquoted: different identifiers are different")]
    fn sql_ident_partial_eq_with_canonical_ident_tests_is_symmetric(
        sql_ident: SqlIdent,
        canonical_ident: CanonicalIdent,
        expected_equal: bool,
    ) {
        assert!(
            (sql_ident == canonical_ident) == expected_equal,
            "impl PartialEq<CanonicalIdent> for SqlIdent"
        );
        assert!(
            (canonical_ident == sql_ident) == expected_equal,
            "impl PartialEq<SqlIdent> for CanonicalIdent"
        );
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Item(CanonicalIdent);

    impl Named for Item {
        type Name = CanonicalIdent;

        fn name(&self) -> &Self::Name {
            &self.0
        }
    }

    impl From<&str> for Item {
        fn from(value: &str) -> Self {
            Self(CanonicalIdent::from(value))
        }
    }

    fn haystack(list: &[&str]) -> Vec<Rc<Item>> {
        list.iter()
            .map(|id| Item(CanonicalIdent::from(*id)))
            .map(Into::into)
            .collect()
    }

    #[test_case(SqlIdent::canonical("foo"), haystack(&[]), Err(FindUniqueMatchError::NotFound(SqlIdent::canonical("foo"))) ; "Canonical: nothing to find when haystack is empty")]
    #[test_case(SqlIdent::quoted("foo"), haystack(&[]), Err(FindUniqueMatchError::NotFound(SqlIdent::quoted("foo"))) ; "Quoted: nothing to find when haystack is empty")]
    #[test_case(SqlIdent::unquoted("foo"), haystack(&[]), Err(FindUniqueMatchError::NotFound(SqlIdent::unquoted("foo"))) ; "Unquoted: nothing to find when haystack is empty")]
    #[test_case(SqlIdent::canonical("foo"), haystack(&["foo"]), Ok(Rc::new(Item::from("foo"))) ; "Canonical: should find match")]
    #[test_case(SqlIdent::canonical("foo"), haystack(&["Foo"]), Err(FindUniqueMatchError::NotFound(SqlIdent::canonical("foo"))) ; "Canonical: should use case-sensitive match")]
    #[test_case(SqlIdent::canonical("foo"), haystack(&["Foo", "foo"]), Ok(Rc::new(Item::from("foo"))) ; "Canonical: should match when other item differing in case is present")]
    #[test_case(SqlIdent::quoted("foo"), haystack(&["foo"]), Ok(Rc::new(Item::from("foo"))) ; "Quoted: should find match")]
    #[test_case(SqlIdent::quoted("foo"), haystack(&["Foo"]), Err(FindUniqueMatchError::NotFound(SqlIdent::quoted("foo"))) ; "Quoted: should use case-sensitive match")]
    #[test_case(SqlIdent::quoted("foo"), haystack(&["Foo", "foo"]), Ok(Rc::new(Item::from("foo"))) ; "Quoted: should match when other item differing in case is present")]
    #[test_case(SqlIdent::unquoted("foo"), haystack(&["foo"]), Ok(Rc::new(Item::from("foo"))) ; "Unquoted: should find match")]
    #[test_case(SqlIdent::unquoted("foo"), haystack(&["Foo"]), Ok(Rc::new(Item::from("Foo"))) ; "Unquoted: should use sensitive match")]
    #[test_case(SqlIdent::unquoted("foo"), haystack(&["Foo", "foo"]), Err(FindUniqueMatchError::Ambiguous(SqlIdent::unquoted("foo"))) ; "Unquoted: should not match multiple items differing only in case are present")]
    fn find_unique_tests(
        needle: SqlIdent,
        haystack: Vec<Rc<Item>>,
        expected: Result<Rc<Item>, FindUniqueMatchError>,
    ) {
        let mut haystack = haystack.iter().cloned();
        assert_eq!(SqlIdent::find_unique(&needle, &mut haystack), expected);
    }

    #[test]
    fn unicase_sanity_check() {
        let a = UniCase::<String>::from(String::from("foo"));
        let b = UniCase::<&str>::from("Foo");

        assert_eq!(a, b);
        assert_eq!(&a, &b);
        assert_eq!(b, a);
        assert_eq!(&b, &a);
    }

    #[test]
    fn sanity_check() {
        let a = CanonicalIdent::from(String::from("Foo"));
        let b = SqlIdent::Unquoted(UnquotedIdent::from("foo"));

        assert_eq!(a, b);
        assert_eq!(&a, &b);
        assert_eq!(b, a);
        assert_eq!(&b, &a);

        let a = CanonicalIdent::from(String::from("foo"));
        let b = SqlIdent::Unquoted(UnquotedIdent::from("Foo"));

        assert_eq!(a, b);
        assert_eq!(&a, &b);
        assert_eq!(b, a);
        assert_eq!(&b, &a);
    }
}
