use super::{
    expression::parse_expression, parse_access_modifier, parse_call_signature, parse_ident,
    parse_index_signature, parse_parameter, parse_property_name, parse_type, parse_type_parameter,
};

use crate::{
    ast::{
        AccessorKind, ClassDeclaration, ClassElement, ConstructorDeclaration, IndexSignature,
        MemberAccessorDeclaration, MemberFunctionDeclaration, MemberVariableDeclaration,
        PropertyMemberDeclaration, Statement,
    },
    parsing::parse_code_block,
    tags::{
        brace_close_tag, brace_open_tag, class_tag, colon_tag, comma_tag, constructor_tag, eq_tag,
        extends_tag, fat_arrow_tag, get_tag, gt_tag, implements_tag, lt_tag, paren_close_tag,
        paren_open_tag, positioned, semi_tag, set_tag, static_tag,
    },
};

use nom::{
    branch::alt,
    combinator::{map, opt, value},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    token::Modifier,
    tokens::Tokens,
};

pub fn parse_constructor_declaration(
    input: Tokens,
) -> TokenResult<Positioned<ConstructorDeclaration>> {
    positioned(map(
        tuple((
            many0(parse_access_modifier),
            preceded(
                constructor_tag,
                delimited(
                    paren_open_tag,
                    separated_list0(comma_tag, parse_parameter),
                    paren_close_tag,
                ),
            ),
            alt((
                map(preceded(fat_arrow_tag, parse_expression), |expression| {
                    expression.span.wrap(vec![expression
                        .span
                        .wrap(Statement::Expression(expression))])
                }),
                parse_code_block,
            )),
        )),
        |(modifiers, parameters, body)| ConstructorDeclaration {
            modifiers,
            parameters,
            body,
        },
    ))(input)
}

pub fn parse_member_variable_declaration(
    input: Tokens,
) -> TokenResult<Positioned<MemberVariableDeclaration>> {
    positioned(map(
        terminated(
            tuple((
                many0(parse_access_modifier),
                opt(positioned(value(Modifier::Static, static_tag))),
                parse_property_name,
                opt(preceded(colon_tag, parse_type)),
                opt(preceded(eq_tag, parse_expression)),
            )),
            semi_tag,
        ),
        |(modifiers, static_modifier, name, ty, initializer)| MemberVariableDeclaration {
            modifiers: [
                modifiers,
                static_modifier
                    .map(|modifier| vec![modifier])
                    .unwrap_or_default(),
            ]
            .concat(),
            name,
            ty,
            initializer,
        },
    ))(input)
}

pub fn parse_member_function_declaration(
    input: Tokens,
) -> TokenResult<Positioned<MemberFunctionDeclaration>> {
    positioned(map(
        tuple((
            many0(parse_access_modifier),
            opt(positioned(value(Modifier::Static, static_tag))),
            parse_property_name,
            parse_call_signature,
            alt((
                map(
                    terminated(preceded(fat_arrow_tag, parse_expression), semi_tag),
                    |expression| {
                        expression.span.wrap(vec![expression
                            .span
                            .wrap(Statement::Expression(expression))])
                    },
                ),
                parse_code_block,
            )),
        )),
        |(modifiers, static_modifier, name, signature, body)| MemberFunctionDeclaration {
            modifiers: [
                modifiers,
                static_modifier
                    .map(|modifier| vec![modifier])
                    .unwrap_or_default(),
            ]
            .concat(),
            name,
            type_parameters: signature.value.0,
            parameters: signature.value.1,
            ty: signature.value.2,
            body,
        },
    ))(input)
}

pub fn parse_member_accessor_declaration(
    input: Tokens,
) -> TokenResult<Positioned<MemberAccessorDeclaration>> {
    positioned(map(
        tuple((
            many0(parse_access_modifier),
            opt(positioned(value(Modifier::Static, static_tag))),
            alt((
                map(
                    tuple((
                        set_tag,
                        parse_property_name,
                        delimited(
                            paren_open_tag,
                            pair(parse_ident, preceded(colon_tag, parse_type)),
                            paren_close_tag,
                        ),
                    )),
                    |(tag, name, (parameter, ty))| {
                        (tag.wrap(AccessorKind::Setter), name, Some(parameter), ty)
                    },
                ),
                map(
                    tuple((
                        get_tag,
                        parse_property_name,
                        paren_open_tag,
                        paren_close_tag,
                        preceded(colon_tag, parse_type),
                    )),
                    |(tag, name, _, _, ty)| (tag.wrap(AccessorKind::Getter), name, None, ty),
                ),
            )),
            alt((
                map(
                    delimited(fat_arrow_tag, parse_expression, semi_tag),
                    |expression| {
                        expression.span.wrap(vec![expression
                            .span
                            .wrap(Statement::Expression(expression))])
                    },
                ),
                parse_code_block,
            )),
        )),
        |(modifiers, static_modifier, (kind, name, parameter, ty), body)| {
            MemberAccessorDeclaration {
                modifiers: [
                    modifiers,
                    static_modifier
                        .map(|modifier| vec![modifier])
                        .unwrap_or_default(),
                ]
                .concat(),
                name,
                kind,
                parameter,
                ty,
                body,
            }
        },
    ))(input)
}

pub fn parse_property_member_declaration(
    input: Tokens,
) -> TokenResult<Positioned<PropertyMemberDeclaration>> {
    positioned(alt((
        map(
            parse_member_variable_declaration,
            PropertyMemberDeclaration::MemberVariableDeclaration,
        ),
        map(
            parse_member_function_declaration,
            PropertyMemberDeclaration::MemberFunctionDeclaration,
        ),
        map(
            parse_member_accessor_declaration,
            PropertyMemberDeclaration::MemberAccessorDeclaration,
        ),
    )))(input)
}

pub fn parse_index_member_declaration(input: Tokens) -> TokenResult<Positioned<IndexSignature>> {
    terminated(parse_index_signature, semi_tag)(input)
}

pub fn parse_class_element(input: Tokens) -> TokenResult<Positioned<ClassElement>> {
    positioned(alt((
        map(
            parse_constructor_declaration,
            ClassElement::ConstructorDeclaration,
        ),
        map(
            parse_property_member_declaration,
            ClassElement::PropertyMemberDeclaration,
        ),
        map(
            parse_index_member_declaration,
            ClassElement::IndexMemberDeclaration,
        ),
    )))(input)
}

pub fn parse_class_declaration(input: Tokens) -> TokenResult<Positioned<ClassDeclaration>> {
    positioned(map(
        tuple((
            preceded(class_tag, parse_ident),
            opt(delimited(
                lt_tag,
                separated_list1(comma_tag, parse_type_parameter),
                gt_tag,
            )),
            opt(preceded(
                extends_tag,
                separated_list1(comma_tag, parse_ident),
            )),
            opt(preceded(
                implements_tag,
                separated_list1(comma_tag, parse_ident),
            )),
            delimited(brace_open_tag, many0(parse_class_element), brace_close_tag),
        )),
        |(name, type_parameters, extends, implements, body)| ClassDeclaration {
            name,
            type_parameters: type_parameters.unwrap_or_default(),
            extends: extends.unwrap_or_default(),
            implements: implements.unwrap_or_default(),
            body,
        },
    ))(input)
}
