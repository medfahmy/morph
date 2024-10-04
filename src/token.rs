use std::fmt;
use Token::*;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // Identifiers (e.g., variable names)
    Identifier(&'a str),

    // Primitives
    Int(&'a str),
    Float(&'a str),
    Bool(bool),
    Char(char),
    Str(&'a str),

    // Keywords
    Let,
    Mut,
    Use,

    // Control Flow
    If,
    Else,
    ElseIf,
    While,
    For,
    Loop,
    Match,
    Break,
    Continue,

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

    // Bitwise Operators
    BitAnd,     // &
    BitOr,      // |
    BitXor,     // ^
    BitNot,     // ~
    LeftShift,  // <<
    RightShift, // >>

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

    // Type-related
    Type,   // type
    Struct, // struct
    Enum,   // enum
    Impl,   // impl
    Trait,  // trait
    Where,  // where
    As,     // as (casting)

    // Function-related
    Function,
    Return,
    ClosureStart, // |
    Arrow,        // ->

    // Punctuation
    Colon,        // :
    DoubleColon,  // ::
    Dot,          // .
    ArrowRight,   // =>
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

    // Pattern Matching
    Pipe,       // |
    Underscore, // _

    // Error Handling
    QuestionMark, // ?

    // Miscellaneous
    Pub,
    Const,
    Static,

    // Comments
    Comment(&'a str),

    Unknown,
    Eof,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            Identifier(val) => return write!(f, "{}", val),
            Int(val) => return write!(f, "{}", val),
            Float(val) => return write!(f, "{}", val),
            Bool(val) => return write!(f, "{}", val),
            Char(val) => return write!(f, "{}", val),
            Str(val) => return write!(f, "{}", val),
            Let => "let",
            Mut => "mut",
            If => "if",
            Else => "else",
            ElseIf => "else if",
            While => "while",
            For => "for",
            Loop => "loop",
            Match => "match",
            Break => "break",
            Continue => "continue",
            Return => "return",
            And => "&&",
            Or => "||",
            Not => "!",
            Equal => "==",
            NotEqual => "!=",
            LessThan => "<",
            GreaterThan => ">",
            LessEqual => "<=",
            GreaterEqual => ">=",
            Plus => "+",
            Minus => "-",
            Multiply => "*",
            Divide => "/",
            Modulo => "%",
            Power => "**",
            BitAnd => "&",
            BitOr => "|",
            BitXor => "^",
            BitNot => "~",
            LeftShift => "<<",
            RightShift => ">>",
            Assign => "=",
            PlusAssign => "+=",
            MinusAssign => "-=",
            MultiplyAssign => "*=",
            DivideAssign => "/=",
            ModuloAssign => "%=",
            BitAndAssign => "&=",
            BitOrAssign => "|=",
            BitXorAssign => "^=",
            LeftShiftAssign => "<<=",
            RightShiftAssign => ">>=",
            Type => "type",
            Struct => "struct",
            Enum => "enum",
            Impl => "impl",
            Trait => "trait",
            Where => "where",
            As => "as",
            Fn => "fn",
            ClosureStart => "|",
            Arrow => "->",
            Colon => ":",
            DoubleColon => "::",
            Dot => ".",
            ArrowRight => "=>",
            At => "@",
            Question => "?",
            Semicolon => ";",
            OpenParen => "(",
            CloseParen => ")",
            OpenBrace => "{",
            CloseBrace => "}",
            Comma => ",",
            Pipe => "|",
            Underscore => "_",
            QuestionMark => "?",
            Pub => "pub",
            Const => "const",
            Static => "static",
            Comment(s) => return write!(f, "// {}", s),
        };

        write!(f, "{}", token_str)
    }
}
