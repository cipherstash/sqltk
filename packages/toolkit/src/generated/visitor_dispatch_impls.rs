use crate::{
    match_concrete_node, BoxOf, ConcreteNode, OptionOf, VecOf, VisitorControlFlow, VisitorDispatch,
    VisitorDispatchNode,
};

include!(concat!(env!("OUT_DIR"), "/generated_dispatch_impls.rs"));
