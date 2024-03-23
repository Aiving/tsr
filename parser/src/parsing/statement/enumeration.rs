use super::{expression::parse_expression, parse_ident};
use crate::{
    ast::{EnumDeclaration, EnumMember},
    tags::{brace_close_tag, brace_open_tag, comma_tag, enum_tag, eq_tag, positioned},
};

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_enum_declaration(input: Tokens) -> TokenResult<Positioned<EnumDeclaration>> {
    positioned(map(
        tuple((
            enum_tag,
            parse_ident,
            delimited(
                brace_open_tag,
                terminated(
                    separated_list0(
                        comma_tag,
                        positioned(alt((
                            map(
                                tuple((parse_ident, eq_tag, parse_expression)),
                                |(name, _, init)| EnumMember {
                                    name,
                                    initializer: Some(init),
                                },
                            ),
                            map(parse_ident, |name| EnumMember {
                                name,
                                initializer: None,
                            }),
                        ))),
                    ),
                    opt(comma_tag),
                ),
                brace_close_tag,
            ),
        )),
        |(_, name, members)| EnumDeclaration { name, members },
    ))(input)
}
