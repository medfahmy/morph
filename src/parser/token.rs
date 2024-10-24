#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub line: u32,
    pub column: u32,
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

    // Types
    TypeDef,
    Impl,
    Trait,
    Where,
    As,
    Derive,

    // Functions
    Function,
    Return,
    Arrow, // ->

    // Symbols
    Ampersand,    // &
    Pipe,         // |
    Caret,        // ^
    Tilde,        // ~
    LeftShift,    // <<
    RightShift,   // >>
    Colon,        // :
    DoubleColon,  // ::
    Dot,          // .
    DoubleDot,    // ..
    DoubleArrow,  // =>
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

    // Bool Operators
    And,          // &&
    Or,           // ||
    Not,          // !
    Equal,        // ==
    NotEqual,     // !=
    LessThan,     // <
    GreaterThan,  // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // Math Operators
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

    // Misc
    Mut,
    Use,
    Pub,
    Const,

    // Comments
    Comment,

    Unknown,
    UntermDoubleQuote,
}
