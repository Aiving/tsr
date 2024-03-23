pub mod class;
pub mod enumeration;
pub mod export;
pub mod expression;
pub mod function;
pub mod if_else;
pub mod import;
pub mod interface;
pub mod returning;
pub mod type_alias;
pub mod variable;

use super::{
    parse_ident, parse_literal,
    signatures::{
        parse_call_signature, parse_construct_signature, parse_index_signature,
        parse_method_signature, parse_property_signature,
    },
    types::parse_type,
};

use crate::{
    ast::{ArrowParameter, Literal, Parameter, PropertyName, Statement, TypeMember, TypeParameter},
    tags::{
        bracket_close_tag, bracket_open_tag, colon_tag, eq_tag, extends_tag, positioned,
        private_tag, protected_tag, public_tag, question_tag, semi_tag,
    },
};

use nom::{
    branch::alt,
    combinator::{map, opt, value},
    sequence::{delimited, preceded, terminated, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    token::Modifier,
    tokens::Tokens,
};

pub fn parse_property_name(input: Tokens) -> TokenResult<Positioned<PropertyName>> {
    positioned(alt((
        map(parse_ident, |ident| {
            PropertyName::LiteralPropertyName(
                ident
                    .span
                    .wrap(Literal::String(ident.span.wrap(ident.value.0))),
            )
        }),
        map(parse_literal, PropertyName::LiteralPropertyName),
        map(
            delimited(
                bracket_open_tag,
                expression::parse_expression,
                bracket_close_tag,
            ),
            PropertyName::ComputedPropertyName,
        ),
    )))(input)
}

pub fn parse_type_member(input: Tokens) -> TokenResult<Positioned<TypeMember>> {
    positioned(alt((
        map(parse_property_signature, TypeMember::PropertySignature),
        map(parse_call_signature, TypeMember::CallSignature),
        map(parse_construct_signature, TypeMember::ConstructSignature),
        map(parse_index_signature, TypeMember::IndexSignature),
        map(parse_method_signature, TypeMember::MethodSignature),
    )))(input)
}

pub fn parse_type_parameter(input: Tokens) -> TokenResult<Positioned<TypeParameter>> {
    positioned(map(
        tuple((
            parse_ident,
            opt(preceded(extends_tag, parse_type)),
            opt(preceded(eq_tag, parse_type)),
        )),
        |(name, constraint, default)| TypeParameter {
            name,
            constraint,
            default,
        },
    ))(input)
}

pub fn parse_arrow_parameter(input: Tokens) -> TokenResult<Positioned<ArrowParameter>> {
    positioned(map(
        tuple((
            parse_ident,
            positioned(opt(question_tag)),
            opt(preceded(colon_tag, parse_type)),
            opt(preceded(eq_tag, expression::parse_expression)),
        )),
        |(name, nullable, ty, default)| ArrowParameter {
            name,
            nullable: nullable.wrap(nullable.value.is_some()),
            ty,
            default,
        },
    ))(input)
}

pub fn parse_parameter(input: Tokens) -> TokenResult<Positioned<Parameter>> {
    positioned(map(
        tuple((
            parse_ident,
            positioned(opt(question_tag)),
            preceded(colon_tag, parse_type),
            opt(preceded(eq_tag, expression::parse_expression)),
        )),
        |(name, nullable, ty, default)| Parameter {
            name,
            nullable: nullable.wrap(nullable.value.is_some()),
            ty,
            default,
        },
    ))(input)
}

pub fn parse_access_modifier(input: Tokens) -> TokenResult<Positioned<Modifier>> {
    positioned(alt((
        value(Modifier::Public, public_tag),
        value(Modifier::Private, private_tag),
        value(Modifier::Protected, protected_tag),
    )))(input)
}

pub fn parse_statement(input: Tokens) -> TokenResult<Positioned<Statement>> {
    positioned(alt((
        map(
            type_alias::parse_type_alias_declaration,
            Statement::TypeAliasDeclaration,
        ),
        map(class::parse_class_declaration, Statement::ClassDeclaration),
        map(
            interface::parse_interface_declaration,
            Statement::InterfaceDeclaration,
        ),
        map(
            function::parse_function_declaration,
            Statement::FunctionDeclaration,
        ),
        map(
            enumeration::parse_enum_declaration,
            Statement::EnumDeclaration,
        ),
        map(
            variable::parse_variable_statement,
            Statement::VariableStatement,
        ),
        map(if_else::parse_if_statement, |statement| {
            Statement::IfStatement(Box::new(statement))
        }),
        returning::parse_return_statement,
        map(expression::parse_expression, Statement::Expression),
    )))(input)
}

pub fn parse_program_statement(input: Tokens) -> TokenResult<Positioned<Statement>> {
    terminated(
        positioned(alt((
            map(import::parse_import_declaration, |declaration| {
                Statement::ImportDeclaration(Box::new(declaration))
            }),
            map(
                export::parse_export_declaration,
                Statement::ExportDeclaration,
            ),
            map(
                type_alias::parse_type_alias_declaration,
                Statement::TypeAliasDeclaration,
            ),
            map(class::parse_class_declaration, Statement::ClassDeclaration),
            map(
                interface::parse_interface_declaration,
                Statement::InterfaceDeclaration,
            ),
            map(
                function::parse_function_declaration,
                Statement::FunctionDeclaration,
            ),
            map(
                enumeration::parse_enum_declaration,
                Statement::EnumDeclaration,
            ),
            map(
                variable::parse_variable_statement,
                Statement::VariableStatement,
            ),
            map(if_else::parse_if_statement, |statement| {
                Statement::IfStatement(Box::new(statement))
            }),
            returning::parse_return_statement,
            map(expression::parse_expression, Statement::Expression),
        ))),
        opt(semi_tag),
    )(input)
}
