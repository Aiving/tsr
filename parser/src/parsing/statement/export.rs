use super::{
    class::parse_class_declaration, enumeration::parse_enum_declaration,
    expression::parse_expression, function::parse_function_declaration,
    interface::parse_interface_declaration, type_alias::parse_type_alias_declaration,
    variable::parse_variable_statement,
};

use crate::{
    ast::{
        ExportDeclaration, ExportDefaultElement, ExportListElement, ExportSingleElement,
        ExportSpecifier,
    },
    parsing::{parse_from_clause, parse_ident},
    tags::{
        as_tag, brace_close_tag, brace_open_tag, comma_tag, export_tag, positioned, semi_tag,
        star_tag,
    },
};

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_export_default_element(
    input: Tokens,
) -> TokenResult<Positioned<ExportDefaultElement>> {
    positioned(alt((
        map(
            parse_function_declaration,
            ExportDefaultElement::FunctionDeclaration,
        ),
        map(
            parse_class_declaration,
            ExportDefaultElement::ClassDeclaration,
        ),
        map(parse_expression, ExportDefaultElement::Expression),
        map(parse_ident, ExportDefaultElement::IdentifierReference),
    )))(input)
}

pub fn parse_export_single_element(input: Tokens) -> TokenResult<Positioned<ExportSingleElement>> {
    positioned(alt((
        map(
            parse_variable_statement,
            ExportSingleElement::VariableStatement,
        ),
        map(
            parse_function_declaration,
            ExportSingleElement::FunctionDeclaration,
        ),
        map(
            parse_class_declaration,
            ExportSingleElement::ClassDeclaration,
        ),
        map(
            parse_interface_declaration,
            ExportSingleElement::InterfaceDeclaration,
        ),
        map(
            parse_type_alias_declaration,
            ExportSingleElement::TypeAliasDeclaration,
        ),
        map(parse_enum_declaration, ExportSingleElement::EnumDeclaration),
    )))(input)
}

pub fn parse_export_list_element(input: Tokens) -> TokenResult<Positioned<ExportListElement>> {
    positioned(alt((
        map(
            preceded(star_tag, parse_from_clause),
            ExportListElement::Namespace,
        ),
        map(
            pair(parse_export_clause, parse_from_clause),
            |(specifiers, module)| ExportListElement::NamespaceExports(specifiers, module),
        ),
        map(parse_export_clause, ExportListElement::NamedExports),
    )))(input)
}

pub fn parse_export_clause(
    input: Tokens,
) -> TokenResult<Positioned<Vec<Positioned<ExportSpecifier>>>> {
    positioned(delimited(
        brace_open_tag,
        separated_list0(
            comma_tag,
            map(
                pair(parse_ident, opt(tuple((as_tag, parse_ident)))),
                |(ident, alias)| {
                    alias
                        .as_ref()
                        .map(|(_, alias)| ident.between(alias))
                        .unwrap_or(ident.span)
                        .wrap(ExportSpecifier {
                            property_name: alias.as_ref().map(|_| ident.clone()),
                            name: alias
                                .map(|(start, alias)| start.between(&alias).wrap(alias.value))
                                .unwrap_or(ident),
                        })
                },
            ),
        ),
        brace_close_tag,
    ))(input)
}

pub fn parse_export_declaration(input: Tokens) -> TokenResult<Positioned<ExportDeclaration>> {
    positioned(delimited(
        export_tag,
        alt((
            map(parse_export_default_element, ExportDeclaration::Default),
            map(parse_export_single_element, ExportDeclaration::Single),
            map(parse_export_list_element, ExportDeclaration::List),
        )),
        opt(semi_tag),
    ))(input)
}
