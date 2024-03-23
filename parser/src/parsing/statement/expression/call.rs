use super::parse_expression;
use crate::{
    ast::{Expression, FunctionCallExpression},
    parsing::parse_code_block,
    tags::{comma_tag, paren_close_tag, paren_open_tag, positioned},
};

use nom::{
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_call_expression(
    input: Tokens,
    fn_handle: Positioned<Expression>,
) -> TokenResult<Positioned<Expression>> {
    map(
        positioned(pair(
            delimited(
                paren_open_tag,
                separated_list0(comma_tag, parse_expression),
                paren_close_tag,
            ),
            opt(parse_code_block),
        )),
        |Positioned {
             value: (arguments, lambda),
             span,
         }| {
            span.wrap(Expression::FunctionCallExpression(Box::new(span.wrap(
                FunctionCallExpression {
                    function: Box::new(fn_handle.clone()),
                    arguments,
                    lambda,
                },
            ))))
        },
    )(input)
}
