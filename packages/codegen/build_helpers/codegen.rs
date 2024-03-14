use super::reachability::Reachability;
use super::{sqlparser_node_extractor, visitable_trait_impls::VisitableImpl};
use proc_macro2::TokenStream;
use sqltk_meta::{ContainerNode, PrimitiveNode, SqlParserMetaQuery};
use sqltk_syn_helpers::generics;

use inflector::Inflector;

use quote::{format_ident, quote, ToTokens, TokenStreamExt};

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

    pub fn generate_node_list_file(&self, dest_file: &PathBuf) {
        // FIXME: ensure sqlparser is built with correct set of features
        let meta = sqlparser_node_extractor::extract(vec![]);

        let file = File::create(dest_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

        serde_json::to_writer_pretty(&file, &meta)
            .expect("Failed to serialise sqlparser metadata to file")
    }

    pub fn generated_sql_node_enum(&self, dest_file: &PathBuf) {
        let mut output = proc_macro2::TokenStream::new();

        let main_nodes = self.meta.main_nodes();
        let main_nodes_variants = main_nodes.iter().map(|(type_path, _)| {
            let ident = &type_path.path.segments.last().unwrap().ident;
            quote! {
                #ident(&'ast #type_path),
            }
        });
        let mut main_node_from_impls = proc_macro2::TokenStream::new();
        for type_path in main_nodes.iter().map(|(type_path, _)| type_path) {
            let ident = &type_path.path.segments.last().unwrap().ident;
            main_node_from_impls.append_all(quote! {
                impl<'ast> From<&'ast #type_path> for SqlNode<'ast> {
                    fn from(value: &'ast #type_path) -> Self {
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
                #ident(&'ast #type_path),
            }
        });
        let mut primitive_node_from_impls = proc_macro2::TokenStream::new();
        for pn in primitive_nodes.iter() {
            let ident = pn.variant_ident();
            let type_path = pn.type_path();
            primitive_node_from_impls.append_all(quote! {
                impl<'ast> From<&'ast #type_path> for SqlNode<'ast> {
                    fn from(value: &'ast #type_path) -> Self {
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
                #variant_ident(&'ast #type_path),
            };
            let from_impl = quote! {
                impl<'ast> From<&'ast #type_path> for #container_enum_ident<'ast> {
                    fn from(value: &'ast #type_path) -> Self {
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
            pub enum SqlNode<'ast> {
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
            impl<'ast, T> From<&'ast Box<T>> for SqlNode<'ast>
            where
                BoxOf<'ast>: From<&'ast Box<T>>,
            {
                fn from(value: &'ast Box<T>) -> Self {
                    Self::Box(BoxOf::from(value))
                }
            }

            #[automatically_derived]
            impl<'ast, T> From<&'ast Vec<T>> for SqlNode<'ast>
            where
                VecOf<'ast>: From<&'ast Vec<T>>,
            {
                fn from(value: &'ast Vec<T>) -> Self {
                    Self::Vec(VecOf::from(value))
                }
            }

            #[automatically_derived]
            impl<'ast, T> From<&'ast Option<T>> for SqlNode<'ast>
            where
                OptionOf<'ast>: From<&'ast Option<T>>,
            {
                fn from(value: &'ast Option<T>) -> Self {
                    Self::Option(OptionOf::from(value))
                }
            }

            #[automatically_derived]
            impl<'ast> std::fmt::Display for SqlNode<'ast> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match_sql_node_enum!(self, |node| { write!(f, "{}", crate::display_type_name_of_value(node)) })
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
                    fn accept<State, VD>(
                        &'ast self,
                        visitor: &VD,
                        state: State,
                    ) -> crate::VisitorControlFlow<State>
                    where
                        VD: crate::VisitorDispatch<'ast, State>
                    {
                        crate::visit(
                            crate::SqlNode::from(self),
                            visitor,
                            state,
                            #[allow(unused_variables)]
                            |visitor, state| {
                                crate::Flow::cont(state)
                            }
                        )
                    }
                }
            });
        }

        output
    }
}
