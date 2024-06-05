use core::cmp::Reverse;
// use core::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{Fields, Ident, ItemEnum, Type, TypePath};

use super::generics::{self};
use super::meta::{SqlParserTypeDef, SqlParserTypeDefKind};

pub(crate) struct VisitableImpl<'a> {
    node: &'a TypePath,
    def: &'a SqlParserTypeDef,
    reachability: &'a HashMap<Ident, bool>,
    terminal_nodes: &'a HashSet<Ident>,
}

impl<'a> ToTokens for VisitableImpl<'a> {
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

        tokens.append_all(quote! {
            #[automatically_derived]
            impl<'ast> crate::Visitable<'ast> for #path {
                fn accept<V>(
                    &'ast self,
                    visitor: &V,
                    state: V::State,
                ) -> ControlFlow<crate::Break<V::State, V::Error>, V::State>
                    where
                        V: crate::Visitor<'ast>
                {
                    visit(self, visitor, state, #[allow(unused_variables)] |visitor, state| {
                        #[allow(unused_mut)]
                        let mut state = state;
                        #body
                        use crate::visitor_extensions::VisitorExtensions;
                        visitor.continue_with_state(state)
                    })
                }
            }
        })
    }
}

impl<'a> VisitableImpl<'a> {
    pub(crate) fn new(
        node: &'a TypePath,
        def: &'a SqlParserTypeDef,
        reachability: &'a HashMap<Ident, bool>,
        terminal_nodes: &'a HashSet<Ident>,
    ) -> Self {
        Self {
            node,
            def,
            reachability,
            terminal_nodes,
        }
    }

    fn walk_struct_fields(&self, fields: &Fields) -> TokenStream {
        let mut tokens = TokenStream::new();
        match fields {
            Fields::Named(named) => {
                let mut fields = named.named.iter().enumerate().collect::<Vec<_>>();
                fields.sort_by_key(|(idx, f)| {
                    (Reverse(self.field_is_source_node_reachable(f)), *idx)
                });
                for (_, field) in fields.iter() {
                    let ident = field.ident.clone().unwrap();
                    tokens.append_all(quote! {
                        state = self.#ident.accept(visitor, state)?;
                    });
                }
            }
            Fields::Unnamed(unnamed) => {
                let mut fields = unnamed.unnamed.iter().enumerate().collect::<Vec<_>>();
                fields.sort_by_key(|(idx, f)| {
                    (Reverse(self.field_is_source_node_reachable(f)), *idx)
                });

                for (idx, _) in fields.iter() {
                    let field_idx = syn::Index::from(*idx);
                    tokens.append_all(quote! {
                        state = self.#field_idx.accept(visitor, state)?;
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
                let mut fields = named.named.iter().enumerate().collect::<Vec<_>>();
                fields.sort_by_key(|(idx, f)| {
                    (Reverse(self.field_is_source_node_reachable(f)), *idx)
                });

                for (_, field) in fields.iter() {
                    let ident = field.ident.clone().unwrap();
                    tokens.append_all(quote! {
                        state = #ident.accept(visitor, state)?;
                    });
                }
            }
            Fields::Unnamed(unnamed) => {
                let mut fields = unnamed.unnamed.iter().enumerate().collect::<Vec<_>>();
                fields.sort_by_key(|(idx, f)| {
                    (Reverse(self.field_is_source_node_reachable(f)), *idx)
                });

                for (idx, _) in fields.iter() {
                    let ident = format_ident!("field{}", idx);
                    tokens.append_all(quote! {
                        state = #ident.accept(visitor, state)?;
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

    fn normalise_ty(&self, ty: &Type) -> Ident {
        let ty: TypePath = syn::parse_quote!(#ty);
        let ty = generics::innermost_generic_type(&ty);

        ty.path.segments.last().unwrap().ident.clone()
    }

    fn field_is_source_node_reachable(&self, field: &syn::Field) -> bool {
        let normalised = self.normalise_ty(&field.ty);
        if self.terminal_nodes.contains(&normalised) {
            false
        } else {
            *self.reachability.get(&normalised).unwrap_or_else(|| {
                panic!("Could not find type {:?} in reachability data", &field.ty)
            })
        }
    }
}
