use nom::branch::alt;

use crate::globals::ByteResult;
use crate::globals::BytesSpan;
use crate::globals::Positioned;

use crate::syntax;

use super::token::Operator;
use super::token::Token;

syntax! {
    and_operator: "&" => Token::Operator(Operator::And);
    and_and_operator: "&&" => Token::Operator(Operator::AndAnd);
    equal_operator: "==" => Token::Operator(Operator::EqEq);
    not_equal_operator: "!=" => Token::Operator(Operator::Ne);
    or_operator: "|" => Token::Operator(Operator::Or);
    or_or_operator: "||" => Token::Operator(Operator::OrOr);
    assign_operator: "=" => Token::Operator(Operator::Eq);
    plus_plus_operator: "++" => Token::Operator(Operator::PlusPlus);
    plus_operator: "+" => Token::Operator(Operator::Plus);
    minus_minus_operator: "--" => Token::Operator(Operator::MinusMinus);
    minus_operator: "-" => Token::Operator(Operator::Minus);
    multiply_operator: "*" => Token::Operator(Operator::Star);
    divide_operator: "/" => Token::Operator(Operator::Slash);
    not_operator: "!" => Token::Operator(Operator::Not);
    greater_equal_operator: ">=" => Token::Operator(Operator::Ge);
    lesser_equal_operator: "<=" => Token::Operator(Operator::Le);
    greater_operator: ">" => Token::Operator(Operator::Gt);
    lesser_operator: "<" => Token::Operator(Operator::Lt);
}

pub fn lex_operator(input: BytesSpan) -> ByteResult<Positioned<Token>> {
    alt((
        and_operator,
        and_and_operator,
        equal_operator,
        not_equal_operator,
        or_operator,
        or_or_operator,
        assign_operator,
        plus_plus_operator,
        plus_operator,
        minus_minus_operator,
        minus_operator,
        multiply_operator,
        divide_operator,
        not_operator,
        greater_equal_operator,
        lesser_equal_operator,
        greater_operator,
        lesser_operator,
    ))(input)
}
