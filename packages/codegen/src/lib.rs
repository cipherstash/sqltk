use ast_node_trait_impls::AstNodeImpl;
use proc_macro2::TokenStream;
use reachability::Reachability;

use inflector::Inflector;

use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use std::{cmp::Ordering, collections::HashMap, fs::File, io::Write, path::PathBuf};
use syn::{Ident, TypePath};

mod ast_node_trait_impls;
mod generics;
mod meta;
mod parser;
mod reachability;
mod sqlparser_node_extractor;

use parser::*;

pub use meta::*;

pub struct Codegen {
    meta: SqlParserMetaQuery,
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            meta: SqlParserMetaQuery::from(sqlparser_node_extractor::extract(vec![])),
        }
    }

    pub fn generate_dispatch_impls(&self, dest_file: &PathBuf) {
        let mut generated_code = TokenStream::new();

        generated_code.append_all(self.define_visitor_dispatch_trait());
        generated_code.append_all(self.define_visitor_dispatch_impl());

        let mut file = File::create(dest_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

        let formatted = prettyplease::unparse(
            &syn::parse_file(&generated_code.to_string())
                .expect("BUG! Generated Rust code could not be parsed"),
        );

        file.write_all(formatted.as_bytes())
            .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display()));
    }

    fn define_visitor_dispatch_trait(&self) -> TokenStream {
        quote! {
            pub trait VisitorDispatch<'ast> {
                fn dispatch_enter(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow;

                fn dispatch_exit(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow;
            }
        }
    }

    fn define_visitor_dispatch_impl(&self) -> TokenStream {
        let bounds: proc_macro2::TokenStream = self.define_visitor_dispatch_trait_bounds();

        quote! {
            impl<'ast, V> VisitorDispatch<'ast> for V
            where
                Self: #bounds
            {
                fn dispatch_enter(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
                    match_concrete_node!(concrete_node, |node| { self.dispatch_node_enter(node) })
                }

                fn dispatch_exit(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
                    match_concrete_node!(concrete_node, |node| { self.dispatch_node_exit(node) })
                }
            }
        }
    }

    fn define_visitor_dispatch_trait_bounds(&self) -> TokenStream {
        let all_nodes = self.meta.all_nodes();
        let bounds = all_nodes.iter().map(|node| {
            quote! {
                VisitorDispatchNode<'ast, #node>
            }
        });

        quote! { #(#bounds +)* }
    }

    pub fn generate_ast_node_impls(&self, dest_file: &PathBuf) {
        eprintln!("cargo:message=REACH 1");
        let reachability = Reachability::derive(&self.meta);

        let reachability_file = PathBuf::from("/tmp/reachable.txt");

        let mut file = File::create(&reachability_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &reachability_file.display()));

        for (ty, distance) in &reachability {
            let _ = file.write(format!("{}: {}\n", ty.to_token_stream(), distance.0).as_bytes());
        }

        eprintln!("cargo:message=REACH 2");

        let mut generated_code = TokenStream::new();

        let main_nodes = self.meta.main_nodes();
        let ast_node_impls_for_main_nodes = main_nodes.iter().map(|(type_path, type_def)| {
            AstNodeImpl::new(type_path, type_def, &reachability, &self.meta)
        });

        let ast_node_impls_for_primitive_nodes =
            self.impl_ast_node_for_primitive_nodes(self.meta.primitive_nodes());

        generated_code.append_all(quote! {
            #(#ast_node_impls_for_main_nodes)*
            #ast_node_impls_for_primitive_nodes
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

    pub fn generate_node_list_file(&self, dest_file: &PathBuf) {
        // FIXME: ensure sqlparser is built with correct set of features
        let meta = sqlparser_node_extractor::extract(vec![]);

        let file = File::create(dest_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

        serde_json::to_writer_pretty(&file, &meta)
            .expect("Failed to serialise sqlparser metadata to file")
    }

    pub fn generated_concrete_node_enum(&self, dest_file: &PathBuf) {
        // FIXME: ensure sqlparser is built with correct set of features

        let mut output = proc_macro2::TokenStream::new();

        let main_nodes = self.meta.main_nodes();
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

        let primitive_nodes = self.meta.primitive_nodes();
        let primitive_nodes_variants = primitive_nodes.iter().map(|primitive_node| {
            let ident = primitive_node.variant_ident();
            let type_path = primitive_node.type_path();
            quote! {
                #ident(Node<'ast, #type_path>),
            }
        });
        let mut primitive_node_from_impls = proc_macro2::TokenStream::new();
        for pn in primitive_nodes.iter() {
            let ident = pn.variant_ident();
            let type_path = pn.type_path();
            primitive_node_from_impls.append_all(quote! {
                impl<'ast> From<Node<'ast, #type_path>> for ConcreteNode<'ast> {
                    fn from(value: Node<'ast, #type_path>) -> Self {
                        Self::#ident(value)
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

        let container_nodes = self.meta.container_nodes();

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

            let variant = quote! {
                #variant_ident(Node<'ast, #type_path>),
            };
            let from_impl = quote! {
                impl<'ast> From<Node<'ast, #type_path>> for #container_enum_ident<'ast> {
                    fn from(value: Node<'ast, #type_path>) -> Self {
                        Self::#variant_ident(value)
                    }
                }
            };

            match node {
                ContainerNode::Box(_) => {
                    box_of_variants.append_all(quote! {
                        #variant
                    });
                    box_of_from_impls.append_all(from_impl);
                }
                ContainerNode::Vec(_) => {
                    vec_of_variants.append_all(quote! {
                        #variant
                    });
                    vec_of_from_impls.append_all(from_impl);
                }
                ContainerNode::Option(_) => {
                    option_of_variants.append_all(quote! {
                        #variant
                    });
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

            #[automatically_derived]
            impl<'ast, T: AstNode<'ast>> From<Node<'ast, Box<T>>> for ConcreteNode<'ast>
            where
                BoxOf<'ast>: From<Node<'ast, Box<T>>>,
            {
                fn from(value: Node<'ast, Box<T>>) -> Self {
                    Self::Box(BoxOf::from(value))
                }
            }

            #[automatically_derived]
            impl<'ast, T: AstNode<'ast>> From<Node<'ast, Vec<T>>> for ConcreteNode<'ast>
            where
                VecOf<'ast>: From<Node<'ast, Vec<T>>>,
                ConcreteNode<'ast>: From<Node<'ast, T>>
            {
                fn from(value: Node<'ast, Vec<T>>) -> Self {
                    Self::Vec(VecOf::from(value))
                }
            }

            #[automatically_derived]
            impl<'ast, T: AstNode<'ast>> From<Node<'ast, Option<T>>> for ConcreteNode<'ast>
            where
                OptionOf<'ast>: From<Node<'ast, Option<T>>>,
            {
                fn from(value: Node<'ast, Option<T>>) -> Self {
                    Self::Option(OptionOf::from(value))
                }
            }

            #[automatically_derived]
            impl<'ast> std::fmt::Display for ConcreteNode<'ast> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match_concrete_node!(self, |node| { write!(f, "{}", node) })
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

    pub fn generate_concrete_node_enum_match_macro(&self, dest_file: &PathBuf) {
        let mut output = proc_macro2::TokenStream::new();

        let main_nodes = self.meta.main_nodes();
        let main_node_variants = main_nodes.iter().map(|(type_path, _)| {
            let ident = &type_path.path.segments.last().unwrap().ident;
            quote! {
                #ident(node)
            }
        });

        let primitive_nodes = self.meta.primitive_nodes();
        let primitive_node_variants = primitive_nodes.iter().map(|primitive_node| {
            let ident = primitive_node.variant_ident();
            quote! {
                #ident(node)
            }
        });

        let mut vec_of_variants = Vec::<proc_macro2::TokenStream>::new();
        let mut box_of_variants = Vec::<proc_macro2::TokenStream>::new();
        let mut option_of_variants = Vec::<proc_macro2::TokenStream>::new();

        let container_nodes = self.meta.container_nodes();

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
            #[macro_export]
            macro_rules! _match_concrete_node {
                ($node:ident, $handler:expr) => {
                    // Due to macro hygeine reasons code is generated using a
                    // closure.  Changing this macro to a proc macro would allow
                    // control over hygeine and we can implement the macro correctly
                    // and avoid clippy complaining.
                    // TODO: refactor into proc macro to satisfy clippy

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

            #[doc = "Fixes macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths"]
            #[doc(hidden)]
            pub use _match_concrete_node as match_concrete_node;
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
        &self,
        primitive_nodes: impl IntoIterator<Item = PrimitiveNode>,
    ) -> TokenStream {
        let mut output = proc_macro2::TokenStream::new();

        for node in primitive_nodes {
            let type_path = node.type_path();
            let ident_str = &type_path.0.path.segments.last().unwrap().ident.to_string();

            output.append_all(quote! {
                #[automatically_derived]
                impl<'ast> crate::AstNode<'ast> for #type_path {
                    fn accept_with_node_builder<V: crate::VisitorDispatch<'ast>>(
                        &'ast self,
                        visitor: &mut V,
                        node_builder: &mut crate::NodeBuilder,
                    ) -> crate::VisitorControlFlow {
                        crate::visit(
                            node_builder.new_node(self).into(),
                            visitor,
                            #[allow(unused_variables)]
                            |visitor| {
                                crate::nav_skip()
                            }
                        )
                    }
                }

                #[automatically_derived]
                impl<'ast> std::fmt::Display for crate::DisplayType<#type_path> {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", #ident_str)
                    }
                }
            });

        }

        output
    }
}
