//! Defines the [`SchemaOps`] trait.

use crate::model::Schema;

/// Trait that provides a method for retrieving the database schema against
/// which tables/views/columns can be resolved.
pub trait SchemaOps {
    /// Get a reference to the database schema.
    fn get_schema(&self) -> &Schema;
}
