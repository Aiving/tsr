use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Parser as _;

use tsr_lexer::globals::Positioned;
use tsr_lexer::globals::TokenResult;

use tsr_lexer::token::Token;
use tsr_lexer::tokens::Tokens;

use self::ast::Block;
use self::parsing::statement::parse_program_statement;
use self::tags::eof_tag;
use self::tags::position;

pub mod ast;
pub mod parsing;
pub mod tags;

pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: &[Positioned<Token>]) -> TokenResult<Block> {
        map(
            tuple((position, many0(parse_program_statement), eof_tag)),
            |(start, program, end)| start.between(end.span).wrap(program),
        )
        .parse(Tokens::new(tokens))
    }
}
