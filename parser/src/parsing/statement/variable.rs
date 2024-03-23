use super::{expression::parse_expression, parse_ident, parse_type};
use crate::{
    ast::{VariableDeclaration, VariableStatement},
    tags::{colon_tag, comma_tag, const_tag, eq_tag, let_tag, positioned, question_tag, semi_tag},
};

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    token::{ReservedWord, Token},
    tokens::Tokens,
};

pub fn parse_variable_statement(input: Tokens) -> TokenResult<Positioned<VariableStatement>> {
    positioned(map(
        tuple((
            alt((let_tag, const_tag)),
            terminated(
                separated_list1(
                    comma_tag,
                    positioned(map(
                        tuple((
                            parse_ident,
                            positioned(opt(question_tag)),
                            opt(preceded(colon_tag, parse_type)),
                            opt(preceded(eq_tag, parse_expression)),
                        )),
                        |(name, nullable, ty, init)| VariableDeclaration {
                            name,
                            ty,
                            nullable: nullable.wrap(nullable.value.is_some()),
                            initializer: init,
                        },
                    )),
                ),
                semi_tag,
            ),
        )),
        |(kind, declarations)| VariableStatement {
            mutable: kind.wrap(kind.value.tok[0].value == Token::ReservedWord(ReservedWord::Let)),
            declarations,
        },
    ))(input)
}
