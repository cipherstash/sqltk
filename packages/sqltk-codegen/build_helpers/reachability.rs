//! # Computes reachability of SOURCE NODES
//!
//! The information is used to derive an AST traversal order that is useful for
//! semantic analysis; I.E. nodes that define things that can be referenced
//! from expressions *must* be traversed before any expressions that could
//! possibly reference them.
//!
//! ## Definitions
//!
//! 1. A SOURCE NODE TYPE is currently just `Table`
//! 2. An arbitrary node type is SOURCE NODE REACHABLE if and only if:
//!    a. is a SOURCE NODE TYPE, or
//!    b. it directly contains a SOURCE NODE field type of or a variant contains
//!       a SOURCE NODE TYPE field type, or
//!    c. it transitively contains a field or variant of a field of node
//!       type that matches condition a or b _and_ the SOURCE NODE TYPE is not
//!       solely reachable via an `Expr` node.
//!
//! ## Explanation
//!
//! In the `sqlparser` AST all data identifiers are ultimately introduced by the
//! `TableFactor` and `Table` types.
//!
//! ## The `Expr` type
//!
//! `Expr` is an AST node type that can contain any expression - including sub
//! selects.
//!
//! Any node type that contains an `Expr` field would therefore be source node
//! reachable but it explicitly stated above that AST nodes are not considered
//! source node reachable if that reachability is soley via an `Expr` node.
//!
//! The reason for this is IDENTIFIER SCOPE. SQL expressions can introduce
//! identifiers but those identifiers can only be referenced within their
//! lexical scope: the surrounding (parent) expressions.

use std::collections::HashMap;
use syn::{Ident, ItemEnum, ItemStruct, Type, TypePath};

use super::meta::{SqlParserMetaQuery, SqlParserTypeDef, SqlParserTypeDefKind, Syn};

pub struct Reachability {
    results: HashMap<Ident, bool>,
    nodes: HashMap<Ident, SqlParserTypeDef>,
    source_types: Vec<Ident>,
    expr_ty: Ident,
}

impl Reachability {
    /// Derive source node reachability metrics from our knowledge of
    /// `sqlparser` AST types.
    pub fn derive(query: &SqlParserMetaQuery) -> HashMap<Ident, bool> {
        Self::new(query).execute()
    }

    fn new(query: &SqlParserMetaQuery) -> Self {
        Self {
            results: HashMap::new(),
            source_types: vec![
                syn::parse_quote!(With),
                syn::parse_quote!(Table),
                syn::parse_quote!(TableFactor),
            ],
            expr_ty: syn::parse_quote!(Expr),
            nodes: query
                .main_nodes()
                .iter()
                .map(|(ty, def)| (ty.path.segments.last().unwrap().ident.clone(), def.clone()))
                .collect(),
        }
    }

    /// Computes source node reachability for every `sqlparser` node type.
    ///
    /// bool(255) as a value in the map indicates that the corresponding
    /// node does not have a reachability relationship with a source node.
    fn execute(&mut self) -> HashMap<Ident, bool> {
        for (node, _) in self.nodes.clone().iter() {
            let source_node_reachable = self.reachability_for_node_type(node);
            self.results.insert(node.clone(), source_node_reachable);
        }

        self.results.clone()
    }

    fn reachability_for_node_type(&mut self, node: &Ident) -> bool {
        // At the type-level the AST is an infinitely recursive graph (not a
        // tree like at the value level), so we need to prevent looping forever
        // and bail when we see a type we've already examined.
        if self.results.contains_key(node) {
            return self.results.get(node).copied().expect("WTF");
        }

        if self.is_expr(node) {
            return false;
        }

        if self.source_types.contains(node) {
            // The base case
            true
        } else {
            // Examine the child nodes
            match self.nodes.get(node) {
                Some(SqlParserTypeDef {
                    ty: SqlParserTypeDefKind::Enum(Syn(ItemEnum { variants, .. })),
                    ..
                }) => {
                    let iter = variants.iter();
                    let iter = iter.flat_map(|v| &v.fields);
                    let field_types = iter.map(|f| f.ty.clone()).collect::<Vec<_>>();
                    self.traverse_child_types(field_types)
                }

                Some(SqlParserTypeDef {
                    ty: SqlParserTypeDefKind::Struct(Syn(ItemStruct { fields, .. })),
                    ..
                }) => {
                    let iter = fields.iter();
                    let field_types = iter.map(|f| f.ty.clone()).collect::<Vec<_>>();
                    self.traverse_child_types(field_types)
                }

                // Terminal nodes can be freely ignored
                None => false,
            }
        }
    }

    fn traverse_child_types(&mut self, child_node_types: Vec<Type>) -> bool {
        // Examine all child nodes but do not check reachabilty through Expr nodes.
        for ty in child_node_types
            .iter()
            .map(|ty| {
                let type_path: TypePath = syn::parse_quote!(#ty);
                type_path.path.segments.last().unwrap().ident.clone()
            })
            .filter(|ty| !self.is_expr(ty))
            .collect::<Vec<_>>()
            .iter()
        {
            let child_source_node_reachable = self.reachability_for_node_type(ty);
            if child_source_node_reachable {
                return true;
            }
        }

        false
    }

    fn is_expr(&self, ty: &Ident) -> bool {
        ty == &self.expr_ty
    }
}
