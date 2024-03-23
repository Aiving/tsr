use nom::branch::alt;

use crate::globals::ByteResult;
use crate::globals::BytesSpan;
use crate::globals::Positioned;

use crate::syntax;

use super::token::Punctuation;
use super::token::Token;

syntax! {
    comma_punctuation: "," => Token::Punctuation(Punctuation::Comma);
    dot_punctuation: "." => Token::Punctuation(Punctuation::Dot);
    ellipsis_punctuation: "..." => Token::Punctuation(Punctuation::Ellipsis);
    fat_arrow_punctuation: "=>" => Token::Punctuation(Punctuation::FatArrow);
    double_slash_punctuation: "//" => Token::Punctuation(Punctuation::DoubleSlash);
    double_quote_punctuation: "\"" => Token::Punctuation(Punctuation::DoubleQuote);
    colon_punctuation: ":" => Token::Punctuation(Punctuation::Colon);
    semi_punctuation: ";" => Token::Punctuation(Punctuation::Semi);
    question_punctuation: "?" => Token::Punctuation(Punctuation::Question);
    pound_punctuation: "#" => Token::Punctuation(Punctuation::Pound);
}

pub fn lex_punctuation(input: BytesSpan) -> ByteResult<Positioned<Token>> {
    alt((
        comma_punctuation,
        ellipsis_punctuation,
        dot_punctuation,
        fat_arrow_punctuation,
        double_slash_punctuation,
        double_quote_punctuation,
        colon_punctuation,
        semi_punctuation,
        question_punctuation,
        pound_punctuation,
    ))(input)
}
