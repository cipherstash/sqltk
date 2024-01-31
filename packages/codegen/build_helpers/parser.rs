use std::collections::HashMap;
use std::ops::Deref;
use std::{collections::HashSet, hash::Hash};

use proc_macro2::Span;
use sqltk_syn_helpers::generics::{
    compose_generic_type, container_type, decompose_generic_type, expect_type_path,
    is_generic_type, ContainerType,
};
use sqltk_meta::{
    ContainerNode, PrimitiveNode, SqlParserMeta, SqlParserTypeDef, SqlParserTypeDefKind, Syn,
};
use syn::{
    Attribute, Field, Ident, Item, ItemImpl, ItemMod, ItemUse, Meta, Type, TypePath, UseGlob,
    UseGroup, UseName, UsePath, UseRename, UseTree, Visibility,
};

use quote::quote;

/// The fully qualified path of a sqlparser type.  In some cases this will be
/// different to the fully-qualified public export of the type.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct InternalTypePath(TypePath);

/// The fully-qualified public path of a sqlparser type.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct PubTypePath(TypePath);

#[derive(Clone, Debug)]
pub struct SqlParserAstAnalyser {
    internal_types: HashMap<InternalTypePath, SqlParserTypeDef>,
    internal_types_by_basename: HashMap<Ident, InternalTypePath>,
    public_types: HashMap<Ident, PubTypePath>,
}

impl SqlParserAstAnalyser {
    pub fn parse_mod(items: &Vec<syn::Item>) -> SqlParserMeta {
        let mut parser = Self::new();
        let mut path: Vec<Ident> = vec![Ident::new("sqlparser", Span::call_site())];
        parser.parse_mod_recursively(items, &mut path);
        parser.finish()
    }

    fn new() -> Self {
        Self {
            internal_types: Default::default(),
            internal_types_by_basename: Default::default(),
            public_types: Default::default(),
        }
    }

    fn parse_mod_recursively(&mut self, items: &Vec<syn::Item>, path: &mut Vec<Ident>) {
        for item in items {
            match item {
                Item::Struct(item_struct) => {
                    path.push(item_struct.ident.clone());
                    self.insert_internal_type(
                        InternalTypePath(to_path(path)),
                        SqlParserTypeDef {
                            has_visit_impl: false,
                            is_non_exhaustive: is_non_exhaustive(&item_struct.attrs[..]),
                            ty: SqlParserTypeDefKind::Struct(item_struct.clone().into()),
                        },
                    );
                    path.pop();
                }
                Item::Enum(item_enum) => {
                    path.push(item_enum.ident.clone());
                    self.insert_internal_type(
                        InternalTypePath(to_path(path)),
                        SqlParserTypeDef {
                            has_visit_impl: false,
                            is_non_exhaustive: is_non_exhaustive(&item_enum.attrs[..]),
                            ty: SqlParserTypeDefKind::Enum(item_enum.clone().into()),
                        },
                    );
                    path.pop();
                }
                Item::Mod(ItemMod {
                    content: Some((_, items)),
                    ident,
                    ..
                }) => {
                    path.push(ident.clone());
                    self.parse_mod_recursively(items, path);
                    path.pop();
                }
                Item::Impl(ItemImpl {
                    trait_: Some((_, trait_path, _)),
                    self_ty,
                    ..
                }) => {
                    if is_impl_of_sqlparser_visit(trait_path) {
                        let type_path = syn_type_to_path(path, self_ty.deref());
                        if let Some(ty) = self
                            .internal_types
                            .get_mut(&InternalTypePath(type_path.clone()))
                        {
                            ty.has_visit_impl = true;
                        }
                    }
                }
                Item::Use(ItemUse {
                    tree,
                    vis: Visibility::Public(_),
                    ..
                }) => {
                    self.walk_use_tree(tree, &mut path.clone());
                }
                _ => {}
            }
        }
    }

    fn insert_internal_type(&mut self, type_path: InternalTypePath, type_def: SqlParserTypeDef) {
        match self.internal_types.insert(type_path.clone(), type_def) {
            None => {
                let basename = &type_path.0.path.segments.last().unwrap().ident;
                self.internal_types_by_basename
                    .insert(basename.clone(), type_path);
            }
            Some(_) => panic!("Internal type {:?} already recorded", type_path),
        }
    }

    fn walk_use_tree(&mut self, tree: &UseTree, mod_path: &mut Vec<Ident>) {
        match tree {
            UseTree::Path(UsePath { tree, .. }) => {
                self.walk_use_tree(tree, mod_path);
            }
            UseTree::Name(UseName { ident }) => {
                mod_path.push(ident.clone());
                self.public_types.insert(
                    ident.clone(),
                    PubTypePath(syn::parse2(quote!(#(#mod_path)::*)).unwrap()),
                );
                mod_path.pop();
            }
            UseTree::Rename(UseRename { .. }) => {
                unimplemented!("Unexpected UseTree::Rename encountered");
            }
            UseTree::Glob(UseGlob { .. }) => {}
            UseTree::Group(UseGroup { items, .. }) => {
                for tree in items {
                    self.walk_use_tree(tree, mod_path);
                }
            }
        }
    }

    fn convert_generics_to_fully_qualified_form(&self, field: &mut Field) {
        if let Type::Path(ref type_path) = field.ty {
            if is_generic_type(type_path) {
                let mut parts = decompose_generic_type(type_path);
                if let Some(resolved) = self.resolve_fully_qualified_type(parts.last().unwrap()) {
                    let last_idx = parts.len() - 1;
                    parts[last_idx] = resolved;
                    field.ty = Type::Path(compose_generic_type(&parts));
                }
            }
        }
    }

    // Converts a type like `Expr` to `sqlparser::ast::Expr`.  Note that this
    // only works because the last path segment happens to be unique in
    // `sqlparser`.
    fn resolve_fully_qualified_type(&self, type_path: &TypePath) -> Option<TypePath> {
        // 1. check if it's in public types and return it
        // 2. check if it's in the internal types and return it
        // 3. return None
        let base_ident = &type_path.path.segments.last().unwrap().ident;

        match self.public_types.get(base_ident) {
            Some(PubTypePath(found)) => Some(found.clone()),
            None => self
                .internal_types_by_basename
                .get(base_ident)
                .map(|InternalTypePath(found)| found.clone()),
        }
    }

    fn finish(&mut self) -> SqlParserMeta {
        // 1. Forget about all types that are not part of the AST. We use
        // `has_visit_impl` as a proxy for this.
        //
        // Ideally, types not marked as #[derive(Visit)] would not be added to
        // `internal_types` at all, but because we rely on `cargo expand` those
        // attributes are elided. Instead we look for the expanded derivation of
        // `Visit`, but when we parse the type, we have not yet seen the impl.
        // That means we consider all the parsed types as merely candidates
        // until we see a `Visit` impl and we need to cleanup at the end of the
        // parse.

        self.internal_types.retain(|_, ty| ty.has_visit_impl);

        // 2. For every type, go through fields and variants expanding generic
        // types so that they refer to the fully qualified public sqlparser type.
        // e.g. Vec<Expr> should be rewritten as Vec<sqlparser::ast::Expr>

        let mut internal_types = self.internal_types.clone();

        for type_def in internal_types.values_mut() {
            match type_def.ty {
                SqlParserTypeDefKind::Enum(ref mut item_enum) => {
                    for variant in item_enum.variants.iter_mut() {
                        for field in variant.fields.iter_mut() {
                            self.convert_generics_to_fully_qualified_form(field);
                        }
                    }
                }
                SqlParserTypeDefKind::Struct(ref mut item_struct) => {
                    for field in item_struct.fields.iter_mut() {
                        self.convert_generics_to_fully_qualified_form(field);
                    }
                }
            }
        }

        // 3. Capture all the types with public fully qualified type names.
        let mut main_nodes: HashMap<Syn<TypePath>, SqlParserTypeDef> = HashMap::new();
        for (InternalTypePath(type_path), type_def) in internal_types.iter() {
            main_nodes.insert(
                Syn(self
                    .resolve_fully_qualified_type(type_path)
                    .unwrap_or(type_path.clone())),
                type_def.clone(),
            );
        }

        // 4. Flatten the nested generics

        let mut container_nodes: HashSet<ContainerNode> = HashSet::new();

        fn flatten_container_types(
            parts: &[TypePath],
            container_nodes: &mut HashSet<ContainerNode>,
        ) {
            if parts.len() > 1 {
                let container_type_path = compose_generic_type(parts);
                match container_type(&parts[0]) {
                    Some(ContainerType::Box) => {
                        container_nodes.insert(ContainerNode::Box(container_type_path.into()));
                    }
                    Some(ContainerType::Vec) => {
                        container_nodes.insert(ContainerNode::Vec(container_type_path.into()));
                    }
                    Some(ContainerType::Option) => {
                        container_nodes.insert(ContainerNode::Option(container_type_path.into()));
                    }
                    None => {}
                }
                flatten_container_types(&parts[1..], container_nodes);
            }
        }

        for type_def in main_nodes.values_mut() {
            match &type_def.ty {
                SqlParserTypeDefKind::Enum(item_enum) => {
                    for variant in &item_enum.variants {
                        for field in &variant.fields {
                            let type_path = expect_type_path(&field.ty);
                            if is_generic_type(type_path) {
                                let mut parts = decompose_generic_type(type_path);
                                let last_index = parts.len() - 1;
                                parts[last_index] = self
                                    .resolve_fully_qualified_type(&parts[last_index])
                                    .unwrap_or(parts[last_index].clone());
                                flatten_container_types(&parts, &mut container_nodes)
                            }
                        }
                    }
                }
                SqlParserTypeDefKind::Struct(item_struct) => {
                    for field in &item_struct.fields {
                        let type_path = expect_type_path(&field.ty);
                        if is_generic_type(type_path) {
                            let mut parts = decompose_generic_type(type_path);
                            let last_index = parts.len() - 1;
                            parts[last_index] = self
                                .resolve_fully_qualified_type(&parts[last_index])
                                .unwrap_or(parts[last_index].clone());
                            flatten_container_types(&parts, &mut container_nodes)
                        }
                    }
                }
            }
        }

        assert!(!main_nodes.is_empty());
        assert!(!container_nodes.is_empty());

        // This is the return type of sqlparser's parser. It's not discoverable
        // by examining the AST so we need to add it manually.
        container_nodes.insert(ContainerNode::Vec(Syn(syn::parse2(quote!(
            Vec<sqlparser::ast::Statement>
        ))
        .unwrap())));

        let primitive_nodes = PrimitiveNode::all().iter().cloned().collect();

        SqlParserMeta::new(main_nodes, container_nodes, primitive_nodes)
    }
}

fn is_non_exhaustive(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if let Meta::Path(path) = &attr.meta {
            return path
                .segments
                .first()
                .map_or(false, |segment| segment.ident == "non_exhaustive");
        }
        false
    })
}

fn is_impl_of_sqlparser_visit(path: &syn::Path) -> bool {
    let idents = path
        .segments
        .iter()
        .map(|s| s.ident.clone())
        .collect::<Vec<Ident>>();
    *idents.last().unwrap() == "Visit"
}

fn to_path(idents: &[Ident]) -> syn::TypePath {
    let s: String = idents
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join("::");

    syn::parse_str(&s).unwrap()
}

fn syn_type_to_path(path: &[Ident], ty: &Type) -> TypePath {
    if let Type::Path(type_path) = ty {
        let last_segment_ident = &type_path.path.segments.last().unwrap().ident;
        if is_std_lib(last_segment_ident) {
            syn::parse2(quote!(#last_segment_ident)).unwrap()
        } else {
            syn::parse2(quote!(#(#path)::* :: #ty)).unwrap()
        }
    } else {
        unreachable!()
    }
}

fn is_std_lib(ident: &Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        "u8" | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "f32"
            | "f64"
            | "bool"
            | "String"
            | "Vec"
            | "Option"
            | "Box"
    )
}
