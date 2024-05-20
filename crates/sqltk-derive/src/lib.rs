use std::{any::type_name_of_val, collections::HashSet};

use deluxe::{Error, ExtractAttributes, ParseMetaItem, ParseMode};
use indoc::indoc;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::ParseStream, parse2, punctuated::Punctuated, spanned::Spanned, token::Comma, Block,
    Data, DataEnum, DeriveInput, Fields, GenericParam, Generics, Ident, Type, TypeParam, TypePath,
    Variant, WhereClause, WherePredicate,
};

/// Derives [`Visitor`](sqltk::Visitor) for a `struct` or `enum`.
///
/// # Deriving `Visitor` for a struct
///
/// 1. `forward` - the derived `Visitor` will delegate to a field of the struct
/// that implements `Visitor`. The value of the attribute is the identifier of
/// the field.
///
/// 2. `enter` and/or `exit` - allows inline definition of the body of an
/// [`Visitor::enter`](sqltk_core::Visitor::enter) and/or an
/// [`Visitor::enter`](sqltk_core::Visitor::exit) method. One or both
/// implementations can be provided. The inline definition should be written in
/// curly braces. `node` and `state` parameters will be in scope and can be
/// freely used in the inline definition.
///
/// 3. `children` - use this for generating compound `Visitor`s which delegate
/// their job to one or more child visitors. The child visitors MUST implement
/// [`Default`]. The order of invocation of child visitors is in declaration
/// order for `enter` and reverse declaration order for `exit`.
///
/// The above three styles are mutually exclusive.
///
/// # Deriving `Visitor` for an enum
///
/// There is only one supported derivation for enums. Every enum variant must be
/// a single-field tuple variant, where the field type is a `Visitor`
/// implementation. The derived `Visitor` will pattern match `self` to pick out
/// the correct variant and forward to its `Visitor`.
///
/// # Additional options
///
/// `state_ty` - use this to specify a concrete state type.
///
/// If `state_ty` is not specified then a `State` generic parameter will be
/// inserted into the `impl` generics.
///
/// `error_ty` - use this to specify a concrete error type.
///
/// If `error_ty` is not specified it will default to `std::convert::Infallible`.
///
/// # Bounds
///
/// All bounds on the `struct` or `enum` will be copied to the derived `Visitor`
/// implementation. This is necessary to support generic `State` parameters.
///
/// An additional bound will be generated for the error type:
/// `::std::error::Error + ::std::fmt::Debug`.
///
/// # Limitations
///
/// Currently the only way to get State bounds into the derived `Visitor` is to
/// copy those from the struct or enum. This can create ugliness if the type
/// does not make use of the `State` generic other than to specify bounds
/// because the compiler will force the `State` and `'ast` lifetime parameters
/// to be referenced. Which means having to resort to using `PhantomData`
/// fields.
///
/// A feature to provide bounds directly to the macro is on the roadmap.
///
/// Another potential gotcha (that is unlikely to be a problem in practice) is
/// that the macro assume the only lifetime used by the struct or enum is `'ast`.
///
/// The macro covers a lot of situations but there are still going to be
/// times when it makes more sense to implement the `Visitor` by hand, such as
/// when an implementation is complex and is not suited to being inlined
/// via `enter` or `exit`.
///
/// Your IDE will be more effective when writing long functions as first class
/// Rust code and not as syntax fragments in a macro invocation.
#[proc_macro_derive(Visitor, attributes(visitor))]
pub fn derive_visitor(input: TokenStream) -> TokenStream {
    let mut di: Result<DeriveInput, _> = parse2(input.into());
    match di {
        Ok(ref mut di) => match deluxe::extract_attributes::<_, VisitorAttributes>(di) {
            Ok(visitor_attributes) => VisitorImpl::try_from(VisitorInput {
                ident: di.ident.clone(),
                data: di.data.clone(),
                generics: di.generics.clone(),
                attributes: visitor_attributes,
                span: di.span(),
            })
            .map(|visitor_impl| visitor_impl.to_token_stream().into())
            .unwrap_or_else(|err| err.to_compile_error().into()),
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}

#[derive(Debug, ExtractAttributes)]
#[deluxe(attributes(visitor))]
struct VisitorAttributes {
    state_ty: Option<SynTypePath>,
    error_ty: Option<SynTypePath>,
    enter: Option<SynBlock>,
    exit: Option<SynBlock>,
    forward: Option<Ident>,
    children: Option<Vec<SynTypePath>>,
}

#[derive(Debug)]
struct VisitorInput {
    ident: Ident,
    data: Data,
    generics: Generics,
    attributes: VisitorAttributes,
    span: Span,
}

struct VisitorImpl {
    ident: Ident,
    generics: Generics,
    impl_state_ty: Option<TypePath>,
    trait_state_ty: TypePath,
    trait_error_ty: TypePath,
    state_ty_is_concrete: bool,
    data: VisitorData,
}

impl TryFrom<VisitorInput> for VisitorImpl {
    type Error = syn::Error;

    fn try_from(input: VisitorInput) -> Result<Self, Self::Error> {
        let data = match input.data {
            Data::Enum(data_enum) => VisitorData::try_from(data_enum),
            Data::Struct(_) => VisitorData::try_from((
                input.attributes.children,
                input.attributes.forward,
                input.attributes.enter,
                input.attributes.exit,
            )),
            Data::Union(_) => Err(syn::Error::new(
                input.span,
                "Visitor can only be dervived for structs or enums",
            )),
        };
        data.map(|data| VisitorImpl {
            ident: input.ident,
            generics: input.generics,
            impl_state_ty: match &input.attributes.state_ty {
                Some(_) => None,
                None => Some(parse2::<TypePath>(quote!(State)).unwrap()),
            },
            trait_state_ty: input
                .attributes
                .state_ty
                .as_ref()
                .map_or(parse2::<TypePath>(quote!(State)).unwrap(), |ty| ty.0.clone()),
            trait_error_ty: input.attributes.error_ty.map_or(
                parse2::<TypePath>(quote!(::std::convert::Infallible)).unwrap(),
                |ty| ty.0,
            ),
            state_ty_is_concrete: input.attributes.state_ty.is_some(),
            data,
        })
    }
}

#[derive(Debug)]
enum VisitorData {
    Struct(VisitorStruct),
    Enum(VisitorEnum),
}

impl
    TryFrom<(
        Option<Vec<SynTypePath>>,
        Option<Ident>,
        Option<SynBlock>,
        Option<SynBlock>,
    )> for VisitorData
{
    type Error = syn::Error;

    fn try_from(
        value: (
            Option<Vec<SynTypePath>>,
            Option<Ident>,
            Option<SynBlock>,
            Option<SynBlock>,
        ),
    ) -> Result<Self, Error> {
        let (children, forward, enter, exit) = value;
        VisitorStruct::try_from((children, forward, enter, exit))
            .map(|value| VisitorData::Struct(value))
    }
}

impl TryFrom<DataEnum> for VisitorData {
    type Error = syn::Error;

    fn try_from(value: DataEnum) -> Result<Self, Error> {
        let result = value
            .variants
            .into_iter()
            .map(VisitorEnumVariant::try_from)
            .collect::<Result<Vec<_>, _>>();

        match result {
            Ok(visitor_enum_variants) => Ok(Self::Enum(VisitorEnum(visitor_enum_variants))),
            Err((span, msg)) => Err(syn::Error::new(span, msg)),
        }
    }
}

impl
    TryFrom<(
        Option<Vec<SynTypePath>>,
        Option<Ident>,
        Option<SynBlock>,
        Option<SynBlock>,
    )> for VisitorStruct
{
    type Error = syn::Error;

    fn try_from(
        value: (
            Option<Vec<SynTypePath>>,
            Option<Ident>,
            Option<SynBlock>,
            Option<SynBlock>,
        ),
    ) -> Result<Self, Self::Error> {
        let (children, forward, enter, exit) = value;

        match (children, forward, enter, exit) {
            (Some(children), None, None, None) => Ok(VisitorStruct::Children(
                children.into_iter().map(|v| v.0).collect(),
            )),
            (None, Some(forward), None, None) => Ok(VisitorStruct::Forward(forward)),
            (None, None, enter @ Some(_), exit) | (None, None, enter, exit @ Some(_)) => {
                VisitorStructEnterAndExit::try_from((enter, exit))
                    .map(|value| VisitorStruct::Inline(value))
            }
            _ => Err(syn::Error::new(
                Span::call_site(),
                String::from(indoc! {r#"
                    At least one of the following mutually exclusive options must be specified when deriving `Visitor` for a struct:
                        1. `enter` and/or `exit`, or
                        2. `forward`, or
                        3. `children`
                    "#}
                ),
            )),
        }
    }
}

#[derive(Debug)]
enum VisitorStruct {
    Inline(VisitorStructEnterAndExit),
    Forward(Ident),

    Children(Vec<TypePath>),
}

#[derive(Debug)]
struct VisitorStructEnterAndExit(Block, Block);

impl TryFrom<(Option<SynBlock>, Option<SynBlock>)> for VisitorStructEnterAndExit {
    type Error = syn::Error;

    fn try_from(value: (Option<SynBlock>, Option<SynBlock>)) -> Result<Self, Self::Error> {
        match (value.0, value.1) {
            (None, None) => Err(syn::Error::new(Span::call_site(), "At least one of `enter` or `exit` must be specified when Visitor is derived for a struct".to_owned())),
            (Some(SynBlock(enter)), None) => Ok(Self(
                enter,
                parse2::<Block>(quote!( { ::sqltk_core::flow::cont(state) })).unwrap()
            )),
            (None, Some(SynBlock(exit))) => Ok(Self(
                parse2::<Block>(quote!( { ::sqltk_core::flow::cont(state) })).unwrap(),
                exit
            )),
            (Some(SynBlock(enter)), Some(SynBlock(exit))) => Ok(Self(
                enter, exit
            )),
        }
    }
}

#[derive(Debug)]
struct VisitorEnum(Vec<VisitorEnumVariant>);

#[derive(Debug)]
struct SynBlock(Block);

impl ParseMetaItem for SynBlock {
    fn parse_meta_item(input: ParseStream, _mode: ParseMode) -> deluxe::Result<Self> {
        match input.parse::<Block>() {
            Ok(block) => Ok(SynBlock(block)),
            Err(err) => Err(deluxe::Error::from(err)),
        }
    }
}

#[derive(Debug, Clone)]
struct SynTypePath(TypePath);

impl ParseMetaItem for SynTypePath {
    fn parse_meta_item(input: ParseStream, _mode: ParseMode) -> deluxe::Result<Self> {
        match input.parse::<Type>() {
            Ok(Type::Path(type_path)) => Ok(SynTypePath(type_path)),
            Ok(ref do_not_want) => Err(deluxe::Error::new_spanned(
                do_not_want,
                String::from(format!(
                    "Expected Type::Path(..), got {}",
                    type_name_of_val(do_not_want)
                )),
            )),
            Err(err) => Err(deluxe::Error::from(err)),
        }
    }
}

#[derive(Debug, Clone)]
struct VisitorEnumVariant(Ident);

impl TryFrom<Variant> for VisitorEnumVariant {
    type Error = (Span, String);

    fn try_from(variant: Variant) -> Result<Self, Self::Error> {
        match variant.fields {
            Fields::Unnamed(ref fields) => {
                let collected = Vec::from_iter(&fields.unnamed);
                if collected.len() == 1 {
                    Ok(Self(variant.ident.clone()))
                } else {
                    Err((
                        variant.span(),
                        String::from("Only tuple variants with one field are permitted"),
                    ))
                }
            }
            _ => Err((
                variant.span(),
                String::from("Only tuple variants with one field are permitted"),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum VisitorMethod {
    Enter,
    Exit,
}

impl ToTokens for VisitorMethod {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            VisitorMethod::Enter => tokens.extend(quote!(enter)),
            VisitorMethod::Exit => tokens.extend(quote!(exit)),
        }
    }
}

struct DispatchVariants(Vec<VisitorEnumVariant>, VisitorMethod);

impl ToTokens for DispatchVariants {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let match_arms = self
            .0
            .iter()
            .map(|variant| MatchArm(variant.clone(), self.1));
        tokens.extend(quote! {
            match self {
                #(#match_arms,)*
            }
        })
    }
}

struct MatchArm(VisitorEnumVariant, VisitorMethod);

impl ToTokens for MatchArm {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.0 .0;
        let method = &self.1;

        tokens.extend(quote! {
            Self::#ident(visitor) => visitor.#method(node, state)
        })
    }
}

impl ToTokens for VisitorImpl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let trait_state_ty = self.trait_state_ty.clone();
        let trait_error_ty = self.trait_error_ty.clone();
        let impl_state_ty = self.impl_state_ty.clone();

        let mut visitor_generics = Generics::default();

        visitor_generics
            .params
            .push(GenericParam::Lifetime(parse2(quote!('ast)).unwrap()));

        let visitor_state_assoc_ty: Type = parse2(quote!(#trait_state_ty)).unwrap();
        let visitor_error_assoc_ty: Type = parse2(quote!(#trait_error_ty)).unwrap();

        let ast_lifetime = GenericParam::Lifetime(parse2(quote!('ast)).unwrap());

        let mut impl_generics = self.generics.clone();

        impl_generics.params.push(ast_lifetime.clone());

        if let Some(ref impl_state_ty) = impl_state_ty {
            impl_generics.params.push(GenericParam::Type(
                parse2::<TypeParam>(quote!(#impl_state_ty)).unwrap(),
            ));
        }

        // Dedupe the params
        let impl_params: HashSet<GenericParam> = HashSet::from_iter(impl_generics.params.clone());
        impl_generics.params = impl_params
            .into_iter()
            .collect::<Punctuated<GenericParam, Comma>>();

        let (_impl_generics, ty_generics, ty_where_clause) = self.generics.split_for_impl();

        let mut trait_bounds: Punctuated<WherePredicate, Comma> = Default::default();
        trait_bounds.push(
            parse2(quote!(#trait_error_ty: ::std::error::Error + ::std::fmt::Debug)).unwrap(),
        );

        let mut where_clause = WhereClause {
            predicates: Default::default(),
            where_token: Default::default(),
        };

        let ty_bounds: Punctuated<WherePredicate, Comma> = match ty_where_clause {
            Some(ty_where_clause) => ty_where_clause.predicates.clone(),
            None => Default::default(),
        };

        where_clause.predicates.extend(trait_bounds);
        where_clause.predicates.extend(ty_bounds);

        let ident = &self.ident;

        let enter_and_exit = match &self.data {
            VisitorData::Enum(VisitorEnum(variants)) => {
                let dispatch_enter = DispatchVariants(variants.clone(), VisitorMethod::Enter);
                let dispatch_exit = DispatchVariants(variants.clone(), VisitorMethod::Exit);

                quote! {
                    fn enter<N: ::sqltk_core::Visitable<'ast>>(
                        &self,
                        node: &#ast_lifetime N,
                        mut state: #trait_state_ty
                    ) -> ::sqltk_core::VisitorControlFlow<#ast_lifetime, #trait_state_ty, #trait_error_ty> {
                        #dispatch_enter
                    }

                    fn exit<N: ::sqltk_core::Visitable<'ast>>(
                        &self,
                        node: &#ast_lifetime N,
                        mut state: #trait_state_ty
                    ) -> ::sqltk_core::VisitorControlFlow<#ast_lifetime, #trait_state_ty, #trait_error_ty> {
                        #dispatch_exit
                    }
                }
            }
            VisitorData::Struct(VisitorStruct::Inline(enter_and_exit)) => {
                let enter = enter_and_exit.0.clone();
                let exit = enter_and_exit.1.clone();

                quote! {
                    fn enter<N: ::sqltk_core::Visitable<'ast>>(
                        &self,
                        node: &#ast_lifetime N,
                        mut state: #trait_state_ty
                    ) -> ::sqltk_core::VisitorControlFlow<#ast_lifetime, #trait_state_ty, #trait_error_ty> {
                        #[allow(unused_braces)]
                        #enter
                    }

                    fn exit<N: ::sqltk_core::Visitable<'ast>>(
                        &self,
                        node: &#ast_lifetime N,
                        mut state: #trait_state_ty
                    ) -> ::sqltk_core::VisitorControlFlow<#ast_lifetime, #trait_state_ty, #trait_error_ty> {
                        #[allow(unused_braces)]
                        #exit
                    }
                }
            }
            VisitorData::Struct(VisitorStruct::Forward(ident)) => {
                quote! {
                    fn enter<N: ::sqltk_core::Visitable<'ast>>(
                        &self,
                        node: &#ast_lifetime N,
                        mut state: #trait_state_ty
                    ) -> ::sqltk_core::VisitorControlFlow<#ast_lifetime, #trait_state_ty, #trait_error_ty> {
                        self.#ident.enter(node, state)
                    }

                    fn exit<N: ::sqltk_core::Visitable<'ast>>(
                        &self,
                        node: &#ast_lifetime N,
                        mut state: #trait_state_ty
                    ) -> ::sqltk_core::VisitorControlFlow<#ast_lifetime, #trait_state_ty, #trait_error_ty> {
                        self.#ident.exit(node, state)
                    }
                }
            }
            VisitorData::Struct(VisitorStruct::Children(children)) => {
                // Create a tuple of (type, term) so that the child visitor can
                // be referenced at both type and term level as required.
                let children = children.clone().into_iter().map(|child| {
                    if self.state_ty_is_concrete {
                        (quote!(#child), quote!(#child))
                    } else {
                        (quote!(#child<Self::State>), quote!(#child::<Self::State>))
                    }
                }).collect::<Vec<_>>();

                let enter_body = children.iter().map(|(child_ty, child_term)|
                    quote! {
                        state = ::sqltk_core::flow::map_break_err::<
                            Self::State,
                            <#child_ty as ::sqltk_core::Visitor>::Error,
                            Self::Error
                        >(
                            #child_term::default().enter(node, state)
                        )?;
                    }
                );

                let mut children = children.clone();
                let exit_body = {
                    children.reverse();
                    children.iter().map(|(child_ty, child_term)|
                        quote! {
                            state = ::sqltk_core::flow::map_break_err::<
                                Self::State,
                                <#child_ty as ::sqltk_core::Visitor>::Error,
                                Self::Error
                            >(
                                #child_term::default().exit(node, state)
                            )?;
                        }
                    )
                };

                quote! {
                    fn enter<N: ::sqltk_core::Visitable<'ast>>(
                        &self,
                        node: &#ast_lifetime N,
                        mut state: #trait_state_ty
                    ) -> ::sqltk_core::VisitorControlFlow<#ast_lifetime, #trait_state_ty, #trait_error_ty> {
                        #(#enter_body)*
                        ::sqltk_core::flow::cont(state)
                    }

                    fn exit<N: ::sqltk_core::Visitable<'ast>>(
                        &self,
                        node: &#ast_lifetime N,
                        mut state: #trait_state_ty
                    ) -> ::sqltk_core::VisitorControlFlow<#ast_lifetime, #trait_state_ty, #trait_error_ty> {
                        #(#exit_body)*
                        ::sqltk_core::flow::cont(state)
                    }
                }
            }
        };

        tokens.extend(quote! {
            impl #impl_generics ::sqltk_core::Visitor #visitor_generics for #ident #ty_generics #where_clause {
                type State = #visitor_state_assoc_ty;
                type Error = #visitor_error_assoc_ty;

                #enter_and_exit
            }
        });
    }
}