pub mod signatures;
pub mod statement;
pub mod types;

use self::statement::parse_statement;
use super::{
    ast::{Block, Ident, Literal},
    tags::{brace_close_tag, brace_open_tag, from_tag, position, semi_tag},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    token::{self, Token},
    tokens::Tokens,
};

use nom::{
    bytes::complete::take,
    combinator::{map_res, opt},
    error::{Error, ErrorKind},
    multi::many0,
    sequence::{delimited, preceded, terminated},
    Err,
};

pub fn parse_literal(input: Tokens) -> TokenResult<Positioned<Literal>> {
    let (tokens, token) = take(1usize)(input)?;

    let token = &token.tok[0];

    match &token.value {
        Token::Literal(literal) => Ok((
            tokens,
            token.wrap(match literal {
                token::Literal::String(string) => Literal::String(token.wrap(string.clone())),
                token::Literal::Number(number) => Literal::Number(token.wrap(*number)),
                token::Literal::Float(float) => Literal::Float(token.wrap(*float)),
                token::Literal::Boolean(boolean) => Literal::Boolean(token.wrap(*boolean)),
            }),
        )),
        _ => Err(Err::Error(Error::new(input, ErrorKind::Tag))),
    }
}

pub fn parse_ident(input: Tokens) -> TokenResult<Positioned<Ident>> {
    let (i1, t1) = take(1usize)(input)?;

    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match &t1.tok[0].value {
            Token::Ident(name) => Ok((i1, t1.tok[0].wrap(Ident(name.clone())))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag))),
        }
    }
}

pub fn parse_from_clause(input: Tokens) -> TokenResult<Positioned<String>> {
    map_res(preceded(from_tag, parse_literal), |literal| {
        match literal.value {
            Literal::String(string) => Ok(string),
            _ => Err(Error::new(input, ErrorKind::Not)),
        }
    })(input)
}

pub fn parse_code_block(input: Tokens) -> TokenResult<Block> {
    let (input, start) = position(input)?;
    let (input, value) = delimited(
        brace_open_tag,
        many0(terminated(parse_statement, opt(semi_tag))),
        brace_close_tag,
    )(input)?;
    let (input, end) = position(input)?;

    Ok((input, start.between(end).wrap(value)))
}
