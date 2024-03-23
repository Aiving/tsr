use crate::globals::ByteResult;
use crate::globals::BytesSpan;
use crate::globals::Positioned;
use crate::globals::Span;
use crate::util::complete_byte_slice_str_from_utf8;

use nom::branch::alt;
use nom::bytes::complete::tag;

use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;

use nom::combinator::map_res;
use nom::combinator::recognize;

use nom::multi::many0;

use nom::sequence::pair;

use super::token::BuiltInType;
use super::token::Literal;
use super::token::Modifier;
use super::token::ReservedWord;
use super::token::Token;

pub fn lex_reserved_ident(input: BytesSpan) -> ByteResult<Positioned<Token>> {
    map_res(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: BytesSpan| {
            let c = complete_byte_slice_str_from_utf8(s);
            let s: Span = s.into();

            c.map(|syntax| {
                s.wrap(match syntax {
                    "public" => Token::Modifier(Modifier::Public),
                    "private" => Token::Modifier(Modifier::Private),
                    "protected" => Token::Modifier(Modifier::Protected),
                    "static" => Token::Modifier(Modifier::Static),
                    "async" => Token::Modifier(Modifier::Async),

                    "const" => Token::ReservedWord(ReservedWord::Const),
                    "let" => Token::ReservedWord(ReservedWord::Let),
                    "operator" => Token::ReservedWord(ReservedWord::Operator),
                    "constructor" => Token::ReservedWord(ReservedWord::Constructor),
                    "class" => Token::ReservedWord(ReservedWord::Class),
                    "interface" => Token::ReservedWord(ReservedWord::Interface),
                    "implements" => Token::ReservedWord(ReservedWord::Implements),
                    "this" => Token::ReservedWord(ReservedWord::This),
                    "return" => Token::ReservedWord(ReservedWord::Return),
                    "function" => Token::ReservedWord(ReservedWord::Function),
                    "if" => Token::ReservedWord(ReservedWord::If),
                    "else" => Token::ReservedWord(ReservedWord::Else),
                    "new" => Token::ReservedWord(ReservedWord::New),
                    "null" => Token::ReservedWord(ReservedWord::Null),
                    "enum" => Token::ReservedWord(ReservedWord::Enum),
                    "namespace" => Token::ReservedWord(ReservedWord::Namespace),
                    "declare" => Token::ReservedWord(ReservedWord::Declare),
                    "export" => Token::ReservedWord(ReservedWord::Export),
                    "import" => Token::ReservedWord(ReservedWord::Import),
                    "default" => Token::ReservedWord(ReservedWord::Default),
                    "when" => Token::ReservedWord(ReservedWord::When),
                    "match" => Token::ReservedWord(ReservedWord::Match),
                    "extends" => Token::ReservedWord(ReservedWord::Extends),
                    "get" => Token::ReservedWord(ReservedWord::Get),
                    "set" => Token::ReservedWord(ReservedWord::Set),
                    "type" => Token::ReservedWord(ReservedWord::Type),
                    "typeOf" => Token::ReservedWord(ReservedWord::TypeOf),
                    "for" => Token::ReservedWord(ReservedWord::For),
                    "in" => Token::ReservedWord(ReservedWord::In),
                    "of" => Token::ReservedWord(ReservedWord::Of),
                    "as" => Token::ReservedWord(ReservedWord::As),
                    "from" => Token::ReservedWord(ReservedWord::From),

                    "any" => Token::BuiltInType(BuiltInType::Any),
                    "number" => Token::BuiltInType(BuiltInType::Number),
                    "float" => Token::BuiltInType(BuiltInType::Float),
                    "boolean" => Token::BuiltInType(BuiltInType::Boolean),
                    "string" => Token::BuiltInType(BuiltInType::String),
                    "symbol" => Token::BuiltInType(BuiltInType::Symbol),
                    "void" => Token::BuiltInType(BuiltInType::Void),

                    "true" => Token::Literal(Literal::Boolean(true)),
                    "false" => Token::Literal(Literal::Boolean(false)),

                    _ => Token::Ident(syntax.to_string()),
                })
            })
        },
    )(input)
}
