use super::parse_expression;
use crate::{
    ast::{ArrowFunction, Expression},
    parsing::statement::{parse_arrow_parameter, parse_type},
    tags::{colon_tag, comma_tag, fat_arrow_tag, paren_close_tag, paren_open_tag, positioned},
};

use nom::{
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_arrow_function_expression(input: Tokens) -> TokenResult<Positioned<Expression>> {
    positioned(map(
        positioned(tuple((
            delimited(
                paren_open_tag,
                separated_list0(comma_tag, parse_arrow_parameter),
                paren_close_tag,
            ),
            opt(preceded(colon_tag, parse_type)),
            preceded(fat_arrow_tag, parse_expression),
        ))),
        |Positioned {
             value: (parameters, ty, body),
             span,
         }| {
            Expression::ArrowFunction(Box::new(span.wrap(ArrowFunction {
                type_parameters: Default::default(),
                parameters,
                ty,
                body,
                modifiers: Default::default(),
            })))
        },
    ))(input)
}
