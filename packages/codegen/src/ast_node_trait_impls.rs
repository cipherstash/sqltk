use std::{cmp::Ordering, collections::HashMap};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{Fields, ItemEnum, Type, TypePath};

use crate::{
    meta::{SqlParserTypeDef, SqlParserTypeDefKind},
    reachability::Distance,
    SqlParserMetaQuery,
};

pub(crate) struct AstNodeImpl<'a> {
    node: &'a TypePath,
    def: &'a SqlParserTypeDef,
    reachability: &'a HashMap<TypePath, Distance>,
    meta: &'a SqlParserMetaQuery,
}

impl<'a> ToTokens for AstNodeImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (ref path, ref body) = match &self.def.ty {
            SqlParserTypeDefKind::Enum(item_enum) => (
                self.node,
                self.match_variants(self.node, item_enum, self.def.is_non_exhaustive),
            ),
            SqlParserTypeDefKind::Struct(item_struct) => {
                (self.node, self.walk_struct_fields(&item_struct.fields))
            }
        };

        let short_name = &self.node.path.segments.last().unwrap().ident.to_string();

        tokens.append_all(quote! {
            #[automatically_derived]
            impl<'ast> crate::AstNode<'ast> for #path {
                fn accept_with_node_builder<V: crate::VisitorDispatch<'ast>>(
                    &'ast self,
                    visitor: &mut V,
                    node_builder: &mut crate::NodeBuilder,
                ) -> VisitorControlFlow {
                    crate::visit(node_builder.new_node(self).into(), visitor, #[allow(unused_variables)] |visitor| {
                        #body
                        ControlFlow::Continue(Navigation::Visit)
                    })
                }
            }

            // TODO: include enum variant names too
            #[automatically_derived]
            impl<'ast> std::fmt::Display for crate::DisplayType<#path> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", #short_name)
                }
            }
        })
    }
}

impl<'a> AstNodeImpl<'a> {
    pub(crate) fn new(
        node: &'a TypePath,
        def: &'a SqlParserTypeDef,
        reachability: &'a HashMap<TypePath, Distance>,
        meta: &'a SqlParserMetaQuery,
    ) -> Self {
        Self {
            node,
            def,
            reachability,
            meta,
        }
    }

    fn walk_struct_fields(&self, fields: &Fields) -> TokenStream {
        let mut tokens = TokenStream::new();
        match fields {
            Fields::Named(named) => {
                let mut fields = named.named.iter().collect::<Vec<_>>();
                fields.sort_by(|a, b| self.compare_fields_by_source_node_distance(a, b));
                for field in fields.iter() {
                    let ident = field.ident.clone().unwrap();
                    tokens.append_all(quote! {
                        self.#ident.accept_with_node_builder(visitor, node_builder)?;
                    });
                }
            }
            Fields::Unnamed(unnamed) => {
                let mut fields = unnamed.unnamed.iter().enumerate().collect::<Vec<_>>();
                fields.sort_by(|a, b| self.compare_fields_by_source_node_distance(a.1, b.1));

                for (idx, _) in fields.iter() {
                    let field_idx = syn::Index::from(*idx);
                    tokens.append_all(quote! {
                        self.#field_idx.accept_with_node_builder(visitor, node_builder)?;
                    });
                }
            }
            Fields::Unit => {}
        }
        tokens
    }

    fn walk_variant_fields(&self, fields: &Fields) -> TokenStream {
        let mut tokens = TokenStream::new();
        match fields {
            Fields::Named(named) => {
                let mut fields = named.named.iter().collect::<Vec<_>>();
                fields.sort_by(|a, b| self.compare_fields_by_source_node_distance(a, b));
                for field in fields.iter() {
                    let ident = field.ident.clone().unwrap();
                    tokens.append_all(quote! {
                        #ident.accept_with_node_builder(visitor, node_builder)?;
                    });
                }
            }
            Fields::Unnamed(unnamed) => {
                let mut fields = unnamed.unnamed.iter().enumerate().collect::<Vec<_>>();
                fields.sort_by(|a, b| self.compare_fields_by_source_node_distance(a.1, b.1));
                for (idx, _) in fields.iter() {
                    let ident = format_ident!("field{}", idx);
                    tokens.append_all(quote! {
                        #ident.accept_with_node_builder(visitor, node_builder)?;
                    });
                }
            }
            Fields::Unit => {}
        }
        tokens
    }

    fn match_variants(
        &self,
        path: &syn::TypePath,
        item_enum: &ItemEnum,
        is_non_exhaustive: bool,
    ) -> TokenStream {
        let mut tokens = TokenStream::new();

        for variant in &item_enum.variants {
            let ident = &variant.ident;
            let visit_nodes = self.walk_variant_fields(&variant.fields);

            match &variant.fields {
                Fields::Named(named) => {
                    let field_names = named.named.iter().map(|f| f.ident.clone().unwrap());

                    tokens.append_all(quote! {
                        #path::#ident { #(#field_names,)* } => {
                            #visit_nodes
                        }
                    });
                }
                Fields::Unnamed(unnamed) => {
                    let field_params = unnamed
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(idx, _)| format_ident!("field{}", idx));

                    tokens.append_all(quote! {
                        #path::#ident( #(#field_params,)* ) => {
                            #visit_nodes
                        }
                    });
                }
                Fields::Unit => {
                    tokens.append_all(quote! {
                        #path::#ident => { }
                    });
                }
            }
        }

        let unreachable_match_arm_handler = if is_non_exhaustive {
            quote! { _ => { unreachable!() }}
        } else {
            quote! {}
        };

        quote! {
            match self {
                #tokens
                #unreachable_match_arm_handler
            }
        }
    }

    fn normalise_ty(&self, ty: &Type) -> Option<&TypePath> {
        let ty: TypePath = syn::parse_quote!(#ty);
        let ident = &ty.path.segments.last().unwrap().ident;
        self.meta.lookup_main_node_by_ident(ident)
    }

    fn compare_fields_by_source_node_distance(&self, a: &syn::Field, b: &syn::Field) -> Ordering {
        let reach_a = self
            .normalise_ty(&a.ty)
            .and_then(|ty| self.reachability.get(ty))
            .unwrap_or(&Distance(255));

        let reach_b = self
            .normalise_ty(&b.ty)
            .and_then(|ty| self.reachability.get(ty))
            .unwrap_or(&Distance(255));

        reach_a.cmp(reach_b)
    }
}
