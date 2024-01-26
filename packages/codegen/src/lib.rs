use lazy_static_include::*;

lazy_static_include_str!(
    pub VISITOR_DISPATCH_IMPLS => "generated_dispatch_impls.rs"
);

lazy_static_include_str!(
    pub AST_NODE_IMPLS => "generated_ast_node_impls.rs"
);

lazy_static_include_str!(
    pub CONCRETE_NODE_ENUM => "generated_concrete_node_enum.rs"
);

lazy_static_include_str!(
    pub CONCRETE_NODE_ENUM_MATCH_MACRO => "generated_concrete_node_enum_match_macro.rs"
);

lazy_static_include_str!(
    pub NODE_LIST => "generated_node_list.json"
);
