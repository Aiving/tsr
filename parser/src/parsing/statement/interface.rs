use super::{parse_ident, parse_type_member, parse_type_parameter};
use crate::{
    ast::InterfaceDeclaration,
    tags::{
        brace_close_tag, brace_open_tag, comma_tag, extends_tag, gt_tag, interface_tag, lt_tag,
        positioned, semi_tag,
    },
};

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_interface_declaration(input: Tokens) -> TokenResult<Positioned<InterfaceDeclaration>> {
    positioned(map(
        tuple((
            interface_tag,
            parse_ident,
            opt(delimited(
                lt_tag,
                separated_list1(comma_tag, parse_type_parameter),
                gt_tag,
            )),
            opt(preceded(
                extends_tag,
                separated_list1(comma_tag, parse_ident),
            )),
            delimited(
                brace_open_tag,
                terminated(
                    separated_list0(alt((comma_tag, semi_tag)), parse_type_member),
                    alt((comma_tag, semi_tag)),
                ),
                brace_close_tag,
            ),
        )),
        |(_, name, type_parameters, extends, members)| InterfaceDeclaration {
            name,
            members,
            type_parameters: type_parameters.unwrap_or_default(),
            extends: extends.unwrap_or_default(),
        },
    ))(input)
}
