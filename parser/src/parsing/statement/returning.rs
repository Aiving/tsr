use super::expression::parse_expression;
use crate::{
    ast::Statement,
    tags::{return_tag, semi_tag},
};

use nom::{combinator::map, sequence::delimited};
use tsr_lexer::{globals::TokenResult, tokens::Tokens};

pub fn parse_return_statement(input: Tokens) -> TokenResult<Statement> {
    map(
        delimited(return_tag, parse_expression, semi_tag),
        Statement::ReturnStatement,
    )(input)
}
