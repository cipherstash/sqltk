use std::{cmp::Ordering};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{Fields, Type, TypePath, ItemEnum};

use crate::meta::{SqlParserTypeDef, SqlParserTypeDefKind};

pub struct AstNodeImpl<'a>(pub &'a TypePath, pub &'a SqlParserTypeDef);

impl<'a> ToTokens for AstNodeImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (ref path, ref body) = match &self.1.ty {
            SqlParserTypeDefKind::Enum(item_enum) => (
                self.0,
                match_variants(self.0, item_enum, self.1.is_non_exhaustive),
            ),
            SqlParserTypeDefKind::Struct(item_struct) => (self.0, walk_struct_fields(&item_struct.fields)),
        };

        tokens.append_all(quote! {
            #[automatically_derived]
            impl<'ast> crate::AstNode<'ast> for #path {
                fn accept_with_id_iter<V: crate::VisitorDispatch<'ast>>(
                    &'ast self,
                    visitor: &V,
                    node_builder: &mut crate::NodeBuilder,
                ) -> VisitorControlFlow {
                    crate::visit(node_builder.new_node(self).into(), visitor, || {
                        #body
                        ControlFlow::Continue(Navigation::Visit)
                    })
                }
            }
        })
    }
}

fn walk_struct_fields(fields: &Fields) -> TokenStream {
    let mut tokens = TokenStream::new();
    match fields {
        Fields::Named(named) => {
            let mut fields = named.named.iter().collect::<Vec<_>>();
            fields.sort_by(preferred_visit_order_named);
            for field in fields.iter() {
                let ident = field.ident.clone().unwrap();
                tokens.append_all(quote! {
                    self.#ident.accept_with_id_iter(visitor, node_builder)?;
                });
            }
        }
        Fields::Unnamed(unnamed) => {
            let mut fields = unnamed.unnamed.iter().enumerate().collect::<Vec<_>>();
            fields.sort_by(preferred_visit_order_unnamed);

            for (idx, _) in fields.iter() {
                let field_idx = syn::Index::from(*idx);
                tokens.append_all(quote! {
                    self.#field_idx.accept_with_id_iter(visitor, node_builder)?;
                });
            }
        }
        Fields::Unit => {}
    }
    tokens
}

fn walk_variant_fields(fields: &Fields) -> TokenStream {
    let mut tokens = TokenStream::new();
    match fields {
        Fields::Named(named) => {
            let mut fields = named.named.iter().collect::<Vec<_>>();
            fields.sort_by(preferred_visit_order_named);
            for field in fields.iter() {
                let ident = field.ident.clone().unwrap();
                tokens.append_all(quote! {
                    #ident.accept_with_id_iter(visitor, node_builder)?;
                });
            }
        }
        Fields::Unnamed(unnamed) => {
            let mut fields = unnamed.unnamed.iter().enumerate().collect::<Vec<_>>();
            fields.sort_by(preferred_visit_order_unnamed);
            for (idx, _) in fields.iter() {
                let ident = format_ident!("field{}", idx);
                tokens.append_all(quote! {
                    #ident.accept_with_id_iter(visitor, node_builder)?;
                });
            }
        }
        Fields::Unit => {}
    }
    tokens
}

fn match_variants(
    path: &syn::TypePath,
    item_enum: &ItemEnum,
    is_non_exhaustive: bool,
) -> TokenStream {
    let mut tokens = TokenStream::new();

    for variant in &item_enum.variants {
        let ident = &variant.ident;
        let visit_nodes = walk_variant_fields(&variant.fields);

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

fn preferred_visit_order_unnamed(a: &(usize, &syn::Field), b: &(usize, &syn::Field)) -> Ordering {
    compare_types(&a.1.ty, &b.1.ty)
}

fn preferred_visit_order_named(a: &&syn::Field, b: &&syn::Field) -> Ordering {
    compare_types(&a.ty, &b.ty)
}

fn compare_types(_a: &Type, _b: &Type) -> Ordering {
    Ordering::Equal
}
