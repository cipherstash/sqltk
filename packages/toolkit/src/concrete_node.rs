#![allow(clippy::redundant_closure_call)]

use crate::{AstNode, Node};

include!(concat!(env!("OUT_DIR"), "/generated_concrete_node_enum.rs"));

include!(concat!(
    env!("OUT_DIR"),
    "/generated_concrete_node_enum_match_macro.rs"
));
