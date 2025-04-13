use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{Fields, ItemEnum, ItemStruct, TypePath};

use super::meta::{AstNode, SqlParserTypeDefKind};

pub(crate) struct TransformableImpl {
    path: TypePath,
    node: AstNode,
}

impl ToTokens for TransformableImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (ref path, ref body) = match &self.node {
            AstNode::SqlParserTypeDef(def) => match &def.ty {
                SqlParserTypeDefKind::Enum(item_enum) => {
                    let variants = self.match_variants(item_enum, def.is_non_exhaustive);
                    (&self.path, variants)
                }
                SqlParserTypeDefKind::Struct(item_struct) => {
                    let fields = self.walk_struct_fields(item_struct);
                    (&self.path, fields)
                }
            },

            AstNode::TerminalNode(terminal) => {
                let copy_or_clone = if terminal.is_primitive() {
                    quote!(*self)
                } else {
                    quote!(self.clone())
                };

                (
                    &self.path,
                    quote!(transformer.transform(node_path, #copy_or_clone)?),
                )
            }
        };

        tokens.append_all(quote! {
            #[automatically_derived]
            impl<'ast> crate::Transformable<'ast> for #path {
                fn apply_transform_with_path<T>(
                    &'ast self,
                    transformer: &mut T,
                    node_path: &mut crate::NodePath<'ast>,
                ) -> Result<Self, T::Error>
                where
                    T: crate::Transform<'ast>
                {
                    node_path.push(self);
                    let transformed = { #body };
                    let transformed = transformer.transform(node_path, transformed)?;
                    node_path.pop();
                    Ok(transformed)
                }
            }
        })
    }
}

impl TransformableImpl {
    pub(crate) fn new(path: TypePath, node: AstNode) -> Self {
        Self { path, node }
    }

    fn walk_struct_fields(&self, item: &ItemStruct) -> TokenStream {
        let mut tokens = TokenStream::new();
        let fields = &item.fields;

        match fields {
            Fields::Named(named) => {
                let field_names = named.named.iter().map(|f| f.ident.clone().unwrap());

                let transformed_fields = field_names.clone().map(|field_name| {
                    quote! {
                        #field_name: #field_name.apply_transform_with_path(transformer, node_path)?
                    }
                });

                tokens.append_all(quote! {
                    let Self { #(#field_names,)* } = self;
                    Self { #(#transformed_fields,)* }
                });
            }
            Fields::Unnamed(unnamed) => {
                let field_params = unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(idx, _)| format_ident!("field{}", idx));

                let transformed_field_params = field_params.clone().map(|field_param| {
                    quote! {
                        #field_param.apply_transform_with_path(transformer, node_path)?
                    }
                });

                tokens.append_all(quote! {
                    let Self(#(#field_params,)*) = self;
                    Self(#(#transformed_field_params,)*)
                });
            }
            Fields::Unit => tokens.append_all(quote!(Ok(Self))),
        }

        tokens
    }

    fn match_variants(&self, item_enum: &ItemEnum, is_non_exhaustive: bool) -> TokenStream {
        let mut tokens = TokenStream::new();
        let path = &self.path;

        for variant in &item_enum.variants {
            let ident = &variant.ident;

            match &variant.fields {
                Fields::Named(named) => {
                    let field_names = named.named.iter().map(|f| f.ident.clone().unwrap());

                    let transformed_fields = field_names.clone().map(|field_name| {
                        quote! {
                            #field_name: #field_name.apply_transform_with_path(transformer, node_path)?
                        }
                    });

                    tokens.append_all(quote! {
                        #path::#ident { #(#field_names,)* } => {
                            #path::#ident { #(#transformed_fields,)* }
                        }
                    });
                }
                Fields::Unnamed(unnamed) => {
                    let field_params = unnamed
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(idx, _)| format_ident!("field{}", idx));

                    let transformed_field_params = field_params.clone().map(|field_param| {
                        quote! {
                            #field_param.apply_transform_with_path(transformer, node_path)?
                        }
                    });

                    tokens.append_all(quote! {
                        #path::#ident( #(#field_params,)* ) => {
                            #path::#ident( #(#transformed_field_params,)* )
                        }
                    });
                }
                Fields::Unit => tokens.append_all(quote! {
                    #path::#ident => {
                        transformer.transform(
                            node_path,
                            #path::#ident
                        )?
                    }
                }),
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
}
