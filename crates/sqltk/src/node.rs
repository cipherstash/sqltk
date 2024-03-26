use indoc::indoc;

include!(concat!(env!("OUT_DIR"), "/generated_node_enum.rs"));

// Newtype wrapper so that From<sqlparser::ast::*> implementations
// can be written without causing orphan rule violations.
struct FromWrapper<T>(T);