use lazy_static_include::*;

lazy_static_include_str!(
    pub AST_NODE_IMPLS => "generated/visitable_impls.rs"
);

lazy_static_include_str!(
    pub NODE_ENUM => "generated/node_enum.rs"
);

lazy_static_include_str!(
    pub NODE_ENUM_MATCH_MACRO => "generated/node_enum_match_macro.rs"
);