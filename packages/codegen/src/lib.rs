use lazy_static_include::*;

lazy_static_include_str!(
    pub VISITOR_DISPATCH_TABLE => "generated/dispatch_table_trait.rs"
);

lazy_static_include_str!(
    pub VISITOR_DISPATCH_TABLE_LOOKUP_IMPLS => "generated/dispatch_table_lookup_impls.rs"
);

lazy_static_include_str!(
    pub VISITOR_DISPATCH_IMPLS => "generated/dispatch_impls.rs"
);

lazy_static_include_str!(
    pub AST_NODE_IMPLS => "generated/visitable_impls.rs"
);

lazy_static_include_str!(
    pub SQL_NODE_ENUM => "generated/sql_node_enum.rs"
);

lazy_static_include_str!(
    pub SQL_NODE_ENUM_MATCH_MACRO => "generated/sql_node_enum_match_macro.rs"
);

lazy_static_include_str!(
    pub NODE_LIST => "generated/node_list.json"
);
