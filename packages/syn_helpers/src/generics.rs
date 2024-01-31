/// Utitity functions for manipulating generic types during code generation.

use quote::quote;
use syn::{PathArguments, Type, TypePath};

/// An enum of container types used by the `sqlparser` AST.
pub enum ContainerType {
    Box,
    Vec,
    Option,
}

/// Test if a [`TypePath`] represents a generic type.
pub fn is_generic_type(path: &TypePath) -> bool {
    matches!(
        path.path.segments.last().unwrap().arguments,
        PathArguments::AngleBracketed(_)
    )
}

/// Asserts (i.e. panics) if `ty` does not contain a `TypePath`.
pub fn expect_type_path(ty: &Type) -> &TypePath {
    if let Type::Path(ref type_path) = ty {
        type_path
    } else {
        panic!("Expected Type::Path(TypePath)")
    }
}

/// Given a [`TypePath`] returns `Some(ContainerType)` if it represents a
/// `ContainerType`, else returns `None`.
pub fn container_type(type_path: &TypePath) -> Option<ContainerType> {
    match type_path
        .path
        .segments
        .last()
        .unwrap()
        .ident
        .to_string()
        .as_str()
    {
        "Box" => Some(ContainerType::Box),
        "Vec" => Some(ContainerType::Vec),
        "Option" => Some(ContainerType::Option),
        _ => None,
    }
}

/// Unnests a generic type into a `Vec<TypePath>`.
///
/// # Example
/// ```
/// # use sqltk_syn_helpers::generics::*;
/// use syn::{TypePath, parse_quote};
///
/// let fragments: Vec<TypePath> =
///     decompose_generic_type(&parse_quote!(Option<Vec<bool>>));
///
/// let expected: Vec<TypePath> =
///     vec![parse_quote!(Option), parse_quote!(Vec), parse_quote!(bool)];
///
/// assert_eq!(fragments, expected);
///
/// // It handles non-generic types gracefully.
///
/// let fragments: Vec<TypePath> =
///     decompose_generic_type(&parse_quote!(String));
///
/// let expected: Vec<TypePath> =
///     vec![parse_quote!(String)];
///
/// assert_eq!(fragments, expected);
/// ```
pub fn decompose_generic_type(ty: &TypePath) -> Vec<TypePath> {
    let mut output: Vec<TypePath> = Vec::new();
    let mut ty: TypePath = ty.clone();
    while is_generic_type(&ty) {
        let ident = ty.path.segments.last().unwrap().ident.clone();
        output.push(syn::parse2(quote!(#ident)).unwrap());
        ty = extract_generic_argument(&ty);
    }
    output.push(ty.clone());
    output
}

/// Returns the most deeply nested generic type from a `TypePath`.
///
/// # Example
/// ```
/// # use sqltk_syn_helpers::generics::*;
/// use syn::{TypePath, parse_quote};
///
/// let innermost: TypePath =
///     innermost_generic_type(&parse_quote!(Option<Vec<bool>>));
///
/// let expected: TypePath = parse_quote!(bool);
///
/// assert_eq!(innermost, expected);
///
/// // It handles non-generic types gracefully.
///
/// let innermost: TypePath =
///     innermost_generic_type(&parse_quote!(bool));
///
/// let expected: TypePath = parse_quote!(bool);
///
/// assert_eq!(innermost, expected);
/// ```
pub fn innermost_generic_type(ty: &TypePath) -> TypePath {
    let mut ty: TypePath = ty.clone();
    while is_generic_type(&ty) {
        ty = extract_generic_argument(&ty);
    }
    ty
}

/// Composes a `TypePath` for a generic type from its parts. The opposite of
/// [`decompose_generic_type`].
///
/// # Example
/// ```
/// # use sqltk_syn_helpers::generics::*;
/// use syn::{TypePath, parse_quote};
///
/// let composed: TypePath =
///     compose_generic_type(&[parse_quote!(Option), parse_quote!(Vec), parse_quote!(bool)]);
///
/// let expected: TypePath = parse_quote!(Option<Vec<bool>>);
///
/// assert_eq!(composed, expected);
/// ```
pub fn compose_generic_type(parts: &[TypePath]) -> TypePath {
    if parts.len() > 1 {
        let first = &parts[0];
        let rest = compose_generic_type(&parts[1..]);
        syn::parse2(quote!(#first<#rest>)).unwrap()
    } else {
        parts[0].clone()
    }
}

/// Extracts the generic argument from a `TypePath`.
///
/// Assumes there is exactly one generic argument - which is the case for all
/// `sqlparser` AST generic types.
///
/// Panics if there are no generic arguments.
pub fn extract_generic_argument(path: &TypePath) -> TypePath {
    match path.path.segments.last().unwrap().arguments {
        PathArguments::AngleBracketed(ref generic) => match generic.args.first().unwrap() {
            syn::GenericArgument::Type(syn::Type::Path(type_path)) => type_path.clone(),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
