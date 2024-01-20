extern crate self as sqltk;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, TokenStreamExt};
use sqltk_codegen::{SqlParserMeta, SqlParserMetaQuery};
use static_include_bytes::static_include_bytes;
use syn::{parse_macro_input, DeriveInput, Generics, TypePath};

static_include_bytes!(
    #[no_mangle]
    RAW_NODE_META = concat!(env!("OUT_DIR"), "/generated_node_list.json")
);

fn node_meta() -> SqlParserMetaQuery {
    let meta: SqlParserMeta = serde_json::from_str(
        String::from_utf8(Vec::from(&RAW_NODE_META[..]))
            .unwrap()
            .as_str(),
    )
    .unwrap();
    meta.into()
}

fn resolve_crate() -> proc_macro2::TokenStream {
    let found_crate = crate_name("sqltk").expect("sqltk is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident )
        }
    }
}

#[proc_macro_derive(VisitorDispatch)]
pub fn derive_visitor_dispatch(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);

    impl_node_support(&ident, &generics).into()
}

fn impl_node_support(visitor: &Ident, generics: &Generics) -> proc_macro2::TokenStream {
    let krate = resolve_crate();
    let meta = node_meta();

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut output = proc_macro2::TokenStream::new();

    for node in meta.all_nodes() {
        let check = check_visits(visitor, generics, &node);

        output.append_all(quote! {
            impl #impl_generics #krate::NodeSupport<#node> for #visitor #type_generics #where_clause {
                type Supported = #krate::Condition<{#check}>;
            }
        });
    }

    output
}

fn check_visits(
    visitor: &Ident,
    _generics: &Generics,
    node: &TypePath,
) -> proc_macro2::TokenStream {
    let krate = resolve_crate();
    // let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        trait AssumeNotImplemented { const ANSWER: bool = false; }
        impl<V> AssumeNotImplemented for V {}

        struct Visits<V, N>(::core::marker::PhantomData<(V, N)>);

        #[allow(dead_code)]
        impl<V, N> Visits<V, N>
        where
            N: 'static + #krate::AstNode<'static>,
            V: #krate::Visitor<'static, N>,
        {
            const ANSWER: bool = true;
        }

        <Visits::<#visitor, #node>>::ANSWER
    }
}
