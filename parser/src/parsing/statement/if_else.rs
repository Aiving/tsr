use super::{super::parse_code_block, expression::parse_expression, parse_statement};
use crate::{
    ast::{Expression, IfStatement},
    tags::{else_tag, if_tag, positioned},
};

use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_if_statement(input: Tokens) -> TokenResult<Positioned<IfStatement>> {
    map(
        positioned(tuple((
            if_tag,
            alt((
                parse_expression,
                map(parse_code_block, |block| {
                    block.span.wrap(Expression::Block(block))
                }),
            )),
            parse_statement,
            opt(preceded(else_tag, parse_statement)),
        ))),
        |Positioned {
             value: (_, expression, then_statement, else_statement),
             span,
         }| {
            span.wrap(IfStatement {
                expression,
                then_statement,
                else_statement,
            })
        },
    )(input)
}
