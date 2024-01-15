//! # Computes reachability of SOURCE NODES
//!
//! The information is used to derive an AST traversal order that is useful for
//! semantic analysis; I.E. nodes that define things that can be referenced
//! from expressions *must* be traversed before any expressions that could
//! possibly reference them.
//!
//! ## Definitions
//!
//! 1. A SOURCE NODE TYPE is one of `ObjectName` or `Table`
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
//! `ObjectName` and `Table` types.
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

use serde::{Deserialize, Serialize};
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    ops::Deref,
};
use syn::{ItemEnum, ItemStruct, TypePath};

use crate::{generics, SqlParserMetaQuery, SqlParserTypeDef, SqlParserTypeDefKind, Syn};

/// Captures the reachability of a source node type from an arbitrary node type.
///
/// `Distance(0)` is a source node, `Distance(1)` is one step removed
/// from a source node etc.
///
/// Ultimately, source node reachability is a bool true/false question but we're
/// keeping track of the distance for debugging reasons.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Distance(pub(crate) u8);

impl Copy for Distance {}

impl Distance {
    /// Create a new `Distance` one step removed from this one.
    fn inc(&self) -> Distance {
        if self.0 == u8::MAX {
            *self
        } else {
            Self(self.0 + 1)
        }
    }

    /// Used to propagate values from child nodes to parent nodes.
    fn propagate(parent: Distance, child: Distance) -> Distance {
        min(parent, child.inc())
    }
}

pub struct Reachability<'a> {
    query: &'a SqlParserMetaQuery,
    results: HashMap<TypePath, Distance>,
    nodes: HashMap<TypePath, SqlParserTypeDef>,
    source_types: Vec<TypePath>,
    expr_ty: TypePath,
}

impl<'a> Reachability<'a> {
    /// Derive source node reachability metrics from our knowledge of
    /// `sqlparser` AST types.
    pub fn derive(query: &'a SqlParserMetaQuery) -> HashMap<TypePath, Distance> {
        Self::new(query).execute()
    }

    fn new(query: &'a SqlParserMetaQuery) -> Self {
        Self {
            query,
            results: HashMap::new(),
            // NOTE: source_types & expr_ty should be a const but
            // syn::parse_quote!  cannot be called in a const context.
            source_types: vec![
                syn::parse_quote!(sqlparser::ast::ObjectName),
                syn::parse_quote!(sqlparser::ast::Table),
            ],
            expr_ty: syn::parse_quote!(sqlparser::ast::Expr),
            nodes: HashMap::new(),
        }
    }

    /// Computes source node reachability for every `sqlparser` node type.
    ///
    /// `None` as a value in the map indicates that the corresponding node does not
    /// have a reachability relationship with a source node.
    fn execute(mut self) -> HashMap<TypePath, Distance> {
        let nodes = self
            .query
            .main_nodes()
            .iter()
            .map(|(ty, def)| (ty.clone(), def.clone()))
            .collect();

        self.nodes = nodes;

        let mut seen: HashSet<TypePath> = HashSet::new();

        for (node, _) in self.nodes.iter() {
            let node_distance = self
                .resolve_ty(node).map(|ty| self.reachability_for_node_type(ty, &mut seen))
                .unwrap_or(Distance(255));
            self.results.insert(node.clone(), node_distance);
        }

        self.results
    }

    fn resolve_ty(&self, ty: &TypePath) -> Option<ResolvedTypePath> {
        // Field types can be generic: we care about the innermost type.
        // E.G. We want the `Expr` from `Vec<Vec<Expr>>`
        let ty = generics::innermost_generic_type(ty);
        self.query
            .lookup_main_node_by_ident(&ty.path.segments.last().unwrap().ident).map(|ty| ResolvedTypePath(ty.clone()))
    }

    fn reachability_for_node_type(
        &self,
        node: ResolvedTypePath,
        seen: &mut HashSet<TypePath>,
    ) -> Distance {
        // At the type-level the AST is an infinitely recursive graph (not a
        // tree like at the value level), so we need to prevent looping forever
        // and bail when we see a type we've already examined.
        if seen.contains(&node.0) {
            return self
                .results
                .get(&node).copied()
                .unwrap_or(Distance(255));
        }

        seen.insert(node.0.clone());

        let mut node_distance = Distance(255);

        if self.source_types.contains(&node) {
            // The base case
            node_distance = Distance(0);
        } else {
            // Examine the child nodes
            match self.nodes.get(&node) {
                Some(SqlParserTypeDef {
                    ty: SqlParserTypeDefKind::Enum(Syn(ItemEnum { variants, .. })),
                    ..
                }) => {
                    let iter = variants.iter();
                    let iter = iter.flat_map(|v| &v.fields);
                    let mut iter = iter.map(|f| &f.ty);
                    node_distance = self.traverse_child_types(&mut iter, node_distance, seen);
                }

                Some(SqlParserTypeDef {
                    ty: SqlParserTypeDefKind::Struct(Syn(ItemStruct { fields, .. })),
                    ..
                }) => {
                    let iter = fields.iter();
                    let mut iter = iter.map(|field| &field.ty);
                    node_distance = self.traverse_child_types(&mut iter, node_distance, seen);
                }

                // Any field types that are not a "main" node (i.e. primitives &
                // containers) can be freely ignored.
                None => {
                    node_distance = Distance(255);
                }
            };
        }

        node_distance
    }

    fn traverse_child_types<'t, I: Iterator<Item = &'t syn::Type>>(
        &self,
        child_node_types: &mut I,
        mut node_distance: Distance,
        seen: &mut HashSet<TypePath>,
    ) -> Distance {
        // Examine all child nodes but do not check reachabilty through Expr nodes.
        for ty in child_node_types
            .map(|ty| syn::parse_quote!(#ty))
            .filter(|ty| !self.is_expr(ty))
        {
            let child_reachable = self
                .resolve_ty(&ty).map(|ty| self.reachability_for_node_type(ty, seen))
                .unwrap_or(Distance(255));
            // Propagate the reachability of the child into the current node.
            node_distance = Distance::propagate(node_distance, child_reachable);
        }

        node_distance
    }

    fn is_expr(&self, ty: &TypePath) -> bool {
        ty == &self.expr_ty
    }
}

struct ResolvedTypePath(TypePath);

// TODO: delete me
impl Deref for ResolvedTypePath {
    type Target = TypePath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
