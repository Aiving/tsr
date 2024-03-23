use super::{super::parse_code_block, parse_call_signature, parse_ident};
use crate::{
    ast::FunctionDeclaration,
    tags::{async_tag, function_tag, positioned},
};

use nom::{
    combinator::{map, opt, value},
    sequence::{preceded, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    token::Modifier,
    tokens::Tokens,
};

pub fn parse_function_declaration(input: Tokens) -> TokenResult<Positioned<FunctionDeclaration>> {
    positioned(map(
        tuple((
            opt(positioned(value(Modifier::Async, async_tag))),
            preceded(function_tag, parse_ident),
            parse_call_signature,
            opt(parse_code_block),
        )),
        |(async_modifier, name, signature, body)| FunctionDeclaration {
            name,
            type_parameters: signature.value.0,
            parameters: signature.value.1,
            ty: signature.value.2,
            body,
            modifiers: async_modifier
                .map(|modifier| vec![modifier])
                .unwrap_or_default(),
        },
    ))(input)
}
