use std::fmt;
use TokenKind::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub literal: &'a str,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Identifiers
    Ident,

    // Literals
    Bool,  //
    Int,   // 69
    Float, // 1.0, 4.20
    Char,  // 'a'
    Str,   // "hello world"

    // Mutable binding
    Mut,
    Use,

    // Control Flow
    If,
    Else,
    While,
    For,
    Loop,
    Match,
    Break,
    Continue,
    Spawn,

    // Type-related
    Type,
    Impl,
    Trait,
    Where,
    As,
    Derive,

    // Function-related
    Function,
    Return,
    Arrow, // ->

    // Punctuation
    Ampersand,    // &
    Pipe,         // |
    Caret,        // ^
    Tilde,        // ~
    LeftShift,    // <<
    RightShift,   // >>
    Colon,        // :
    DoubleColon,  // ::
    Dot,          // .
    DoubleArrow,   // =>
    At,           // @
    Question,     // ?
    Semicolon,    // ;
    OpenParen,    // (
    CloseParen,   // )
    OpenBracket,  // [
    CloseBracket, // ]
    OpenBrace,    // {
    CloseBrace,   // }
    Comma,        // ,
    Underscore,   // _

    // Boolean Operators
    And,          // &&
    Or,           // ||
    Not,          // !
    Equal,        // ==
    NotEqual,     // !=
    LessThan,     // <
    GreaterThan,  // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // Mathematical Operators
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %
    Power,    // **

    // Assignment Operators
    Assign,           // =
    PlusAssign,       // +=
    MinusAssign,      // -=
    MultiplyAssign,   // *=
    DivideAssign,     // /=
    ModuloAssign,     // %=
    PowerAssign,      // **=
    BitAndAssign,     // &=
    BitOrAssign,      // |=
    BitXorAssign,     // ^=
    LeftShiftAssign,  // <<=
    RightShiftAssign, // >>=

    // Error Handling
    QuestionMark, // ?

    // Miscellaneous
    Pub,
    Const,

    // Comments
    Comment,

    Unknown,
    UntermDoubleQuote,
}
