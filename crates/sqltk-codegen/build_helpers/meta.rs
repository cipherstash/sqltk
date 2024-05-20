//! This module implements a persistable storage of `sqlparser` AST nodes.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::DerefMut;
use std::{collections::HashSet, ops::Deref};

use quote::{quote, ToTokens};
use syn::TypePath;
use syn::{parse::Parse, ItemEnum, ItemStruct};

#[derive(Clone)]
pub struct SqlParserMeta {
    /// Types for which `sqlparser` derives `Visit`, which are the main structs
    /// and enums plus primitive types & BigDecimal.  The [`TypePath`] is the
    /// *public* fully-qualified type name.
    main_nodes: HashMap<Syn<TypePath>, SqlParserTypeDef>,

    /// Primitives used by sqlparser (most Rust primitives + String + BigDecimal)
    primitive_nodes: HashSet<PrimitiveNode>,
}

/// Provides useful queries over a [`SqlParserMeta`].
pub struct SqlParserMetaQuery {
    meta: SqlParserMeta,
}

impl From<SqlParserMeta> for SqlParserMetaQuery {
    fn from(meta: SqlParserMeta) -> Self {
        Self { meta }
    }
}

impl SqlParserMetaQuery {
    pub fn main_nodes(&self) -> Vec<(TypePath, SqlParserTypeDef)> {
        let mut main_nodes: Vec<(Syn<TypePath>, SqlParserTypeDef)> =
            self.meta.main_nodes.clone().into_iter().collect();

        main_nodes.sort();

        main_nodes
            .into_iter()
            .map(|(Syn(type_path), type_def)| (type_path, type_def))
            .collect()
    }

    pub fn primitive_nodes(&self) -> Vec<PrimitiveNode> {
        let mut primitive_nodes: Vec<PrimitiveNode> =
            self.meta.primitive_nodes.clone().into_iter().collect();
        primitive_nodes.sort();
        primitive_nodes
    }
}

impl SqlParserMeta {
    pub fn new(
        main_nodes: HashMap<Syn<TypePath>, SqlParserTypeDef>,
        container_nodes: HashSet<ContainerNode>,
        primitive_nodes: HashSet<PrimitiveNode>,
    ) -> Self {
        let mut all_nodes: Vec<Syn<TypePath>> = main_nodes
            .keys()
            .cloned()
            .chain(
                container_nodes
                    .iter()
                    .map(|container_node| container_node.type_path().clone()),
            )
            .chain(
                primitive_nodes
                    .iter()
                    .map(|primitive_node| primitive_node.type_path()),
            )
            .collect();

        all_nodes.sort();

        Self {
            main_nodes,
            primitive_nodes,
        }
    }
}

/// A type definition from `sqlparser`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SqlParserTypeDef {
    /// `true` if this type as a Visit impl. After parsing, we'll discard any
    /// that are `false`.
    pub has_visit_impl: bool,
    /// `true` if this type represents a struct or enum marked as
    /// `#[non_exhaustive]`
    pub is_non_exhaustive: bool,
    /// The fields/variants of the struct or enum.
    pub ty: SqlParserTypeDefKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SqlParserTypeDefKind {
    Enum(Syn<ItemEnum>),
    Struct(Syn<ItemStruct>),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum ContainerNode {
    Box(Syn<TypePath>),
    Vec(Syn<TypePath>),
    Option(Syn<TypePath>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum PrimitiveNode {
    BigDecimal,
    Bool,
    Char,
    I16,
    I32,
    I64,
    I8,
    String,
    U16,
    U32,
    U64,
    U8,
}

static ALL_PRIMITIVE_NODES: &[PrimitiveNode] = &[
    PrimitiveNode::BigDecimal,
    PrimitiveNode::Bool,
    PrimitiveNode::Char,
    PrimitiveNode::I16,
    PrimitiveNode::I32,
    PrimitiveNode::I64,
    PrimitiveNode::I8,
    PrimitiveNode::String,
    PrimitiveNode::U16,
    PrimitiveNode::U32,
    PrimitiveNode::U64,
    PrimitiveNode::U8,
];

impl PrimitiveNode {
    pub fn type_path(&self) -> Syn<TypePath> {
        let tokens = match self {
            Self::BigDecimal => quote!(bigdecimal::BigDecimal),
            Self::Bool => quote!(bool),
            Self::Char => quote!(char),
            Self::I16 => quote!(i16),
            Self::I32 => quote!(i32),
            Self::I64 => quote!(i64),
            Self::I8 => quote!(i8),
            Self::String => quote!(String),
            Self::U16 => quote!(u16),
            Self::U32 => quote!(u32),
            Self::U64 => quote!(u64),
            Self::U8 => quote!(u8),
        };
        Syn(syn::parse2(tokens).unwrap())
    }

    pub fn all() -> &'static [PrimitiveNode] {
        ALL_PRIMITIVE_NODES
    }
}

impl From<TypePath> for PrimitiveNode {
    fn from(value: TypePath) -> Self {
        ALL_PRIMITIVE_NODES
            .iter()
            .find(|item| item.type_path() == value)
            .unwrap_or_else(|| panic!("Unexpected primitive: {}", quote!(#value)))
            .clone()
    }
}

impl ContainerNode {
    pub fn type_path(&self) -> &Syn<TypePath> {
        match self {
            Self::Box(type_path) => type_path,
            Self::Vec(type_path) => type_path,
            Self::Option(type_path) => type_path,
        }
    }
}

/// Wraps a type from the `syn` crate in order to support
/// serialization/deserialization with `serde`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Syn<T: ToTokens + Parse + Eq + Hash + Clone>(pub T);

impl<T: ToTokens + Parse + Eq + Hash + Clone> ToTokens for Syn<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl<T: ToTokens + Parse + Eq + Hash + Clone> PartialEq<T> for Syn<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: ToTokens + Parse + Eq + Hash + Clone> Deref for Syn<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ToTokens + Parse + Eq + Hash + Clone> DerefMut for Syn<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ToTokens + Parse + Eq + Hash + Clone> From<T> for Syn<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: ToTokens + Parse + Eq + Hash + Clone> PartialOrd for Syn<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// NOTE: this is not a very efficient Ord implementation, but it is only used at
// build time and only when the `SqlParserMeta` requires regenerating (which
// only happens when bumping a version of the `sqlparser` crate, or when
// building for the first time).
impl<T: ToTokens + Parse + Eq + Hash + Clone> Ord for Syn<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .to_token_stream()
            .to_string()
            .cmp(&other.to_token_stream().to_string())
    }
}
