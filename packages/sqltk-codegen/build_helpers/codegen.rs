use super::apply_transform_trait_impls::ApplyTransformImpl;
use super::meta::{AstNode, SqlParserMetaQuery};
use super::reachability::Reachability;
use super::{sqlparser_node_extractor, visitable_trait_impls::VisitableImpl};
use proc_macro2::TokenStream;

use quote::{quote, ToTokens, TokenStreamExt};

use std::{collections::HashSet, fs::File, io::Write, path::PathBuf};

pub struct Codegen {
    meta: SqlParserMetaQuery,
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            meta: SqlParserMetaQuery::from(sqlparser_node_extractor::extract(vec![])),
        }
    }

    pub fn generate_apply_transform_impls(&self, dest_file: &PathBuf) {
        let mut generated_code = TokenStream::new();

        let main_nodes = self.meta.main_nodes();
        let terminal_nodes = self.meta.terminal_nodes();

        let transformable_impls_for_main_nodes = main_nodes.iter().map(|(type_path, type_def)| {
            ApplyTransformImpl::new(
                type_path.clone(),
                AstNode::SqlParserTypeDef(type_def.clone()),
            )
        });

        let transformable_impls_for_terminal_nodes = terminal_nodes.iter().map(|terminal_node| {
            ApplyTransformImpl::new(
                terminal_node.type_path().0.clone(),
                AstNode::TerminalNode(terminal_node.clone()),
            )
        });

        generated_code.append_all(quote! {
            #(#transformable_impls_for_main_nodes)*
            #(#transformable_impls_for_terminal_nodes)*
        });

        let mut file = File::create(dest_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

        let parsed = syn::parse_file(&generated_code.to_string());
        let formatted = parsed.map(|parsed| prettyplease::unparse(&parsed));

        match formatted {
            Ok(formatted) => file
                .write_all(formatted.as_bytes())
                .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display())),
            Err(_) => file
                .write_all(generated_code.to_string().as_bytes())
                .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display())),
        }
    }

    pub fn generate_visitable_impls(&self, dest_file: &PathBuf, reachability_debug_file: &PathBuf) {
        let reachability = Reachability::derive(&self.meta);

        let mut file = File::create(reachability_debug_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &reachability_debug_file.display()));

        for (ty, source_node_reachable) in &reachability {
            let _ = file
                .write(format!("{} {}\n", source_node_reachable, ty.to_token_stream()).as_bytes());
        }

        let mut generated_code = TokenStream::new();

        let main_nodes = self.meta.main_nodes();
        let terminal_nodes = self
            .meta
            .terminal_nodes()
            .iter()
            .map(|pn| pn.type_path().path.segments.last().unwrap().ident.clone())
            .collect::<HashSet<_>>();

        let visitable_impls_for_main_nodes = main_nodes.into_iter().map(|(type_path, type_def)| {
            VisitableImpl::new(
                type_path,
                AstNode::SqlParserTypeDef(type_def),
                reachability.clone(),
                terminal_nodes.clone(),
            )
        });

        let visitable_impls_for_terminal_nodes =
            self.meta.terminal_nodes().into_iter().map(|terminal_node| {
                VisitableImpl::new(
                    terminal_node.type_path().0,
                    AstNode::TerminalNode(terminal_node),
                    reachability.clone(),
                    terminal_nodes.clone(),
                )
            });

        generated_code.append_all(quote! {
            #(#visitable_impls_for_main_nodes)*
            #(#visitable_impls_for_terminal_nodes)*
        });

        let mut file = File::create(dest_file)
            .unwrap_or_else(|_| panic!("Could not open {}", &dest_file.display()));

        let formatted = prettyplease::unparse(
            &syn::parse_file(&generated_code.to_string())
                .expect("BUG! Generated Rust code could not be parsed"),
        );

        file.write_all(formatted.as_bytes())
            .unwrap_or_else(|_| panic!("Could not write to {}", &dest_file.display()));
    }
}
