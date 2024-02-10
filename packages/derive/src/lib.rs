use indoc::indoc;
use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote, quote_spanned, TokenStreamExt};
use sqltk_codegen::NODE_LIST;
use sqltk_meta::{SqlParserMeta, SqlParserMetaQuery};
use sqltk_syn_helpers::generics::decompose_generic_type;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    DeriveInput, Token, TypePath,
};

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
        fn enter(&mut self, node: &'ast Expr) -> EnterControlFlow {
            self.logger.log(node);
            ControlFlow::Continue(Navigation::Visit)
        }
    }

    impl<'ast, T> Visitor<'ast, Statement> for MyGenericVisitor<T> {
        fn enter(&mut self, node: &'ast Expr) -> EnterControlFlow {
            self.logger.log(node);
            ControlFlow::Continue(Navigation::Visit)
        }
    }

    #[derive(VisitorDispatch)]
    struct Wrapper(MyGenericVisitor<OtelLogger>);

    // Implement Visitor<'ast, N> on Wrapper for all Visitor<'ast, N> implemented by the generic type.
    impl<'ast, N> Visitor<'ast, N> for Wrapper where MyGenericVisitor<OtelLogger>: Visitor<'ast, N> {
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
///
/// #[derive(VisitorDispatch)]
/// struct ExprCounter {
///     counter: usize
/// }
///
/// impl<'ast> Visitor<'ast, ast::Expr> for ExprCounter {
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

    if generics.params.len() > 0 {
        let err_msg = VISITOR_DISPATCH_DERIVE_GENERIC_ERROR;
        return quote_spanned!( input.span() => compile_error!(#err_msg);).into();
    }

    let mut output = proc_macro2::TokenStream::new();
    let mut entries = proc_macro2::TokenStream::new();

    for node in meta.all_nodes() {
        let chunks = decompose_generic_type(&node)
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

/// Creates a type that implements [`sqltk_core::Pipeline`].
///
/// Example that creates a type called `MyPipeline` that implements
/// [`sqltk_core::pipeline::Pipeline`].
///
/// ```rust,ignore
/// pipeline!(MyPipeline, BalancedExprsCheck => ExprCounter);
///
/// let pipeline = MyPipeline::new(RootScope::new());
///
/// let ast = ...;
///
/// match pipeline.execute(&ast) {
///     Some(scope) => /* do something with the scope */,
///     Err(err) => /* something went wrong */
/// }
/// ```
#[proc_macro]
pub fn pipeline(input: TokenStream) -> TokenStream {
    let input: PipelineInput = parse_macro_input!(input);

    let krate = resolve_crate();
    let pipeline_ty = &input.ident;

    let mut stage_fields = proc_macro2::TokenStream::new();
    let mut stages_init_enter = proc_macro2::TokenStream::new();
    let mut stages_init_exit = proc_macro2::TokenStream::new();
    let mut where_clause = proc_macro2::TokenStream::new();
    let mut dispatch_enter = proc_macro2::TokenStream::new();
    let mut dispatch_exit = proc_macro2::TokenStream::new();

    let mut stages: Vec<(usize, &TypePath)> = input.stages.stages.iter().enumerate().collect();

    for (idx, stage) in &stages {
        let field = format_ident!("stage{}", idx);
        stage_fields.append_all(quote!(#field: #stage,));
        stages_init_enter.append_all(quote!(#stage::init_enter(&mut scope)?;));
        where_clause.append_all(quote!(#stage: VisitorDispatch<'ast>,));

        // dispatch_enter.append_all(quote!(self.0.#field.enter(node.clone())?;));
        dispatch_enter.append_all(quote!(#krate::VisitorDispatch::enter(&mut self.0.#field, node.clone())?;));
    }

    stages.reverse();

    for (idx, stage) in &stages {
        let field = format_ident!("stage{}", idx);
        stages_init_exit
            .append_all(quote!(#field: #stage::init_exit(&mut scope)?,));
        // dispatch_exit.append_all(quote!(self.0.#field.exit(node.clone())?;));
        dispatch_exit.append_all(quote!(#krate::VisitorDispatch::exit(&mut self.0.#field, node.clone())?;));
    }

    quote! {
        use #krate::Pipeline;

        struct #pipeline_ty {
            scope: RootScope,
            #stage_fields
        }

        #[automatically_derived]
        impl<'ast> #krate::Pipeline<'ast> for #pipeline_ty where #where_clause {
            fn new(scope: #krate::RootScope) -> Result<Self, #krate::PipelineInitError> {
                let mut scope = scope;
                #stages_init_enter
                Ok(
                    Self {
                        #stages_init_exit
                        scope,
                    }
                )
            }

            fn execute<N: Visitable<'ast>>(self, node: &'ast N) -> Result<RootScope, RootScope> {
                struct Dispatcher(#pipeline_ty);

                impl<'ast> #krate::VisitorDispatch<'ast> for Dispatcher {
                    fn enter(&mut self, node: #krate::SqlNode<'ast>) -> #krate::EnterControlFlow {
                        #dispatch_enter
                        std::ops::ControlFlow::Continue(#krate::Navigation::Visit)
                    }

                    fn exit(&mut self, node: #krate::SqlNode<'ast>) -> #krate::ExitControlFlow {
                        #dispatch_exit
                        std::ops::ControlFlow::Continue(())
                    }
                }

                let mut dispatcher = Dispatcher(self);

                match node.accept(&mut dispatcher) {
                    ControlFlow::Continue(_) => Ok(dispatcher.0.scope),
                    ControlFlow::Break(_) => Err(dispatcher.0.scope),
                }
            }
        }
    }.into()
}

struct PipelineInput {
    ident: Ident,
    _comma: Token![,],
    stages: Stages,
}

impl Parse for PipelineInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(PipelineInput {
            ident: input.parse()?,
            _comma: input.parse()?,
            stages: input.parse()?,
        })
    }
}

struct Stages {
    stages: Punctuated<TypePath, Token![=>]>,
}

impl Parse for Stages {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut stages = Punctuated::new();
        while !input.is_empty() {
            let ty: TypePath = input.parse()?;
            stages.push(ty);

            if input.peek(Token![=>]) {
                input.parse::<Token![=>]>()?;
            } else {
                break;
            }
        }

        Ok(Stages { stages })
    }
}

#[cfg(test)]
mod test {
    use crate::Stages;

    #[test]
    fn test_parse_stages() {
        let stages: Stages = syn::parse_quote!( Foo => Bar);
        eprintln!("Stages: {:#?}", &stages.stages);
    }
}