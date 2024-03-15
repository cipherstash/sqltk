use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use sqltk_codegen::NODE_LIST;
use sqltk_meta::{ContainerNode, SqlParserMeta, SqlParserMetaQuery};
use sqltk_syn_helpers::generics::{self};
use syn::{parse::Parse, parse_macro_input, parse_quote, Attribute, DeriveInput, Expr, Type};

/// Derives [`VisitorDispatch`].
///
/// ```
/// # extern crate sqltk_core as sqltk;
/// # use sqltk_derive::*;
/// use sqltk::*;
/// use sqltk::sqlparser::*;
/// use std::ops::ControlFlow;
/// use std::marker::PhantomData;
/// use derive_more::AsMut;
///
/// #[derive(VisitorDispatch)]
/// #[visitor_dispatch(state = usize)]
/// struct ExprCounter;
///
/// impl<'ast> Visitor<'ast, ast::Expr, usize> for ExprCounter {
///     fn enter(&self, node: &'ast ast::Expr, mut counter: usize) -> VisitorControlFlow<usize> {
///         counter += 1usize;
///         Flow::cont(counter)
///     }
/// }
///
/// let dialect = dialect::GenericDialect {};
///
/// let sql = "SELECT 123;";
///
/// let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();
/// let result = ast.accept(&ExprCounter, 0);
///
/// assert!(matches!(result, VisitorControlFlow::Continue(1) ));
/// ```
#[proc_macro_derive(VisitorDispatch, attributes(visitor_dispatch))]
pub fn derive_visitor_dispatch(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let args = parse_visitor_dispatch_attribute(&input.attrs);

    let state_ty: Type = match args {
        Ok(Some(VisitorDispatchArgs { state_ty })) => state_ty,
        Ok(None) => parse_quote!(()),
        Err(err) => {
            return err.into_compile_error().into();
        }
    };

    let visitor = &input.ident;
    let generics = input.generics;
    let krate = resolve_crate();

    let mut modified_generics = generics.clone();

    if !modified_generics
        .lifetimes()
        .any(|lt| lt.lifetime.ident == "ast")
    {
        modified_generics.params.push(parse_quote!('ast));
    }

    let (impl_generics, _, _) = modified_generics.split_for_impl();
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let mut output = proc_macro2::TokenStream::new();

    let dispatch_enter = dispatch_sql_node(state_ty.clone(), DispatchFn::Enter);
    let dispatch_exit = dispatch_sql_node(state_ty.clone(), DispatchFn::Exit);

    output.append_all(quote! {
        impl #impl_generics #krate::VisitorDispatch<'ast, #state_ty> for #visitor #ty_generics #where_clause {
            fn enter(&self, node: #krate::SqlNode<'ast>, state: #state_ty) -> #krate::VisitorControlFlow<#state_ty> {
                #dispatch_enter
            }

            fn exit(&self, node: #krate::SqlNode<'ast>, state: #state_ty) -> #krate::VisitorControlFlow<#state_ty> {
                #dispatch_exit
            }
        }
    });

    output.into()
}

fn node_meta() -> SqlParserMetaQuery {
    let meta: SqlParserMeta = serde_json::from_str(
        String::from_utf8(Vec::from(&NODE_LIST[..]))
            .unwrap()
            .as_str(),
    )
    .unwrap();
    meta.into()
}

fn resolve_crate() -> proc_macro2::TokenStream {
    let found_crate = crate_name("sqltk_core").expect("sqltk_core not found in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident )
        }
    }
}

#[derive(Clone, Copy)]
enum DispatchFn {
    Enter,
    Exit,
}

impl ToTokens for DispatchFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Enter => tokens.append_all(quote!(enter)),
            Self::Exit => tokens.append_all(quote!(exit)),
        }
    }
}

struct SqlNodeVariant {
    match_expr: Expr,
}

impl SqlNodeVariant {
    fn new(match_expr: Expr) -> Self {
        Self { match_expr }
    }
}

struct DispatchSqlNodeVariant {
    sql_node_variant: SqlNodeVariant,
    state_ty: Type,
    dispatch_fn: DispatchFn,
}

impl DispatchSqlNodeVariant {
    fn new(sql_node_variant: SqlNodeVariant, state_ty: Type, dispatch_fn: DispatchFn) -> Self {
        Self {
            sql_node_variant,
            state_ty,
            dispatch_fn,
        }
    }
}

impl ToTokens for DispatchSqlNodeVariant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            sql_node_variant: SqlNodeVariant { match_expr },
            state_ty,
            dispatch_fn,
        } = self;

        tokens.append_all(quote! {
            #match_expr => (&&&Dispatch::<'_, '_, _, _, #state_ty>(self, node, PhantomData)).#dispatch_fn(node, state)
        });
    }
}

fn dispatch_sql_node(state_ty: Type, dispatch_fn: DispatchFn) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();
    let krate = resolve_crate();
    let meta = node_meta();
    let mut sql_node_variants = Vec::<SqlNodeVariant>::new();

    let main_nodes = meta.main_nodes();
    sql_node_variants.extend(main_nodes.iter().map(|(type_path, _)| {
        let ident = &type_path.path.segments.last().unwrap().ident;
        SqlNodeVariant::new(parse_quote!(SqlNode::#ident(node)))
    }));

    let primitive_nodes = meta.primitive_nodes();
    sql_node_variants.extend(primitive_nodes.iter().map(|primitive_node| {
        let variant_ident = primitive_node.variant_ident().0;
        SqlNodeVariant::new(parse_quote!(SqlNode::#variant_ident(node)))
    }));

    let container_nodes = meta.container_nodes();

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

        match node {
            ContainerNode::Box(_) => {
                sql_node_variants.push(SqlNodeVariant::new(
                    parse_quote!(SqlNode::Box(BoxOf::#variant_ident(node))),
                ));
            }
            ContainerNode::Vec(_) => {
                sql_node_variants.push(SqlNodeVariant::new(
                    parse_quote!(SqlNode::Vec(VecOf::#variant_ident(node))),
                ));
            }
            ContainerNode::Option(_) => {
                sql_node_variants.push(SqlNodeVariant::new(
                    parse_quote!(SqlNode::Option(OptionOf::#variant_ident(node))),
                ));
            }
        }
    }

    let dispatch_arms: Vec<DispatchSqlNodeVariant> = sql_node_variants
        .into_iter()
        .map(|snv| DispatchSqlNodeVariant::new(snv, state_ty.clone(), dispatch_fn))
        .collect();

    output.append_all(quote! {
        use #krate::dispatch::specialization::{Dispatch, ViaVisitor, ViaFallback, ViaIgnored};
        use #krate::sqlparser::{self};
        use #krate::bigdecimal::BigDecimal;
        use #krate::{SqlNode, BoxOf, OptionOf, VecOf};
        use core::marker::PhantomData;

        match node {
            #( #dispatch_arms, )*
        }
    });

    output
}

mod kw {
    syn::custom_keyword!(state);
}

// #[visitor_dispatch(state = State)]
fn parse_visitor_dispatch_attribute(
    attrs: &Vec<Attribute>,
) -> syn::Result<Option<VisitorDispatchArgs>> {
    for attr in attrs {
        if attr.path().is_ident("visitor_dispatch") {
            let args: VisitorDispatchArgs = attr.parse_args()?;
            return Ok(Some(args));
        }
    }
    Ok(None)
}

struct VisitorDispatchArgs {
    state_ty: Type,
}

impl Parse for VisitorDispatchArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::state = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        Ok(Self {
            state_ty: input.parse()?,
        })
    }
}
