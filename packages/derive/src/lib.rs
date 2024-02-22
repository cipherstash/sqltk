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
    DeriveInput, GenericParam, Token, TypePath,
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
    VisitorDispatch can only be derived for types with exactly zero or one
    generic lifetime parameter. More than one lifetime parameter, or any number
    of generic type parameters or generic const parameters are not permitted.

    The sole lifetime parameter, if present, represents the lifetime of AST
    nodes and is required if your type needs to keep references to AST nodes as
    part of its internal state but is otherwise not required.

    To workaround this limitation, define a newtype wrapper that
    instantiates all of the generic types and derive VisitorDispatch
    on the wrapper.

    Additionally, define a blanket Visitor implementation on the
    wrapper for all Visitor implementations implemented by the
    generic type and forward the methods to the generic type.

    See the following example:

    struct MyGenericVisitor<'ast, T: NodeLogger<'ast>> {
        logger: T
    }

    impl<'ast, T: NodeLogger> Visitor<'ast, Expr> for MyGenericVisitor<'ast, T> {
        fn enter(&mut self, node: &'ast Expr) -> EnterControlFlow {
            self.logger.log(node);
            ControlFlow::Continue(Navigation::Visit)
        }
    }

    impl<'ast, T> Visitor<'ast, Statement> for MyGenericVisitor<'ast, T> {
        fn enter(&mut self, node: &'ast Expr) -> EnterControlFlow {
            self.logger.log(node);
            ControlFlow::Continue(Navigation::Visit)
        }
    }

    #[derive(VisitorDispatch)]
    struct Wrapper<'ast>(MyGenericVisitor<'ast, OtelLogger>);

    // Implement Visitor<'ast, N> on Wrapper<'ast> for all Visitor<'ast, N> implemented by the generic type.
    impl<'ast, N> Visitor<'ast, N> for Wrapper<'ast> where MyGenericVisitor<'ast, OtelLogger>: Visitor<'ast, N> {
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
/// struct ExprCounter<'ast> {
///     counter: usize,
///     _ast: PhantomData<&'ast ()>,
/// }
///
/// impl<'ast> Visitor<'ast, ast::Expr> for ExprCounter<'ast> {
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

    if count_lifetime_params > 1  {
        let err_msg = VISITOR_DISPATCH_DERIVE_GENERIC_ERROR;
        return quote_spanned!( input.span() => compile_error!(#err_msg);).into();
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut output = proc_macro2::TokenStream::new();
    let mut entries = proc_macro2::TokenStream::new();

    let visitor_static = if count_lifetime_params == 1 {
        quote!(#visitor<'static>)
    } else {
        quote!(#visitor)
    };

    let visitor_nonstatic = if count_lifetime_params == 1 {
        quote!(#visitor<'a>)
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
            type #ty_ident<'a> = #krate::dispatch::If<
                {#krate::dispatch::IsVisitor::<#visitor_static, #node>::ANSWER},
                #krate::dispatch::Handle<#visitor_nonstatic, #node>,
                #krate::dispatch::Fallback<#visitor_nonstatic>
            >;
        });
    }

    // TODO: bring Nope and AssumeNotImplemented into scope
    output.append_all(quote! {
        impl #impl_generics #krate::DispatchTable for #visitor #ty_generics #where_clause {
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
        dispatch_enter
            .append_all(quote!(#krate::VisitorDispatch::enter(&mut self.0.#field, node.clone())?;));
    }

    stages.reverse();

    for (idx, stage) in &stages {
        let field = format_ident!("stage{}", idx);
        stages_init_exit.append_all(quote!(#field: #stage::init_exit(&mut scope)?,));
        // dispatch_exit.append_all(quote!(self.0.#field.exit(node.clone())?;));
        dispatch_exit
            .append_all(quote!(#krate::VisitorDispatch::exit(&mut self.0.#field, node.clone())?;));
    }

    quote! {
        use #krate::Pipeline;

        struct #pipeline_ty<'ast> {
            scope: RootScope,
            #stage_fields
            // If none of the stage types make use of the 'ast lifetime we'd get
            // a compilation error so we need the phantom reference.
            _ast: PhantomData<&'ast ()>,
        }

        #[automatically_derived]
        impl<'ast> #krate::Pipeline<'ast> for #pipeline_ty<'ast> where #where_clause {
            fn new(scope: #krate::RootScope) -> Result<Self, #krate::PipelineInitError> {
                let mut scope = scope;
                #stages_init_enter
                Ok(
                    Self {
                        #stages_init_exit
                        scope,
                        _ast: Default::default(),
                    }
                )
            }

            fn execute<N: Visitable<'ast>>(self, node: &'ast N) -> Result<RootScope, RootScope> {
                struct Dispatcher<'a>(#pipeline_ty<'a>);

                impl<'ast> #krate::VisitorDispatch<'ast> for Dispatcher<'ast> {
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
    }
    .into()
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
