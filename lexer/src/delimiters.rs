use super::token::{Delimiter, Token};
use crate::{
    globals::{ByteResult, BytesSpan, Positioned},
    syntax,
};
use nom::branch::alt;

syntax! {
   lparen_delimiter: "(" => Token::Delimiter(Delimiter::ParenOpen);
   rparen_delimiter: ")" => Token::Delimiter(Delimiter::ParenClose);
   lbrace_delimiter: "{" => Token::Delimiter(Delimiter::BraceOpen);
   rbrace_delimiter: "}" => Token::Delimiter(Delimiter::BraceClose);
   lbracket_delimiter: "[" => Token::Delimiter(Delimiter::BracketOpen);
   rbracket_delimiter: "]" => Token::Delimiter(Delimiter::BracketClose);
}

pub fn lex_delimiter(input: BytesSpan) -> ByteResult<Positioned<Token>> {
    alt((
        lparen_delimiter,
        rparen_delimiter,
        lbrace_delimiter,
        rbrace_delimiter,
        lbracket_delimiter,
        rbracket_delimiter,
    ))(input)
}
