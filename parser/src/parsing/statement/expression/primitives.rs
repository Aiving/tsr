use crate::{
    ast::Expression,
    parsing::{parse_ident, parse_literal},
    tags::{null_tag, positioned, this_tag},
};

use nom::combinator::map;
use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_literal_expression(input: Tokens) -> TokenResult<Positioned<Expression>> {
    positioned(map(parse_literal, Expression::Literal))(input)
}

pub fn parse_ident_expression(input: Tokens) -> TokenResult<Positioned<Expression>> {
    positioned(map(parse_ident, Expression::Ident))(input)
}

pub fn parse_this(input: Tokens) -> TokenResult<Positioned<Expression>> {
    map(this_tag, |tag| tag.wrap(Expression::This))(input)
}

pub fn parse_null(input: Tokens) -> TokenResult<Positioned<Expression>> {
    map(null_tag, |tag| tag.wrap(Expression::Null))(input)
}
