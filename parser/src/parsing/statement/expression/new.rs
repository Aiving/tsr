use super::{parse_expression, primitives::parse_ident_expression};
use crate::{
    ast::{Expression, NewExpression},
    parsing::statement::parse_type_parameter,
    tags::{comma_tag, gt_tag, lt_tag, new_tag, paren_close_tag, paren_open_tag, positioned},
};

use nom::{
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_new_expression(input: Tokens) -> TokenResult<Positioned<Expression>> {
    map(
        positioned(map(
            preceded(
                new_tag,
                tuple((
                    parse_ident_expression,
                    opt(delimited(
                        lt_tag,
                        separated_list1(comma_tag, parse_type_parameter),
                        gt_tag,
                    )),
                    delimited(
                        paren_open_tag,
                        separated_list0(comma_tag, parse_expression),
                        paren_close_tag,
                    ),
                )),
            ),
            |(expression, type_parameters, arguments)| NewExpression {
                expression: Box::new(expression),
                type_parameters: type_parameters.unwrap_or_default(),
                arguments,
            },
        )),
        |expression| expression.span.wrap(Expression::NewExpression(expression)),
    )(input)
}
