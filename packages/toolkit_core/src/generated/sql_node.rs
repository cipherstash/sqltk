#![allow(clippy::redundant_closure_call)]

use crate::{Visitable, Node};

include!(concat!(env!("OUT_DIR"), "/generated_sql_node_enum.rs"));

include!(concat!(
    env!("OUT_DIR"),
    "/generated_sql_node_enum_match_macro.rs"
));
