use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{Fields, ItemEnum, ItemStruct, TypePath};

use super::meta::{SqlParserTypeDef, SqlParserTypeDefKind};

pub(crate) struct ApplyTransformImpl<'a> {
    node: &'a TypePath,
    def: &'a SqlParserTypeDef,
}

impl<'a> ToTokens for ApplyTransformImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (ref path, ref body) = match &self.def.ty {
            SqlParserTypeDefKind::Enum(item_enum) => {
                let variants =
                    self.match_variants(self.node, item_enum, self.def.is_non_exhaustive);
                (self.node, variants)
            }
            SqlParserTypeDefKind::Struct(item_struct) => {
                let fields = self.walk_struct_fields(item_struct);
                (self.node, fields)
            }
        };

        tokens.append_all(quote! {
            #[automatically_derived]
            impl<'ast> crate::ApplyTransform<'ast> for #path {
                fn apply_transform<T>(
                    &'ast self,
                    transformer: &T,
                    context: &T::Context
                ) -> Result<Self, T::Error>
                where
                    T: crate::Transform<'ast>
                {
                    #body
                }
            }
        })
    }
}

impl<'a> ApplyTransformImpl<'a> {
    pub(crate) fn new(node: &'a TypePath, def: &'a SqlParserTypeDef) -> Self {
        Self { node, def }
    }

    fn walk_struct_fields(&self, item: &ItemStruct) -> TokenStream {
        let mut tokens = TokenStream::new();
        let fields = &item.fields;

        match fields {
            Fields::Named(named) => {
                let field_names = named.named.iter().map(|f| f.ident.clone().unwrap());

                let transformed_fields = field_names.clone().map(|field_name| {
                    quote! {
                        #field_name: #field_name.apply_transform(transformer, context)?
                    }
                });

                tokens.append_all(quote! {
                    let Self { #(#field_names,)* } = self;
                    transformer.transform(
                        self,
                        Self {
                            #(#transformed_fields,)*
                        },
                        context
                    )
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
                        #field_param.apply_transform(transformer, context)?
                    }
                });

                tokens.append_all(quote! {
                    let Self(#(#field_params,)*) = self;
                    transformer.transform(
                        self,
                        Self(#(#transformed_field_params,)*),
                        context
                    )
                });
            }
            Fields::Unit => tokens.append_all(quote!(Ok(Self))),
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

            match &variant.fields {
                Fields::Named(named) => {
                    let field_names = named.named.iter().map(|f| f.ident.clone().unwrap());

                    let transformed_fields = field_names.clone().map(|field_name| {
                        quote! {
                            #field_name: #field_name.apply_transform(transformer, context)?
                        }
                    });

                    tokens.append_all(quote! {
                        #path::#ident { #(#field_names,)* } => transformer.transform(
                            self,
                            #path::#ident {
                                #(#transformed_fields,)*
                            },
                            context
                        ),
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
                            #field_param.apply_transform(transformer, context)?
                        }
                    });

                    tokens.append_all(quote! {
                        #path::#ident( #(#field_params,)* ) => transformer.transform(
                            self,
                            #path::#ident(
                                #(#transformed_field_params,)*
                            ),
                            context
                        ),
                    });
                }
                Fields::Unit => tokens.append_all(quote! {
                    #path::#ident => transformer.transform(self, #path::#ident, context),
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
