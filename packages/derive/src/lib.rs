use indoc::indoc;
use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, quote_spanned, TokenStreamExt};
use sqltk_codegen::NODE_LIST;
use sqltk_meta::{SqlParserMeta, SqlParserMetaQuery};
use sqltk_syn_helpers::generics::decompose_generic_type;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, GenericParam, TypePath};

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

static VISITOR_DISPATCH_DERIVE_GENERIC_ERROR: &str = indoc! {"
    VisitorDispatch can only be derived for types with exactly zero or two
    lifetime parameters. Generic type paramters and generic const parameters
    are not permitted.

    To workaround these derivation limitations, define a newtype wrapper that
    instantiates all of the generic types and derive VisitorDispatch
    on the wrapper.

    Additionally, define a blanket Visitor implementation on the
    wrapper for all Visitor implementations implemented by the
    generic type and forward the methods to the generic type.

    See the following example:

    struct MyGenericVisitor<'state, 'ast: 'satte, T: 'state + NodeLogger<'ast>> {
        logger: T
    }

    impl<'state, 'ast: 'state, T: 'state + NodeLogger> Visitor<'state, 'ast, Expr> for MyGenericVisitor<'state, 'ast, T> {
        fn enter(&mut self, node: &'ast Expr) -> EnterControlFlow {
            self.logger.log(node);
            ControlFlow::Continue(Navigation::Visit)
        }
    }

    impl<'state, 'ast: 'state, T> Visitor<'state, 'ast, Statement> for MyGenericVisitor<'state, 'ast, T + 'state> {
        fn enter(&mut self, node: &'ast Expr) -> EnterControlFlow {
            self.logger.log(node);
            ControlFlow::Continue(Navigation::Visit)
        }
    }

    #[derive(VisitorDispatch)]
    struct Wrapper<'state, 'ast: 'state>(MyGenericVisitor<'state, 'ast, OtelLogger>);

    // Implement Visitor<'state, 'ast, N> on Wrapper<'state, 'ast> for all Visitor<'state, 'ast, N> implemented by the generic type.
    impl<'state, 'ast: 'state, N> Visitor<'state, 'ast, N> for Wrapper<'state, 'ast> where MyGenericVisitor<'state, 'ast, OtelLogger>: Visitor<'state, 'ast, N> {
        fn enter(&mut self, node: &'ast N) -> EnterControlFlow {
            self.0.enter(node)
        }

        fn exit(&mut self, node: &'ast N) -> ExitControlFlow {
            self.0.exit(node)
        }
    }
"};

/// Derives [`VisitorDispatch`] for a non-generic type.
///
/// ```
/// # extern crate sqltk_core as sqltk;
/// # use sqltk_derive::*;
/// use sqltk::*;
/// use sqltk::sqlparser::*;
/// use std::ops::ControlFlow;
/// use std::marker::PhantomData;
///
/// #[derive(VisitorDispatch)]
/// struct ExprCounter<'state, 'ast: 'state> {
///     counter: usize,
///     _ast: PhantomData<&'ast ()>,
///     _state: PhantomData<&'state ()>,
/// }
///
/// impl<'state, 'ast: 'state> Visitor<'state, 'ast, ast::Expr> for ExprCounter<'state, 'ast> {
///   fn enter(&mut self, node: &'ast ast::Expr) -> EnterControlFlow {
///     self.counter += 1;
///     ControlFlow::Continue(Navigation::Visit)
///   }
/// }
/// ```
#[proc_macro_derive(VisitorDispatch)]
pub fn derive_visitor_dispatch(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let visitor = &input.ident;
    let generics = &input.generics;
    let krate = resolve_crate();
    let meta = node_meta();

    let mut count_lifetime_params = 0usize;
    let mut count_type_params = 0usize;
    let mut count_const_params = 0usize;

    for gp in generics.params.iter() {
        match gp {
            GenericParam::Lifetime(_) => count_lifetime_params += 1,
            GenericParam::Type(_) => count_type_params += 1,
            GenericParam::Const(_) => count_const_params += 1,
        }
    }

    if (count_type_params, count_const_params) != (0, 0) {
        let err_msg = VISITOR_DISPATCH_DERIVE_GENERIC_ERROR;
        return quote_spanned!( input.span() => compile_error!(#err_msg);).into();
    }

    if count_lifetime_params > 2 {
        let err_msg = VISITOR_DISPATCH_DERIVE_GENERIC_ERROR;
        return quote_spanned!( input.span() => compile_error!(#err_msg);).into();
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut output = proc_macro2::TokenStream::new();
    let mut entries = proc_macro2::TokenStream::new();

    let visitor_static = if count_lifetime_params == 2 {
        quote!(#visitor<'static, 'static>)
    } else if count_lifetime_params == 1 {
        quote!(#visitor<'static>)
    } else {
        quote!(#visitor)
    };

    let visitor_nonstatic = if count_lifetime_params == 2 {
        quote!(#visitor<'s, 'a>)
    } else if count_lifetime_params == 1 {
        quote!(#visitor<'state>)
    } else {
        quote!(#visitor)
    };

    for node in meta.all_nodes() {
        let chunks = decompose_generic_type(&node)
            .iter()
            .map(|tp| tp.path.segments.last().unwrap().ident.to_string())
            .collect::<Vec<_>>();
        let joined_chunks = &chunks.join("Of");
        let type_cased = Inflector::to_pascal_case(joined_chunks);
        let ty_ident: TypePath = syn::parse_str(&type_cased).unwrap();

        entries.append_all(quote! {
            type #ty_ident<'s, 'a: 's> = #krate::dispatch::If<
                {#krate::dispatch::IsVisitor::<#visitor_static, #node>::ANSWER},
                #krate::dispatch::Handle<#visitor_nonstatic, #node>,
                #krate::dispatch::Fallback<#visitor_nonstatic>
            >;
        });
    }

    // TODO: bring Nope and AssumeNotImplemented into scope
    output.append_all(quote! {
        use #krate::Nope as _;
        impl #impl_generics #krate::DispatchTable for #visitor #ty_generics #where_clause {
            #entries
        }
    });

    output.into()
}
