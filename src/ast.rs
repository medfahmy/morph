use crate::Token;

use std::collections::HashMap;

pub struct Program<'a> {
    results: Vec<ParseResult<'a>>,
}

pub struct Error<'a> {
    message: &'a str,
    token: &'a Token<'a>,
}

pub enum ParseResult<'a> {
    Statement(Statement<'a>),
    Error(Error<'a>),
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn push(&mut self, result: ParseResult<'a>) -> Self {
        todo!()
    }
}

pub enum Statement<'a> {
    Let(&'a str),
    Return,
    Loop,
    Call,
    Expr(Expression<'a>),
    Spawn(Vec<Statement<'a>>),
}

pub enum Expression<'a> {
    Unit,
    Underscore,
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Str(&'a str),
    Identifier(&'a str),
    Struct,
    Enum,
    Tuple,
    Block,
    Path,
    Index,
    Field,
    Closure,
    Method,
    Prefix {
        operator: Operator,
        operand: Box<Expression<'a>>,
    },
    Infix {
        operator: Operator,
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
    },
    If {
        condition: Box<Expression<'a>>,
        consequent: Box<Expression<'a>>,
        alternative: Option<Box<Expression<'a>>>,
    },
    Match {
        matched: Box<Expression<'a>>,
        match_arms: HashMap<Expression<'a>, Vec<Statement<'a>>>,
    },
    Function {
        args: Vec<&'a str>,
        body: Vec<Statement<'a>>,
    },
    Range,
}

pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
}
