use super::{parse_ident, parse_type, parse_type_parameter};
use crate::{
    ast::TypeAliasDeclaration,
    tags::{comma_tag, eq_tag, gt_tag, lt_tag, positioned, semi_tag, type_tag},
};

use nom::{
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_type_alias_declaration(
    input: Tokens,
) -> TokenResult<Positioned<TypeAliasDeclaration>> {
    positioned(map(
        tuple((
            preceded(type_tag, parse_ident),
            opt(delimited(
                lt_tag,
                separated_list1(comma_tag, parse_type_parameter),
                gt_tag,
            )),
            terminated(preceded(eq_tag, parse_type), semi_tag),
        )),
        |(name, type_parameters, ty)| TypeAliasDeclaration {
            name,
            type_parameters: type_parameters.unwrap_or_default(),
            ty,
        },
    ))(input)
}
