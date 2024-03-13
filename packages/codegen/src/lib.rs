use lazy_static_include::*;

lazy_static_include_str!(
    pub AST_NODE_IMPLS => "generated/visitable_impls.rs"
);

lazy_static_include_str!(
    pub SQL_NODE_ENUM => "generated/sql_node_enum.rs"
);

lazy_static_include_str!(
    pub NODE_LIST => "generated/node_list.json"
);
