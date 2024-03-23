use syn::{Attribute /* , Expr, ExprLit, Lit */, MetaList /* , MetaNameValue, Path */};

pub(crate) fn get_attribute<T: Into<String>>(
    attributes: &[Attribute],
    name: T,
) -> Option<Attribute> {
    let name: String = name.into();

    let attribute = attributes
        .iter()
        .find(|attribute| attribute.path().segments[0].ident == name)?;

    Some(attribute.clone())
}

pub(crate) fn get_list<T: Into<String>>(
    attributes: &[Attribute],
    name: T,
) -> Option<(Attribute, MetaList)> {
    let attribute = get_attribute(attributes, name).map(|attribute| {
        (
            attribute.clone(),
            attribute.meta.require_list().unwrap().clone(),
        )
    });

    attribute
}

/*
pub(crate) fn get_name_value<T: Into<String>>(
    attributes: &[Attribute],
    name: T,
) -> Option<(Attribute, MetaNameValue)> {
    let attribute = get_attribute(attributes, name).map(|attribute| {
        (
            attribute.clone(),
            attribute.meta.require_name_value().unwrap().clone(),
        )
    });

    attribute
}

pub(crate) fn get_string_value(name_value: MetaNameValue) -> Option<String> {
    let lit = match name_value.value {
        Expr::Lit(ExprLit { attrs: _, lit }) => lit.clone(),
        _ => return None,
    };
    let value = match lit {
        Lit::Str(lit_str) => lit_str.value(),
        _ => return None,
    };

    Some(value)
}

pub(crate) fn get_path<T: Into<String>>(
    attributes: &[Attribute],
    name: T,
) -> Option<(Attribute, Path)> {
    let attribute = get_attribute(attributes, name)
        .map(|attribute| (attribute.clone(), attribute.meta.path().clone()));

    attribute
}
 */
