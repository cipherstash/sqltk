use super::generics;
use super::meta::{ContainerNode, PrimitiveNode, SqlParserMetaQuery};
use super::reachability::Reachability;
use super::{sqlparser_node_extractor, visitable_trait_impls::VisitableImpl};
use proc_macro2::TokenStream;

use inflector::Inflector;

use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use std::process::Command;
use std::{collections::HashSet, fs::File, io::Write, path::PathBuf};

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

    pub fn generate_visitable_impls(&self, dest_file: &PathBuf, reachability_debug_file: &PathBuf) {
        let reachability = Reachability::derive(&self.meta);

        let mut file = File::create(reachability_debug_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &reachability_debug_file.display()));

        for (ty, source_node_reachable) in &reachability {
            let _ = file
                .write(format!("{} {}\n", source_node_reachable, ty.to_token_stream()).as_bytes());
        }

        let mut generated_code = TokenStream::new();

        let main_nodes = self.meta.main_nodes();
        let primitive_nodes = self
            .meta
            .primitive_nodes()
            .iter()
            .map(|pn| pn.type_path().path.segments.last().unwrap().ident.clone())
            .collect::<HashSet<_>>();

        let visitable_impls_for_main_nodes = main_nodes.iter().map(|(type_path, type_def)| {
            VisitableImpl::new(type_path, type_def, &reachability, &primitive_nodes)
        });

        let visitable_impls_for_primitive_nodes =
            self.impl_visitable_for_primitive_nodes(self.meta.primitive_nodes());

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

    pub fn generated_node_enum(&self, dest_file: &PathBuf) {
        let mut output = proc_macro2::TokenStream::new();

        let main_nodes = self.meta.main_nodes();
        let main_nodes_variants = main_nodes.iter().map(|(type_path, _)| {
            let ident = &type_path.path.segments.last().unwrap().ident;
            let display_ident = ident.to_string();
            let format_string = display_ident.to_string();
            quote! {
                #[display(fmt = #format_string)]
                #ident(&'ast #type_path),
            }
        });
        let mut main_node_from_impls = proc_macro2::TokenStream::new();
        for type_path in main_nodes.iter().map(|(type_path, _)| type_path) {
            let ident = &type_path.path.segments.last().unwrap().ident;
            main_node_from_impls.append_all(quote! {
                #[automatically_derived]
                impl<'ast> From<&'ast #type_path> for Node<'ast> {
                    fn from(value: &'ast #type_path) -> Self {
                        Self::#ident(value)
                    }
                }

                #[automatically_derived]
                impl<'ast> TryFrom<Node<'ast>> for FromWrapper<&'ast #type_path> {
                    type Error = ();

                    fn try_from(value: Node<'ast>) -> Result<Self, Self::Error> {
                        match value {
                            Node::#ident(node) => Ok(FromWrapper(node)),
                            _ => Err(())
                        }
                    }
                }
            });
        }

        let primitive_nodes = self.meta.primitive_nodes();
        let primitive_nodes_variants = primitive_nodes.iter().map(|primitive_node| {
            let ident = primitive_node.variant_ident();
            let display_ident = ident.to_string();
            let format_string = if display_ident.contains("BigDecimal") || display_ident == "String"
            {
                display_ident.to_string()
            } else {
                display_ident.to_lowercase().to_string()
            };
            let type_path = primitive_node.type_path();
            quote! {
                #[display(fmt = #format_string)]
                #ident(&'ast #type_path),
            }
        });
        let mut primitive_node_from_impls = proc_macro2::TokenStream::new();
        for pn in primitive_nodes.iter() {
            let ident = pn.variant_ident();
            let type_path = pn.type_path();
            primitive_node_from_impls.append_all(quote! {
                #[automatically_derived]
                impl<'ast> From<&'ast #type_path> for Node<'ast> {
                    fn from(value: &'ast #type_path) -> Self {
                        Self::#ident(value)
                    }
                }

                #[automatically_derived]
                impl<'ast> TryFrom<Node<'ast>> for FromWrapper<&'ast #type_path> {
                    type Error = ();

                    fn try_from(value: Node<'ast>) -> Result<Self, Self::Error> {
                        match value {
                            Node::#ident(node) => Ok(FromWrapper(node)),
                            _ => Err(())
                        }
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
            let container_variant_ident = format_ident!(
                "{}",
                &type_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_string()
                    .to_pascal_case()
            );
            let container_enum_ident = format_ident!("{}Of", &container_variant_ident);

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
                #variant_ident(&'ast #type_path),
            };
            let from_impl = quote! {
                #[automatically_derived]
                impl<'ast> From<&'ast #type_path> for #container_enum_ident<'ast> {
                    fn from(value: &'ast #type_path) -> Self {
                        Self::#variant_ident(value)
                    }
                }

                #[automatically_derived]
                impl<'ast> TryFrom<Node<'ast>> for FromWrapper<&'ast #type_path>
                where
                    Self: TryFrom<#container_enum_ident<'ast>, Error = ()>,
                {
                    type Error = ();

                    fn try_from(value: Node<'ast>) -> Result<Self, Self::Error> {
                        match value {
                            Node::#container_variant_ident(container) => container.try_into(),
                            _ => Err(())
                        }
                    }
                }

                #[automatically_derived]
                impl<'ast> TryFrom<#container_enum_ident<'ast>> for FromWrapper<&'ast #type_path> {
                    type Error = ();

                    fn try_from(value: #container_enum_ident<'ast>) -> Result<Self, Self::Error> {
                        match value {
                            #container_enum_ident::#variant_ident(container) => Ok(FromWrapper(container)),
                            _ => Err(())
                        }
                    }
                }
            };

            let display_ident = variant_ident.to_string();

            match node {
                ContainerNode::Box(_) => {
                    let format_string = format!("Box<{}>", &display_ident);
                    box_of_variants.append_all(quote! {
                        #[display(fmt = #format_string)]
                        #variant
                    });
                    box_of_from_impls.append_all(from_impl);
                }
                ContainerNode::Vec(_) => {
                    let format_string = format!("Vec<{}>", &display_ident);
                    vec_of_variants.append_all(quote! {
                        #[display(fmt = #format_string)]
                        #variant
                    });
                    vec_of_from_impls.append_all(from_impl);
                }
                ContainerNode::Option(_) => {
                    let format_string = format!("Option<{}>", &display_ident);
                    option_of_variants.append_all(quote! {
                        #[display(fmt = #format_string)]
                        #variant
                    });
                    option_of_from_impls.append_all(from_impl);
                }
            }
        }

        output.append_all(quote! {

            #[doc = indoc! {"
            A `sqlparser` AST node that's a `Box` of something.
            "}]
            #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, derive_more::Display)]
            #[display(fmt = "{}")]
            pub enum BoxOf<'ast> {
                #box_of_variants
            }

            impl<'ast> Copy for BoxOf<'ast> {}

            #[doc = indoc! {"
            A `sqlparser` AST node that's an `Option` of something.
            "}]
            #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, derive_more::Display)]
            #[display(fmt = "{}")]
            pub enum OptionOf<'ast> {
                #option_of_variants
            }

            impl<'ast> Copy for OptionOf<'ast> {}

            #[doc = indoc! {"
            A `sqlparser` AST node that's a `Vec` of something.
            "}]
            #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, derive_more::Display)]
            #[display(fmt = "{}")]
            pub enum VecOf<'ast> {
                #vec_of_variants
            }

            impl<'ast> Copy for VecOf<'ast> {}

            #[doc = indoc! {"
            An enum with variants a variant for all `sqlparser` AST node types.

            Each variant contains a reference to an AST node of a specific type.

            `Node::Box(BoxOf::...)`, `Node::Vec(VecOf::...)` and
            `Node::Option(OptionOf::...)` represent nodes that are themselves
            container types.

            See: [`BoxOf`], [`VecOf`] and [`OptionOf`].

            `Node` implements [`TryFrom`] and [`From`] to support conversion from
            raw node types.
            "}]
            #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, derive_more::Display)]
            #[display(fmt = "{}")]
            pub enum Node<'ast> {
                #(#main_nodes_variants)*
                #(#primitive_nodes_variants)*

                #[display(fmt = "{}", _0)]
                Box(BoxOf<'ast>),

                #[display(fmt = "{}", _0)]
                Option(OptionOf<'ast>),

                #[display(fmt = "{}", _0)]
                Vec(VecOf<'ast>),
            }

            impl<'ast> Copy for Node<'ast> {}

            #box_of_from_impls
            #vec_of_from_impls
            #option_of_from_impls

            #primitive_node_from_impls
            #main_node_from_impls

            #[automatically_derived]
            impl<'ast, T> From<&'ast Box<T>> for Node<'ast>
            where
                BoxOf<'ast>: From<&'ast Box<T>>,
            {
                fn from(value: &'ast Box<T>) -> Self {
                    Self::Box(BoxOf::from(value))
                }
            }

            #[automatically_derived]
            impl<'ast, T> From<&'ast Vec<T>> for Node<'ast>
            where
                VecOf<'ast>: From<&'ast Vec<T>>,
            {
                fn from(value: &'ast Vec<T>) -> Self {
                    Self::Vec(VecOf::from(value))
                }
            }

            #[automatically_derived]
            impl<'ast, T> From<&'ast Option<T>> for Node<'ast>
            where
                OptionOf<'ast>: From<&'ast Option<T>>,
            {
                fn from(value: &'ast Option<T>) -> Self {
                    Self::Option(OptionOf::from(value))
                }
            }
        });

        let mut file = File::create(dest_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

        let formatted = prettyplease::unparse(
            &syn::parse_file(&output.to_string())
                .expect("BUG! Generated Rust code could not be parsed"),
        );

        file.write_all(formatted.as_bytes())
            .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display()));
    }

    pub(crate) fn impl_visitable_for_primitive_nodes(
        &self,
        primitive_nodes: impl IntoIterator<Item = PrimitiveNode>,
    ) -> TokenStream {
        let mut output = proc_macro2::TokenStream::new();

        for node in primitive_nodes {
            let type_path = node.type_path();

            output.append_all(quote! {
                #[automatically_derived]
                impl<'ast> crate::Visitable<'ast> for #type_path {
                    fn accept<State, E, V>(
                        &'ast self,
                        visitor: &V,
                        state: State,
                    ) -> crate::VisitorControlFlow<'ast, State, E>
                    where
                        E: std::error::Error + std::fmt::Debug,
                        V: crate::Visitor<'ast, State, E>
                    {
                        visit(
                            self,
                            visitor,
                            state,
                            #[allow(unused_variables)]
                            |visitor, state| {
                                crate::flow::cont(state)
                            }
                        )
                    }
                }
            });
        }

        output
    }

    /// Creates a macro_rules macro that can generate a match statement for the
    /// Node enum with a handler for the match arms provided by the macro
    /// caller.
    pub fn generate_node_enum_match_macro(&self, dest_file: &PathBuf) {
        let mut output = proc_macro2::TokenStream::new();

        let main_nodes = self.meta.main_nodes();
        let main_node_variants = main_nodes.iter().map(|(type_path, _)| {
            let ident = &type_path.path.segments.last().unwrap().ident;
            quote! {
                #ident(concrete_node)
            }
        });

        let primitive_nodes = self.meta.primitive_nodes();
        let primitive_node_variants = primitive_nodes.iter().map(|primitive_node| {
            let ident = primitive_node.variant_ident();
            quote! {
                #ident(concrete_node)
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

            let variant = quote!(#variant_ident(concrete_node));

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
        output.append_all(
            quote! {
                #[doc(hidden)]
                macro_rules! dispatch_node {
                    ($node:ident, $visitor:expr, $action:ident, $state:expr) => {
                        match $node {
                            #( Node::#main_node_variants => $visitor.$action(concrete_node, $state),)*

                            #( Node::#primitive_node_variants => $visitor.$action(concrete_node, $state),)*

                            #( Node::Box(crate::BoxOf::#box_of_variants) => $visitor.$action(concrete_node, $state),)*

                            #( Node::Option(crate::OptionOf::#option_of_variants) => $visitor.$action(concrete_node, $state),)*

                            #( Node::Vec(crate::VecOf::#vec_of_variants) => $visitor.$action(concrete_node, $state),)*
                        }
                    }
                }
            }
        );

        let mut file = File::create(dest_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

        file.write_all(output.to_string().as_bytes())
            .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display()));

        // rustfmt <filename>
        Command::new("rustfmt")
            .args([dest_file.to_str().unwrap()])
            .output()
            .expect("failed to execute formatting command");
    }
}
