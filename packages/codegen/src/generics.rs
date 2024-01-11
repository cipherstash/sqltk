use quote::quote;
use syn::{PathArguments, Type, TypePath};
pub enum ContainerType {
    Box,
    Vec,
    Option,
}

pub(crate) fn is_generic_type(path: &TypePath) -> bool {
    match path.path.segments.last().unwrap().arguments {
        PathArguments::AngleBracketed(_) => true,
        _ => false,
    }
}

pub(crate) fn expect_type_path(ty: &Type) -> &TypePath {
    match ty {
        Type::Path(ref type_path) => type_path,
        _ => panic!("Expected Type::Path(TypePath)"),
    }
}

pub(crate) fn container_type(type_path: &TypePath) -> Option<ContainerType> {
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

/// input: Vec<Vec<Expr>>
/// output:
///     vec![Vec, Vec, Expr] // the Vec is of type Vec<TypePath>
pub(crate) fn decompose_generic_type(ty: &TypePath) -> Vec<TypePath> {
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

pub(crate) fn compose_generic_type(parts: &[TypePath]) -> TypePath {
    if parts.len() > 1 {
        let first = &parts[0];
        let rest = compose_generic_type(&parts[1..]);
        syn::parse2(quote!(#first<#rest>)).unwrap()
    } else {
        parts[0].clone()
    }
}

pub(crate) fn extract_generic_argument(path: &TypePath) -> TypePath {
    match path.path.segments.last().unwrap().arguments {
        PathArguments::AngleBracketed(ref generic) => match generic.args.first().unwrap() {
            syn::GenericArgument::Type(ty) => match ty {
                syn::Type::Path(type_path) => type_path.clone(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use quote::quote;

    #[test]
    fn decompose() {
        let type_path: syn::TypePath = syn::parse2(quote!(Vec<Option<Expr>>)).unwrap();
        let decomposed = super::decompose_generic_type(&type_path);

        assert_eq!(
            decomposed,
            [quote!(Vec), quote!(Option), quote!(Expr)]
                .iter()
                .map(|tp| syn::parse2::<syn::TypePath>(tp.clone()).unwrap())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn compose() {
        let type_path: syn::TypePath = syn::parse2(quote!(Vec<Option<Expr>>)).unwrap();
        let decomposed = super::decompose_generic_type(&type_path);

        assert_eq!(
            type_path,
            super::compose_generic_type(&decomposed),
        );
    }
}
