use crate::{
    match_concrete_node, BoxOf, ConcreteNode, EnterControlFlow, ExitControlFlow, OptionOf, VecOf,
    VisitorDispatch, VisitorDispatchNode,
};

include!(concat!(env!("OUT_DIR"), "/generated_dispatch_impls.rs"));
