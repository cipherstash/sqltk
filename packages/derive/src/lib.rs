use indoc::indoc;
use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, quote_spanned, TokenStreamExt};
use sqltk_meta::{SqlParserMeta, SqlParserMetaQuery};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, TypePath};

fn node_meta() -> SqlParserMetaQuery {
    let meta: SqlParserMeta = serde_json::from_str(
        String::from_utf8(Vec::from(&sqltk_codegen::NODE_LIST[..]))
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

static VISITOR_DISPATCH_DERIVE_GENERIC_ERROR: &'static str = indoc! {"
    VisitorDispatch cannot be derived for generic types.

    To workaround this limitation, define a newtype wrapper that
    instantiates all of the generic types and derive VisitorDispatch
    on the newtype.

    Additionally, define a blanket Visitor implementation on the
    newtype for all Visitor implementations implemented by the
    generic type and forward the calls to the generic type.

    See the following example:

    struct MyGenericVisitor<T: NodeLogger> {
        logger: T
    }

    impl<'ast, T: NodeLogger> Visitor<'ast, Expr> for MyGenericVisitor<T> {
        fn enter(&mut self, node: Node<'ast, Expr>) -> EnterControlFlow {
            self.logger.log(node);
            ControlFlow::Continue(Navigation::Visit)
        }
    }

    impl<'ast, T> Visitor<'ast, Statement> for MyGenericVisitor<T> {
        fn enter(&mut self, node: Node<'ast, Expr>) -> EnterControlFlow {
            self.logger.log(node);
            ControlFlow::Continue(Navigation::Visit)
        }
    }

    #[derive(VisitorDispatch)]
    struct Wrapper(MyGenericVisitor<OtelLogger>);

    // Implement Visitor<'ast, N> on Wrapper for all Visitor<'ast, N> implemented by the generic type.
    impl<'ast, N> Visitor<'ast, N> for Wrapper where MyGenericVisitor<OtelLogger>: Visitor<'ast, N> {
        fn enter(&mut self, node: Node<'ast, N>) -> EnterControlFlow {
            self.0.enter(node)
        }

        fn exit(&mut self, node: Node<'ast, N>) -> ExitControlFlow {
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
///
/// #[derive(VisitorDispatch)]
/// struct ExprCounter {
///     counter: usize
/// }
///
/// impl<'ast> Visitor<'ast, ast::Expr> for ExprCounter {
///   fn enter(&mut self, node: Node<'ast, ast::Expr>) -> EnterControlFlow {
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

    if generics.params.len() > 0 {
        let err_msg = VISITOR_DISPATCH_DERIVE_GENERIC_ERROR;
        return quote_spanned!( input.span() => compile_error!(#err_msg);).into();
    }

    let mut output = proc_macro2::TokenStream::new();
    let mut entries = proc_macro2::TokenStream::new();

    for node in meta.all_nodes() {
        let chunks = sqltk_codegen_helpers::generics::decompose_generic_type(&node)
            .iter()
            .map(|tp| tp.path.segments.last().unwrap().ident.to_string())
            .collect::<Vec<_>>();
        let joined_chunks = &chunks.join("Of");
        let type_cased = Inflector::to_pascal_case(joined_chunks);
        let ty_ident: TypePath = syn::parse_str(&type_cased).unwrap();

        entries.append_all(quote! {
            type #ty_ident = #krate::dispatch::If<
                {#krate::dispatch::IsVisitor::<Self, #node>::ANSWER},
                #krate::dispatch::Handle<Self, #node>,
                #krate::dispatch::Fallback<Self>
            >;
        });
    }

    // TODO: bring Nope and AssumeNotImplemented into scope
    output.append_all(quote! {
        impl<'ast> #krate::DispatchTable<'ast> for #visitor {
            #entries
        }
    });

    output.into()
}
