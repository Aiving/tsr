use super::parse_expression;
use crate::{
    ast::{BinaryExpression, Expression},
    tags::{not_tag, positioned},
};

use nom::{combinator::map, sequence::pair};
use tsr_lexer::{
    globals::{Positioned, TokenResult},
    token::Operator,
    tokens::Tokens,
};

pub fn parse_simple_binary_expression(input: Tokens) -> TokenResult<Positioned<Expression>> {
    positioned(map(pair(not_tag, parse_expression), |(operator, left)| {
        Expression::BinaryExpression(Box::new(operator.span.between(left.span).wrap(
            BinaryExpression {
                operator: operator.span.wrap(Operator::Not),
                right: left.span.wrap(Expression::Null),
                left,
            },
        )))
    }))(input)
}
