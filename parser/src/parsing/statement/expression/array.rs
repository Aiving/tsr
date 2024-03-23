use super::parse_expression;
use crate::{
    ast::Expression,
    tags::{bracket_close_tag, bracket_open_tag, comma_tag, ellipsis_tag, positioned},
};

use nom::{
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_array(input: Tokens) -> TokenResult<Positioned<Expression>> {
    positioned(map(
        delimited(
            bracket_open_tag,
            pair(
                separated_list0(comma_tag, parse_expression),
                positioned(opt(ellipsis_tag)),
            ),
            bracket_close_tag,
        ),
        |(elements, dynamic)| Expression::Array {
            elements,
            is_dynamic: dynamic.span.wrap(dynamic.value.is_some()),
        },
    ))(input)
}
