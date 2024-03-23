use crate::{ast::Expression, parsing::parse_code_block};
use nom::combinator::map;
use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_code_block_expression(input: Tokens) -> TokenResult<Positioned<Expression>> {
    map(parse_code_block, |block| {
        block.span.wrap(Expression::Block(block))
    })(input)
}
