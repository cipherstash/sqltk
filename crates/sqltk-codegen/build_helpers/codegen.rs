use super::meta::{PrimitiveNode, SqlParserMetaQuery};
use super::reachability::Reachability;
use super::{sqlparser_node_extractor, visitable_trait_impls::VisitableImpl};
use proc_macro2::TokenStream;

use quote::{quote, ToTokens, TokenStreamExt};

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
                    fn accept<V>(
                        &'ast self,
                        visitor: &V,
                        state: V::State,
                    ) -> crate::VisitorControlFlow<'ast, V::State, V::Error>
                    where
                        V: crate::Visitor<'ast>
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
}
