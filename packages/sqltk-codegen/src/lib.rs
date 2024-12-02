use lazy_static_include::*;

lazy_static_include_str!(
    pub VISITABLE_IMPLS => "generated/visitable_impls.rs"
);

lazy_static_include_str!(
    pub APPLY_TRANSFORM_IMPLS => "generated/apply_transform_impls.rs"
);
