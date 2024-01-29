extern crate self as sqltk;

use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, TokenStreamExt};
use sqltk_meta::{SqlParserMeta, SqlParserMetaQuery};
use syn::{parse_macro_input, DeriveInput, Generics, TypePath};

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

    impl_dispatch_table(&ident, &generics).into()
}

fn impl_dispatch_table(visitor: &Ident, _generics: &Generics) -> proc_macro2::TokenStream {
    let krate = resolve_crate();
    let meta = node_meta();

    // let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

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

        entries.append_all(quote!{
            type #ty_ident = #krate::dispatch::If<{#krate::dispatch::IsVisitor::<Self, #node>::ANSWER}, #krate::dispatch::Handle<Self, #node>, #krate::dispatch::Fallback<Self>>;
        });
    }

    // let mod_ident: Ident = format_ident!("{}_dispatch_table", &visitor);
    // use #krate::dispatch::Nope;

    output.append_all(quote! {

        impl<'ast> #krate::DispatchTable<'ast> for #visitor {
            #entries
        }
    });

    output
}
