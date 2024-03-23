/// TOOD (Aiving): Add built-in types (Union for example).
///
/// Built-in types are tokens like reserved words, but only for the types.
#[derive(Clone, Debug, PartialEq)]
pub enum BuiltInType {
    Any,
    Number,
    Float,
    Boolean,
    String,
    Symbol,
    Void,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReservedWord {
    Const,
    Let,
    Operator,
    Constructor,
    Class,
    Interface,
    Implements,
    This,
    Return,
    Function,
    If,
    Else,
    New,
    Null,
    Enum,
    Namespace,
    Declare,
    Export,
    Import,
    Default,
    Match,
    Extends,
    Get,
    Set,
    Type,
    TypeOf,
    For,
    In,
    Of,
    As,
    From,
    When
}

#[derive(Clone, Debug, PartialEq)]
pub enum Punctuation {
    Comma,
    Dot,
    Ellipsis,
    FatArrow,
    DoubleSlash,
    DoubleQuote,
    Colon,
    Semi,
    Question,
    Pound,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    And,
    AndAnd,
    Plus,
    Star,
    Slash,
    Or,
    OrOr,
    PlusPlus,
    Minus,
    MinusMinus,
    EqEq,
    Eq,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
    Not,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Modifier {
    Public,
    Private,
    Protected,
    Async,
    Static,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Delimiter {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    ParenOpen,
    ParenClose,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,
    Comment(String),
    Delimiter(Delimiter),
    Ident(String),
    Literal(Literal),
    BuiltInType(BuiltInType),
    ReservedWord(ReservedWord),
    Punctuation(Punctuation),
    Modifier(Modifier),
    Operator(Operator),
}
