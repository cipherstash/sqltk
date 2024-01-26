//! This module implements a persistable storage of `sqlparser` AST nodes.

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Formatter;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::{collections::HashSet, ops::Deref};

use proc_macro2::Span;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use syn::{Ident, TypePath};
use syn::{parse::Parse, ItemEnum, ItemStruct};
use quote::{quote, ToTokens};

/// Serializable/deserializable data structure that captures AST node metadata
/// extracted from `sqlparser`.
#[derive(Clone, Serialize, Deserialize)]
pub struct SqlParserMeta {
    /// Types for which `sqlparser` derives `Visit`, which are the main structs
    /// and enums plus primitive types & BigDecimal.  The [`TypePath`] is the
    /// *public* fully-qualified type name.
    #[serde(serialize_with = "ordered_map")]
    main_nodes: HashMap<Syn<TypePath>, SqlParserTypeDef>,

    /// Nodes that are wrapped in Vec, Option or Box.
    #[serde(serialize_with = "ordered_list")]
    container_nodes: HashSet<ContainerNode>,

    /// Primitives used by sqlparser (most Rust primitives + String + BigDecimal)
    #[serde(serialize_with = "ordered_list")]
    primitive_nodes: HashSet<PrimitiveNode>,

    /// A map to support looking up the synthetic ID of a Node.
    #[serde(serialize_with = "ordered_map")]
    node_type_to_id: HashMap<Syn<TypePath>, usize>,
}

/// Provides useful queries over a [`SqlParserMeta`].
pub struct SqlParserMetaQuery {
    meta: SqlParserMeta,
    /// All `sqlparser` AST types are unique in the last component of their type
    /// name. Use this to map from an `Ident` to a `TypePath`.
    main_nodes_by_ident: HashMap<Ident, TypePath>,
}

impl From<SqlParserMeta> for SqlParserMetaQuery {
    fn from(meta: SqlParserMeta) -> Self {
        let mut main_nodes_by_ident: HashMap<Ident, TypePath> = HashMap::new();
        for type_path in meta.main_nodes.keys() {
            main_nodes_by_ident.insert(
                type_path.0.path.segments.last().unwrap().ident.clone(),
                type_path.0.clone(),
            );
        }
        Self {
            meta,
            main_nodes_by_ident,
        }
    }
}

impl SqlParserMetaQuery {
    pub fn lookup_main_node_by_ident(&self, ident: &Ident) -> Option<&TypePath> {
        self.main_nodes_by_ident.get(ident)
    }

    pub fn lookup_node_id(&self, type_path: &TypePath) -> Option<usize> {
        self.meta
            .node_type_to_id
            .get(&Syn(type_path.clone()))
            .cloned()
    }

    pub fn all_nodes(&self) -> Vec<TypePath> {
        let mut all_nodes: Vec<Syn<TypePath>> = Vec::new();

        all_nodes.extend(
            self.meta
                .main_nodes
                .keys()
                .cloned()
                .chain(
                    self.meta
                        .container_nodes
                        .iter()
                        .map(|c| c.type_path().clone()),
                )
                .chain(self.meta.primitive_nodes.iter().map(|c| c.type_path())),
        );

        all_nodes.sort();

        let all_nodes: Vec<TypePath> = all_nodes
            .into_iter()
            .map(|Syn(type_path)| type_path)
            .collect();

        all_nodes
    }

    pub fn all_nodes_with_node_id(&self) -> Vec<(TypePath, usize)> {
        let mut nodes: Vec<_> = self
            .all_nodes()
            .iter()
            .map(|type_path| (type_path.clone(), self.lookup_node_id(type_path).unwrap()))
            .collect();

        nodes.sort_by(|(_, id1), (_, id2)| id1.cmp(id2));

        nodes
    }

    pub fn main_nodes(&self) -> Vec<(TypePath, SqlParserTypeDef)> {
        let mut main_nodes: Vec<(Syn<TypePath>, SqlParserTypeDef)> =
            self.meta.main_nodes.clone().into_iter().collect();

        main_nodes.sort();

        main_nodes
            .into_iter()
            .map(|(Syn(type_path), type_def)| (type_path, type_def))
            .collect()
    }

    pub fn container_nodes(&self) -> Vec<ContainerNode> {
        let mut container_nodes: Vec<ContainerNode> =
            self.meta.container_nodes.clone().into_iter().collect();
        container_nodes.sort();
        container_nodes
    }

    pub fn primitive_nodes(&self) -> Vec<PrimitiveNode> {
        let mut primitive_nodes: Vec<PrimitiveNode> =
            self.meta.primitive_nodes.clone().into_iter().collect();
        primitive_nodes.sort();
        primitive_nodes
    }
}

/// For use with serde's [serialize_with] attribute
fn ordered_map<S, K: Ord + Serialize, V: Serialize>(
    value: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

/// For use with serde's [serialize_with] attribute
fn ordered_list<S, K: Ord + Serialize>(value: &HashSet<K>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ordered: Vec<_> = value.iter().collect();
    ordered.sort();
    ordered.serialize(serializer)
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

        let node_type_to_id: HashMap<Syn<TypePath>, usize> = all_nodes
            .into_iter()
            .enumerate()
            .map(|(idx, type_path)| (type_path, idx))
            .collect();

        Self {
            main_nodes,
            container_nodes,
            primitive_nodes,
            node_type_to_id,
        }
    }
}

/// A type definition from `sqlparser`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum SqlParserTypeDefKind {
    Enum(Syn<ItemEnum>),
    Struct(Syn<ItemStruct>),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub enum ContainerNode {
    Box(Syn<TypePath>),
    Vec(Syn<TypePath>),
    Option(Syn<TypePath>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
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
    pub fn variant_ident(&self) -> Syn<Ident> {
        let name = match self {
            PrimitiveNode::BigDecimal => "BigDecimal",
            PrimitiveNode::Bool => "Bool",
            PrimitiveNode::Char => "Char",
            PrimitiveNode::I16 => "I16",
            PrimitiveNode::I32 => "I32",
            PrimitiveNode::I64 => "I64",
            PrimitiveNode::I8 => "I8",
            PrimitiveNode::String => "String",
            PrimitiveNode::U16 => "U16",
            PrimitiveNode::U32 => "U32",
            PrimitiveNode::U64 => "U64",
            PrimitiveNode::U8 => "U8",
        };

        Syn(Ident::new(name, Span::call_site()))
    }

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

impl<T: ToTokens + Parse + Eq + Hash + Clone> Serialize for Syn<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.to_token_stream().to_string().as_str())
    }
}

impl<'de, T: ToTokens + Parse + Eq + Hash + Clone> Deserialize<'de> for Syn<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SynTypeVisitor::<T>(PhantomData))
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

struct SynTypeVisitor<T: ToTokens + Eq + Hash>(PhantomData<T>);

impl<'de, T: ToTokens + Parse + Eq + Hash + Clone> serde::de::Visitor<'de> for SynTypeVisitor<T> {
    type Value = Syn<T>;

    fn expecting(&self, _formatter: &mut Formatter) -> std::fmt::Result {
        Ok(())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let result: Result<T, _> = syn::parse_str(v);
        match result {
            Ok(value) => Ok(Syn(value)),
            Err(_) => Err(E::custom("failed to parse syn type")),
        }
    }
}
