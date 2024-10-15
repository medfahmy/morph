use crate::Token;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

pub struct ParseError<'a> {
    pub message: &'static str,
    pub token: Option<Token<'a>>,
}

impl<'a> Program<'a> {
    pub fn push(&mut self, statement: Statement<'a>) {
        self.statements.push(statement)
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    Variable {
        identifier: &'a str,
        value: Expression<'a>,
    },
    Return {
        value: Expression<'a>,
    },
    Loop {
        expression: Expression<'a>,
        body: Vec<Statement<'a>>,
    },
    For {
        expression: Vec<Statement<'a>>,
        body: Vec<Statement<'a>>,
    },
    While {
        expression: Vec<Statement<'a>>,
        body: Vec<Statement<'a>>,
    },
    Call {
        function: &'a str,
        args: Vec<Expression<'a>>,
    },
    Scope {
        body: Vec<Statement<'a>>,
    },
    Expr(Expression<'a>),
    Spawn(Vec<Statement<'a>>),
}

#[derive(Debug)]
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

#[derive(Debug)]
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
