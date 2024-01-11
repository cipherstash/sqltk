use proc_macro2::{TokenStream};
use visitable_trait_impls::AstNodeImpl;

use inflector::Inflector;

use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use std::{cmp::Ordering, collections::HashMap, fs::File, io::Write, path::PathBuf};
use syn::{Ident, TypePath};

mod generics;
mod meta;
mod parser;
mod sqlparser_node_extractor;
mod visitable_trait_impls;

use parser::*;

pub use meta::*;

pub fn generate_dispatch_impls(dest_file: &PathBuf) {
    let mut generated_code = TokenStream::new();

    generated_code.append_all(define_visitor_dispatch_trait());
    generated_code.append_all(define_visitor_dispatch_impl());

    let mut file = File::create(dest_file)
        .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

    let formatted = prettyplease::unparse(
        &syn::parse_file(&generated_code.to_string())
            .expect("BUG! Generated Rust code could not be parsed"),
    );

    file.write_all(formatted.as_bytes())
        .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display()));
}

fn define_visitor_dispatch_trait() -> TokenStream {
    quote! {
        pub trait VisitorDispatch<'ast> {
            fn dispatch_enter(&self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow;

            fn dispatch_exit(&self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow;
        }
    }
}

fn define_visitor_dispatch_impl() -> TokenStream {
    let bounds: proc_macro2::TokenStream = define_visitor_dispatch_trait_bounds();

    quote! {
        impl<'ast, V> VisitorDispatch<'ast> for V
        where
            Self: #bounds
        {
            fn dispatch_enter(&self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
                match_concrete_node!(concrete_node, |node| { self.dispatch_node_enter(node) })
            }

            fn dispatch_exit(&self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
                match_concrete_node!(concrete_node, |node| { self.dispatch_node_exit(node) })
            }
        }
    }
}

fn define_visitor_dispatch_trait_bounds() -> TokenStream {
    let meta = SqlParserMetaQuery::from(sqlparser_node_extractor::extract(vec![]));
    let all_nodes = meta.all_nodes();
    let bounds = all_nodes.iter().map(|node| quote! {
        VisitorDispatchNode<'ast, #node>
    });

    quote! { #(#bounds +)* }
}

pub fn generate_visitable_impls(dest_file: &PathBuf) {
    let mut generated_code = TokenStream::new();

    // FIXME: ensure sqlparser is built with correct set of features
    let query = SqlParserMetaQuery::from(sqlparser_node_extractor::extract(vec![]));

    let main_nodes = query.main_nodes();
    let visitable_impls_for_main_nodes = main_nodes
        .iter()
        .map(|(type_path, type_def)| AstNodeImpl(type_path, type_def));

    let visitable_impls_for_primitive_nodes =
        impl_ast_node_for_primitive_nodes(query.primitive_nodes());

    generated_code.append_all(quote! {
        #(#visitable_impls_for_main_nodes)*
        #visitable_impls_for_primitive_nodes
    });

    let mut file = File::create(dest_file)
        .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

    let formatted = prettyplease::unparse(
        &syn::parse_file(&generated_code.to_string())
            .expect("BUG! Generated Rust code could not be parsed"),
    );

    file.write_all(formatted.as_bytes())
        .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display()));
}

pub fn generate_node_list_file(dest_file: &PathBuf) {
    // FIXME: ensure sqlparser is built with correct set of features
    let meta = sqlparser_node_extractor::extract(vec![]);

    let file = File::create(dest_file)
        .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

    serde_json::to_writer_pretty(&file, &meta)
        .expect("Failed to serialise sqlparser metadata to file")
}

pub fn generated_concrete_node_enum(dest_file: &PathBuf) {
    // FIXME: ensure sqlparser is built with correct set of features
    let query = SqlParserMetaQuery::from(sqlparser_node_extractor::extract(vec![]));

    let mut output = proc_macro2::TokenStream::new();

    let main_nodes = query.main_nodes();
    let main_nodes_variants = main_nodes.iter().map(|(type_path, _)| {
        let ident = &type_path.path.segments.last().unwrap().ident;
        quote! {
            #ident(Node<'ast, #type_path>),
        }
    });
    let mut main_node_from_impls = proc_macro2::TokenStream::new();
    for type_path in main_nodes.iter().map(|(type_path, _)| type_path) {
        let ident = &type_path.path.segments.last().unwrap().ident;
        main_node_from_impls.append_all(quote! {
            impl<'ast> From<Node<'ast, #type_path>> for ConcreteNode<'ast> {
                fn from(value: Node<'ast, #type_path>) -> Self {
                    Self::#ident(value)
                }
            }
        });
    }

    let primitive_nodes = query.primitive_nodes();
    let primitive_nodes_variants = primitive_nodes.iter().map(|primitive_node| {
        let ident = primitive_node.variant_ident();
        let type_path = primitive_node.type_path();
        quote! {
            #ident(Node<'ast, #type_path>),
        }
    });
    let mut primitive_node_from_impls = proc_macro2::TokenStream::new();
    for pn in primitive_nodes.iter() {
        let variant_ident = pn.variant_ident();
        let type_path = pn.type_path();
        primitive_node_from_impls.append_all(quote! {
            impl<'ast> From<Node<'ast, #type_path>> for ConcreteNode<'ast> {
                fn from(value: Node<'ast, #type_path>) -> Self {
                    Self::#variant_ident(value)
                }
            }
        });
    }

    let mut vec_of_variants = proc_macro2::TokenStream::new();
    let mut box_of_variants = proc_macro2::TokenStream::new();
    let mut option_of_variants = proc_macro2::TokenStream::new();
    let mut vec_of_from_impls = proc_macro2::TokenStream::new();
    let mut box_of_from_impls = proc_macro2::TokenStream::new();
    let mut option_of_from_impls = proc_macro2::TokenStream::new();

    let container_nodes = query.container_nodes();

    for node in container_nodes.iter() {
        let type_path = &node.type_path();
        let type_path_of_generic = &generics::extract_generic_argument(type_path);
        let container_enum_ident = format_ident!(
            "{}Of",
            &type_path
                .path
                .segments
                .last()
                .unwrap()
                .ident
                .to_string()
                .to_pascal_case()
        );

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

        let variant = quote!(#variant_ident(Node<'ast, #type_path>),);
        let from_impl = quote! {
            impl<'ast> From<Node<'ast, #type_path>> for #container_enum_ident<'ast> {
                fn from(value: Node<'ast, #type_path>) -> Self {
                    Self::#variant_ident(value)
                }
            }
        };

        match node {
            ContainerNode::Box(_) => {
                box_of_variants.append_all(variant);
                box_of_from_impls.append_all(from_impl);
            }
            ContainerNode::Vec(_) => {
                vec_of_variants.append_all(variant);
                vec_of_from_impls.append_all(from_impl);
            }
            ContainerNode::Option(_) => {
                option_of_variants.append_all(variant);
                option_of_from_impls.append_all(from_impl);
            }
        }
    }

    output.append_all(quote! {
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum BoxOf<'ast> {
            #box_of_variants
        }

        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum OptionOf<'ast> {
            #option_of_variants
        }

        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum VecOf<'ast> {
            #vec_of_variants
        }

        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum ConcreteNode<'ast> {
            #(#main_nodes_variants)*
            #(#primitive_nodes_variants)*
            Box(BoxOf<'ast>),
            Option(OptionOf<'ast>),
            Vec(VecOf<'ast>),
        }

        #box_of_from_impls
        #vec_of_from_impls
        #option_of_from_impls

        #primitive_node_from_impls
        #main_node_from_impls

        impl<'ast, T: AstNode<'ast>> From<Node<'ast, Box<T>>> for ConcreteNode<'ast>
        where
            BoxOf<'ast>: From<Node<'ast, Box<T>>>,
        {
            fn from(value: Node<'ast, Box<T>>) -> Self {
                Self::Box(BoxOf::from(value))
            }
        }

        impl<'ast, T: AstNode<'ast>> From<Node<'ast, Vec<T>>> for ConcreteNode<'ast>
        where
            VecOf<'ast>: From<Node<'ast, Vec<T>>>,
        {
            fn from(value: Node<'ast, Vec<T>>) -> Self {
                Self::Vec(VecOf::from(value))
            }
        }

        impl<'ast, T: AstNode<'ast>> From<Node<'ast, Option<T>>> for ConcreteNode<'ast>
        where
            OptionOf<'ast>: From<Node<'ast, Option<T>>>,
        {
            fn from(value: Node<'ast, Option<T>>) -> Self {
                Self::Option(OptionOf::from(value))
            }
        }
    });

    let mut file = File::create(dest_file)
        .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

    eprintln!("cargo:message=ENUM {}", &output.to_string());

    let formatted = prettyplease::unparse(
        &syn::parse_file(&output.to_string())
            .expect("BUG! Generated Rust code could not be parsed"),
    );

    file.write_all(formatted.as_bytes())
        .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display()));
}

pub fn generate_concrete_node_enum_match_macro(dest_file: &PathBuf) {
    // FIXME: ensure sqlparser is built with correct set of features
    let query = SqlParserMetaQuery::from(sqlparser_node_extractor::extract(vec![]));

    let mut output = proc_macro2::TokenStream::new();

    let main_nodes = query.main_nodes();
    let main_node_variants = main_nodes.iter().map(|(type_path, _)| {
        let ident = &type_path.path.segments.last().unwrap().ident;
        quote! {
            #ident(node)
        }
    });

    let primitive_nodes = query.primitive_nodes();
    let primitive_node_variants = primitive_nodes.iter().map(|primitive_node| {
        let ident = primitive_node.variant_ident();
        quote! {
            #ident(node)
        }
    });

    let mut vec_of_variants = Vec::<proc_macro2::TokenStream>::new();
    let mut box_of_variants = Vec::<proc_macro2::TokenStream>::new();
    let mut option_of_variants = Vec::<proc_macro2::TokenStream>::new();

    let container_nodes = query.container_nodes();

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
        macro_rules! match_concrete_node {
            ($node:ident, $handler:expr) => {
                match $node {
                    #(
                        ConcreteNode::#main_node_variants => { $handler(node) }
                    )*

                    #(
                        ConcreteNode::#primitive_node_variants => { $handler(node) }
                    )*

                    #(
                        ConcreteNode::Box(BoxOf::#box_of_variants) => { $handler(node) }
                    )*

                    #(
                        ConcreteNode::Option(OptionOf::#option_of_variants) => { $handler(node) }
                    )*

                    #(
                        ConcreteNode::Vec(VecOf::#vec_of_variants) => { $handler(node) }
                    )*
                }
            }
        }
    });

    let mut file = File::create(dest_file)
        .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

    eprintln!("cargo:message=ENUM {}", &output.to_string());

    let formatted = prettyplease::unparse(
        &syn::parse_file(&output.to_string())
            .expect("BUG! Generated Rust code could not be parsed"),
    );

    file.write_all(formatted.as_bytes())
        .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display()));
}

pub(crate) fn impl_ast_node_for_primitive_nodes(
    primitive_nodes: impl IntoIterator<Item = PrimitiveNode>,
) -> TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    for node in primitive_nodes {
        let type_path = node.type_path();
        output.append_all(quote! {
            #[automatically_derived]
            impl<'ast> crate::AstNode<'ast> for #type_path {
                fn accept_with_id_iter<V: crate::VisitorDispatch<'ast>>(
                    &'ast self,
                    visitor: &V,
                    node_builder: &mut crate::NodeBuilder,
                ) -> crate::VisitorControlFlow {
                    crate::visit(
                        node_builder.new_node(self).into(),
                        visitor,
                        crate::nav_skip
                    )
                }
            }
        })
    }

    output
}

pub fn generate_identifiable_impls(dest_file: &PathBuf) {
    // FIXME: ensure sqlparser is built with correct set of features
    let query = SqlParserMetaQuery::from(sqlparser_node_extractor::extract(vec![]));

    let mut output = proc_macro2::TokenStream::new();

    for (type_path, id) in query.all_nodes_with_node_id() {
        output.append_all(quote! {
            impl crate::Identifiable for #type_path {
                const ID: usize = #id;
            }
        })
    }

    let mut file = File::create(dest_file)
        .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

    let formatted = prettyplease::unparse(
        &syn::parse_file(&output.to_string())
            .expect("BUG! Generated Rust code could not be parsed"),
    );

    file.write_all(formatted.as_bytes())
        .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display()));
}
