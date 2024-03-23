use super::{
    parse_ident, parse_literal,
    statement::{parse_parameter, parse_type_member, parse_type_parameter},
};

use crate::{
    ast::{
        ArraySize, IntersectionOrPrimaryType, Literal, PredefinedType, PrimaryType, Type,
        UnionOrIntersectionOrPrimaryType,
    },
    tags::{
        and_tag, any_tag, boolean_tag, brace_close_tag, brace_open_tag, bracket_close_tag,
        bracket_open_tag, comma_tag, fat_arrow_tag, float_tag, gt_tag, lt_tag, new_tag, number_tag,
        or_tag, paren_close_tag, paren_open_tag, positioned, semi_tag, string_tag, symbol_tag,
        this_tag, void_tag,
    },
};

use nom::{
    branch::alt,
    combinator::{map, map_res, opt, value},
    error::{Error, ErrorKind},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_type(input: Tokens) -> TokenResult<Positioned<Type>> {
    positioned(alt((
        map(
            preceded(
                new_tag,
                tuple((
                    opt(delimited(
                        lt_tag,
                        separated_list1(comma_tag, parse_type_parameter),
                        gt_tag,
                    )),
                    delimited(
                        paren_open_tag,
                        separated_list0(comma_tag, parse_parameter),
                        paren_close_tag,
                    ),
                    preceded(fat_arrow_tag, parse_type),
                )),
            ),
            |(type_parameters, parameters, ty)| {
                Type::ConstructorType(
                    type_parameters
                        .map(|ps| ps.into_iter().map(|p| p.value).collect())
                        .unwrap_or_default(),
                    parameters.into_iter().map(|p| p.value).collect(),
                    Box::new(ty.value),
                )
            },
        ),
        map(
            tuple((
                opt(delimited(
                    lt_tag,
                    separated_list1(comma_tag, parse_type_parameter),
                    gt_tag,
                )),
                delimited(
                    paren_open_tag,
                    separated_list0(comma_tag, parse_parameter),
                    paren_close_tag,
                ),
                preceded(fat_arrow_tag, parse_type),
            )),
            |(type_parameters, parameters, ty)| {
                Type::FunctionType(
                    type_parameters
                        .map(|ps| ps.into_iter().map(|p| p.value).collect())
                        .unwrap_or_default(),
                    parameters.into_iter().map(|p| p.value).collect(),
                    Box::new(ty.value),
                )
            },
        ),
        map(parse_union_or_intersection_or_primary_type, |ty| {
            Type::UnionOrIntersectionOrPrimaryType(ty)
        }),
    )))(input)
}

pub fn parse_union_or_intersection_or_primary_type(
    input: Tokens,
) -> TokenResult<UnionOrIntersectionOrPrimaryType> {
    alt((
        map(
            tuple((
                parse_intersection_or_primary_type,
                preceded(or_tag, parse_intersection_or_primary_type),
                opt(preceded(
                    or_tag,
                    separated_list1(or_tag, parse_intersection_or_primary_type),
                )),
            )),
            |(first_type, second_type, types)| {
                UnionOrIntersectionOrPrimaryType::UnionType(
                    [vec![first_type, second_type], types.unwrap_or_default()].concat(),
                )
            },
        ),
        map(parse_intersection_or_primary_type, |ty| {
            UnionOrIntersectionOrPrimaryType::IntersectionOrPrimaryType(ty)
        }),
    ))(input)
}

pub fn parse_intersection_or_primary_type(input: Tokens) -> TokenResult<IntersectionOrPrimaryType> {
    alt((
        map(
            tuple((
                parse_primary_type,
                preceded(and_tag, parse_primary_type),
                opt(preceded(
                    and_tag,
                    separated_list1(and_tag, parse_primary_type),
                )),
            )),
            |(first_type, second_type, types)| {
                IntersectionOrPrimaryType::IntersectionType(
                    [vec![first_type, second_type], types.unwrap_or_default()].concat(),
                )
            },
        ),
        map(parse_primary_type, |ty| {
            IntersectionOrPrimaryType::PrimaryType(ty)
        }),
    ))(input)
}

pub fn parse_primary_type(input: Tokens) -> TokenResult<PrimaryType> {
    alt((
        map(
            delimited(paren_open_tag, parse_type, paren_close_tag),
            |ty| PrimaryType::ParenthesizedType(Box::new(ty.value)),
        ),
        map(parse_predefined_type, PrimaryType::PredefinedType),
        map(
            pair(
                parse_ident,
                opt(delimited(
                    lt_tag,
                    separated_list1(comma_tag, parse_ident),
                    gt_tag,
                )),
            ),
            |(ty, type_parameters)| {
                PrimaryType::TypeReference(
                    ty.value,
                    type_parameters
                        .map(|p| p.into_iter().map(|p| p.value).collect())
                        .unwrap_or_default(),
                )
            },
        ),
        map(
            delimited(
                brace_open_tag,
                terminated(
                    separated_list0(alt((comma_tag, semi_tag)), parse_type_member),
                    opt(alt((comma_tag, semi_tag))),
                ),
                brace_close_tag,
            ),
            |object| PrimaryType::ObjectType(object.into_iter().map(|tm| tm.value).collect()),
        ),
        map(
            delimited(
                bracket_open_tag,
                separated_list0(comma_tag, parse_type),
                bracket_close_tag,
            ),
            |tuple| PrimaryType::TupleType(tuple.into_iter().map(|ty| ty.value).collect()),
        ),
        map(
            tuple((
                parse_primary_type,
                delimited(bracket_open_tag, opt(parse_literal), bracket_close_tag),
            )),
            |(ty, size)| {
                PrimaryType::ArrayType(
                    Box::new(ty),
                    match size {
                        Some(size) => match size.value {
                            Literal::Number(value) => ArraySize::Fixed(value.value as usize),
                            _ => ArraySize::Dynamic,
                        },
                        None => ArraySize::Dynamic,
                    },
                )
            },
        ),
        map(this_tag, |_| PrimaryType::ThisType),
    ))(input)
}

pub fn parse_predefined_type(input: Tokens) -> TokenResult<PredefinedType> {
    alt((
        value(PredefinedType::Any, any_tag),
        value(PredefinedType::Number, number_tag),
        value(PredefinedType::Float, float_tag),
        value(PredefinedType::Boolean, boolean_tag),
        value(PredefinedType::String, string_tag),
        map_res(parse_literal, |literal| match literal.value {
            Literal::String(string) => Ok(PredefinedType::StringLiteral(string.value)),
            _ => Err(Error::new(input, ErrorKind::Not)),
        }),
        value(PredefinedType::Symbol, symbol_tag),
        value(PredefinedType::Void, void_tag),
    ))(input)
}
