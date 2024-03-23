use super::parse_expression;
use crate::{
    ast::{Expression, MatchExpression, SwitchVariant},
    parsing::statement::parse_statement,
    tags::{
        brace_close_tag, brace_open_tag, comma_tag, fat_arrow_tag, match_tag, paren_close_tag,
        paren_open_tag, positioned, when_tag,
    },
};

use nom::{
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, terminated},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_switch_expression(input: Tokens) -> TokenResult<Positioned<Expression>> {
    positioned(map(
        positioned(pair(
            preceded(
                match_tag,
                delimited(paren_open_tag, parse_expression, paren_close_tag),
            ),
            delimited(
                brace_open_tag,
                terminated(
                    separated_list0(
                        comma_tag,
                        positioned(map(
                            pair(
                                preceded(
                                    when_tag,
                                    delimited(paren_open_tag, parse_expression, paren_close_tag),
                                ),
                                preceded(fat_arrow_tag, parse_statement),
                            ),
                            |(value, callback)| SwitchVariant { value, callback },
                        )),
                    ),
                    opt(comma_tag),
                ),
                brace_close_tag,
            ),
        )),
        |Positioned {
             value: (target, variants),
             span,
         }| {
            Expression::MatchExpression(Box::new(span.wrap(MatchExpression { target, variants })))
        },
    ))(input)
}
