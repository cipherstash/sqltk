//! Types that model a database schema.
//!
//! Column type information is unused currently.

use core::fmt::Debug;
use derive_more::Display;
use std::rc::Rc;

mod schema_api;
mod sql_ident;

pub use schema_api::*;
pub use sql_ident::*;

use super::expr_source::TableColumn;

/// A database schema.
///
/// It has a name and some tables. Tables and views are not distinguishable as
/// it makes no difference to the analysis functionality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub name: CanonicalIdent,
    // TODO: consider changing this type to something that behaves like a
    // HashMap, but is fast for a low number of entries.
    pub tables: Vec<Rc<Table>>,
    pub aggregates: Vec<Rc<String>>,
}

/// A table (or view).
///
/// It has a name and some columns
#[derive(Debug, Clone, PartialEq, Eq, Display, PartialOrd, Ord, Hash)]
#[display(fmt = "{}", name)]
pub struct Table {
    pub name: Rc<CanonicalIdent>,
    pub columns: Vec<Rc<Column>>,
    // Stores indices into the columns Vec.
    pub primary_key: Vec<usize>,
}

/// A column.
#[derive(Debug, Clone, PartialEq, Eq, Display, PartialOrd, Ord, Hash)]
#[display(fmt = "{}", name)]
pub struct Column {
    pub name: Rc<CanonicalIdent>,
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
        self.tables.push(table.into());
        self
    }

    /// Resolves a table by `SqlIdent`, which takes into account the SQL rules
    /// of quoted and unquoted identifier matching.
    pub fn resolve_table(&self, name: &SqlIdent) -> Result<Rc<Table>, FindUniqueMatchError> {
        SqlIdent::find_unique(name, &mut self.tables.iter().cloned())
    }

    pub fn resolve_table_column(
        &self,
        table: &SqlIdent,
        column: &SqlIdent,
    ) -> Result<TableColumn, FindUniqueMatchError> {
        match SqlIdent::find_unique(table, &mut self.tables.iter().cloned()) {
            Ok(table) => match table.get_column(column) {
                Ok(column) => Ok(TableColumn::new(Rc::clone(&table), Rc::clone(&column))),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}

impl Table {
    /// Create a new named table with no columns.
    pub fn new(name: &str) -> Self {
        Self {
            name: Rc::new(name.into()),
            primary_key: Vec::with_capacity(1),
            columns: Vec::with_capacity(16),
        }
    }

    /// Adds a column to the table.
    pub fn add_column(mut self, column: Column, part_of_primary_key: bool) -> Self {
        self.columns.push(column.into());
        if part_of_primary_key {
            self.primary_key.push(self.columns.len());
        }
        self
    }

    /// Checks if a column named `name` exists in the table.
    pub fn contains_column(&self, name: &SqlIdent) -> bool {
        self.get_column(name).is_ok()
    }

    /// Gets a column from a table by name.
    pub fn get_column(&self, name: &SqlIdent) -> Result<Rc<Column>, FindUniqueMatchError> {
        let mut haystack = self.columns.iter().cloned();
        SqlIdent::find_unique(name, &mut haystack)
    }
}

/// A DSL to create a [`Schema`] for testing purposes.
#[cfg(test)]
#[macro_export]
macro_rules! make_schema {
    (@name $schema_name:literal) => {
        stringify!($schema_name)
    };
    (@schema $schema:ident $schema_name:ident) => {
        $crate::model::Schema::new($schema_name)
    };
    (@schema $schema:ident) => {
        $crate::schema::Schema::new("public")
    };
    (
        @match_tables $schema:ident
        tables: {
            $($table_name:ident : $column_defs:tt)*
        }
        $(,$($rest:tt)*)?
    ) => {
        {
            $( make_schema!(@add_table $schema $table_name table $column_defs); )*
            $( make_schema!(@add_aggregates $schema $($rest)*); )?
            $schema
        }
    };
    (@add_aggregates $schema:ident [ $($aggregate_name:ident),* ]) => {
        {
            $schema.aggregates = vec![$($aggregate_name,)* ];
        }
    };
    (@add_table $schema:ident $table_name:ident $table:ident { $($columns:tt)* }) => {
        let $schema = $schema.add_table(
            {
                let $table = Table::new(stringify!($table_name));
                make_schema!(@add_columns $table $($columns)*);
                $table
            }
        );
    };
    (@add_columns $table:ident $( $column_name:ident $(($($options:tt)+))? , )* ) => {
        $( make_schema!(@add_column $table $column_name $(($($options)*))? ); )*
    };
    (@add_column $table:ident $column_name:ident (PK) ) => {
        let $table = $table.add_column($crate::model::Column{
            name: std::rc::Rc::new($crate::model::CanonicalIdent::from(stringify!($column_name)))
        }, true);
    };
    (@add_column $table:ident $column_name:ident () ) => {
        let $table = $table.add_column($crate::model::schema::Column{
            name: $crate::model::schema::CanonicalIdent::from(stringify!($column_name))
        }, false);
    };
    (@add_column $table:ident $column_name:ident ) => {
        let $table = $table.add_column($crate::model::Column{
            name: std::rc::Rc::new($crate::model::CanonicalIdent::from(stringify!($column_name)))
        }, false);
    };
    // Main entry points
    {
        name: $schema_name:ident
        $(,$($rest:tt)*)?
    } => {
        {
            let schema_name = stringify!($schema_name);
            let schema = make_schema!(@schema schema schema_name);
            $( let schema = make_schema!(@match_tables schema $($rest)* ); )?
            schema
        }
    };
    // Main entry points
    {
        name: $schema_name:literal
        $(,$($rest:tt)*)?
    } => {
        {
            let schema_name = $schema_name;
            let schema = make_schema!(@schema schema schema_name);
            $( let schema = make_schema!(@match_tables schema $($rest)* ); )?
            schema
        }
    };
    // Main entry points
    { $($rest:tt)+ } => {
        {
            let schema_name = "public";
            let schema = make_schema!(@schema schema schema_name);
            let schema = make_schema!(@match_tables schema $($rest)* );
            schema
        }
    };
}
