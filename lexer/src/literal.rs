use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::AsBytes;
use nom_locate::position;

use crate::globals::ByteResult;
use crate::globals::BytesSpan;
use crate::globals::Positioned;
use crate::globals::Span;
use crate::util::complete_byte_slice_str_from_utf8;
use crate::util::complete_str_from_str;
use crate::util::concat_slice_vec;
use crate::util::convert_vec_utf8;

use super::punctuation::double_quote_punctuation;
use super::token::Literal;
use super::token::Token;

pub fn lex_integer(input: BytesSpan) -> ByteResult<Positioned<Token>> {
    let (input, start) = position(input)?;
    let (input, value) = alt((map(
        map_res(
            map_res(pair(opt(tag("-")), digit1), |(is_negative, data)| {
                complete_byte_slice_str_from_utf8(data).map(|number| (is_negative, number))
            }),
            |(is_negative, data)| {
                complete_str_from_str(data).map(|number: i64| {
                    if is_negative.is_some() {
                        -number
                    } else {
                        number
                    }
                })
            },
        ),
        Literal::Number,
    ),))(input)?;
    let (input, end) = position(input)?;
    let start: Span = start.into();
    let end: Span = end.into();

    Ok((input, start.between(end).wrap(Token::Literal(value))))
}

fn pis(input: BytesSpan) -> ByteResult<Vec<u8>> {
    use std::result::Result::*;

    let (second_input, character) = take(1usize)(input)?;

    match character.as_bytes() {
        b"\"" => Ok((input, vec![])),
        b"\\" => {
            let (i2, c2) = take(1usize)(second_input)?;

            pis(i2).map(|(slice, done)| (slice, concat_slice_vec(c2.fragment(), done)))
        }
        c => pis(second_input).map(|(slice, done)| (slice, concat_slice_vec(c, done))),
    }
}

pub fn lex_string(input: BytesSpan) -> ByteResult<Positioned<Token>> {
    let (input, start) = position(input)?;
    let (input, value) = delimited(
        double_quote_punctuation,
        map_res(pis, convert_vec_utf8),
        double_quote_punctuation,
    )(input)?;
    let (input, end) = position(input)?;
    let start: Span = start.into();
    let end: Span = end.into();

    Ok((
        input,
        start
            .between(end)
            .wrap(Token::Literal(Literal::String(value))),
    ))
}

pub fn lex_literal(input: BytesSpan) -> ByteResult<Positioned<Token>> {
    alt((lex_string, lex_integer))(input)
}
