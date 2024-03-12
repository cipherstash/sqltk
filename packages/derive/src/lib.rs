use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use sqltk_codegen::NODE_LIST;
use sqltk_meta::{ContainerNode, SqlParserMeta, SqlParserMetaQuery};
use sqltk_syn_helpers::generics::{self};
use syn::{parse_macro_input, parse_quote, DeriveInput};

/// Derives [`VisitorDispatch`].
///
/// ```
/// # extern crate sqltk_core as sqltk;
/// # use sqltk_derive::*;
/// use sqltk::*;
/// use sqltk::sqlparser::*;
/// use std::ops::ControlFlow;
/// use std::marker::PhantomData;
/// use derive_more::AsMut;
///
/// #[derive(VisitorDispatch)]
/// struct ExprCounter  {
///     counter: usize
/// }
///
/// impl<'ast> Visitor<'ast, ast::Expr> for ExprCounter {
///     fn enter(&mut self, node: &'ast ast::Expr) -> EnterControlFlow {
///         self.counter += 1usize;
///         ControlFlow::Continue(Nav::Visit)
///     }
/// }
/// ```
#[proc_macro_derive(VisitorDispatch)]
pub fn derive_visitor_dispatch(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let visitor = &input.ident;
    let generics = input.generics;
    let krate = resolve_crate();

    let mut modified_generics = generics.clone();

    modified_generics.params.push(parse_quote!('ast__));

    let (impl_generics, _, _) = modified_generics.split_for_impl();
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let mut output = proc_macro2::TokenStream::new();

    let dispatch_enter = dispatch_sql_node(&parse_quote!(node), DispatchFn::Enter);
    let dispatch_exit = dispatch_sql_node(&parse_quote!(node), DispatchFn::Exit);

    output.append_all(quote! {
        impl #impl_generics #krate::VisitorDispatch<'ast__> for #visitor #ty_generics #where_clause {
            fn enter(&mut self, node: #krate::SqlNode<'ast__>) -> #krate::EnterControlFlow {
                #dispatch_enter
            }

            fn exit(&mut self, node: #krate::SqlNode<'ast__>) -> #krate::ExitControlFlow {
                #dispatch_exit
            }
        }
    });

    output.into()
}

fn node_meta() -> SqlParserMetaQuery {
    let meta: SqlParserMeta = serde_json::from_str(
        String::from_utf8(Vec::from(&NODE_LIST[..]))
            .unwrap()
            .as_str(),
    )
    .unwrap();
    meta.into()
}

fn resolve_crate() -> proc_macro2::TokenStream {
    let found_crate = crate_name("sqltk_core").expect("sqltk_core not found in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident )
        }
    }
}

enum DispatchFn {
    Enter,
    Exit,
}

impl ToTokens for DispatchFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Enter => tokens.append_all(quote!(enter)),
            Self::Exit => tokens.append_all(quote!(exit)),
        }
    }
}

fn dispatch_sql_node(node: &Ident, dispatch_fn: DispatchFn) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();
    let krate = resolve_crate();

    let meta = node_meta();

    let main_nodes = meta.main_nodes();
    let main_node_variants = main_nodes.iter().map(|(type_path, _)| {
        let ident = &type_path.path.segments.last().unwrap().ident;
        quote! {
            #ident(node)
        }
    });

    let primitive_nodes = meta.primitive_nodes();
    let primitive_node_variants = primitive_nodes.iter().map(|primitive_node| {
        let ident = primitive_node.variant_ident();
        quote! {
            #ident(node)
        }
    });

    let mut vec_of_variants = Vec::<proc_macro2::TokenStream>::new();
    let mut box_of_variants = Vec::<proc_macro2::TokenStream>::new();
    let mut option_of_variants = Vec::<proc_macro2::TokenStream>::new();

    let container_nodes = meta.container_nodes();

    for node in container_nodes.iter() {
        let type_path = &node.type_path();
        let type_path_of_generic = &generics::extract_generic_argument(type_path);

        let decomposed_generics = generics::decompose_generic_type(type_path_of_generic);

        let variant_ident = format_ident!(
            "{}",
            decomposed_generics
                .iter()
                .map(|tp| tp
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_string()
                    .to_pascal_case())
                .collect::<Vec<String>>()
                .join("Of")
        );

        let variant = quote!(#variant_ident(node));

        match node {
            ContainerNode::Box(_) => {
                box_of_variants.push(variant);
            }
            ContainerNode::Vec(_) => {
                vec_of_variants.push(variant);
            }
            ContainerNode::Option(_) => {
                option_of_variants.push(variant);
            }
        }
    }

    output.append_all(quote! {
        use #krate::dispatch::specialization::{Dispatch, ViaVisitor, ViaFallback, ViaIgnored};
        use #krate::{BoxOf, OptionOf, VecOf};

        match #node {
            #( SqlNode::#main_node_variants => (&mut &mut &mut Dispatch(self, node)).#dispatch_fn(node),)*
            #( SqlNode::#primitive_node_variants => (&mut &mut &mut Dispatch(self, node)).#dispatch_fn(node),)*
            #( SqlNode::Box(BoxOf::#box_of_variants) => (&mut &mut &mut Dispatch(self, node)).#dispatch_fn(node),)*
            #( SqlNode::Option(OptionOf::#option_of_variants) => (&mut &mut &mut Dispatch(self, node)).#dispatch_fn(node),)*
            #( SqlNode::Vec(VecOf::#vec_of_variants) => (&mut &mut &mut Dispatch(self, node)).#dispatch_fn(node),)*
        }
    });

    output
}
