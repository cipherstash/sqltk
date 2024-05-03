//! Types that model a database schema.
//!
//! Column type information is unused currently.

use core::fmt::Debug;
use derive_more::Display;
use std::collections::HashMap;
use unicase::UniCase;

/// A database schema.
///
/// It has a name and some tables. Tables and views are not distinguishable as
/// it makes no difference to the analysis functionality.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Schema {
    pub name: UniCase<String>,
    pub tables: HashMap<UniCase<String>, Table>,
    pub aggregates: Vec<String>,
}

/// A table (or view).
///
/// It has a name and some columns
#[derive(Debug, Clone, PartialEq, Eq, Display, PartialOrd, Ord, Hash)]
#[display(fmt = "{}", name)]
pub struct Table {
    pub name: UniCase<String>,
    pub primary_key_columns: Vec<UniCase<String>>,
    pub columns: Vec<UniCase<String>>,
}

impl Schema {
    /// Creates a new named empty schema.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            tables: Default::default(),
            aggregates: Default::default(),
        }
    }

    /// Adds a table (or view) to the schema.
    pub fn add_table(mut self, table: Table) -> Self {
        self.tables.insert(table.name.clone(), table);
        self
    }

    /// Resolves a table by name in the schema.
    pub fn resolve_table(&self, name: &UniCase<String>) -> Option<Table> {
        self.tables.get(name).cloned()
    }
}

impl Table {
    /// Create a new named table with no columns.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            primary_key_columns: Vec::with_capacity(1),
            columns: Vec::with_capacity(16),
        }
    }

    /// Adds a column to the table.
    pub fn add_column(mut self, column: UniCase<String>) -> Self {
        self.columns.push(column);
        self
    }

    /// Checks if a column named `name` exists in the table.
    pub fn contains_column(&self, name: &UniCase<String>) -> bool {
        self.get_column(name).map(|_| true).unwrap_or(false)
    }

    /// Gets a column from a table by name.
    pub fn get_column(&self, name: &UniCase<String>) -> Option<&UniCase<String>> {
        self.columns.iter().find(|c| *c == name)
    }
}

/// A DSL to create a [`Schema`] for testing against
/// TODO: add support for marking columns as participting in a primary key
#[cfg(test)]
#[macro_export]
macro_rules! make_schema {
    (@add_table $schema:ident $table:ident ( $($column:ident )+ )) => {
        let $schema = $schema.add_table(
            Table::new(stringify!($table))
                $( .add_column(unicase::UniCase::new(stringify!($column).into()))  )+
        );
    };
    // Main entry point
    ($($table:ident $tokens:tt)*) => {
        {
            let schema = $crate::schema::Schema::new("Test schema");
            $( make_schema!(@add_table schema $table $tokens); )*
            schema
        }
    }
}
