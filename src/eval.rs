use crate::{Expression, Program, Statement};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Env<'a> {
    symbol_table: HashMap<&'a str, Box<Expression<'a>>>,
    outer_scope: Option<SharedEnv<'a>>,
}

pub type SharedEnv<'a> = Rc<RefCell<Env<'a>>>;

pub enum Value<'a> {
    Unit,
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Str(&'a str),
    Return(Box<Value<'a>>),
    Error(String),
    Function {
        args: Vec<&'a str>,
        body: Vec<Statement<'a>>,
        outer_scope: SharedEnv<'a>,
    },
}

pub trait Eval {
    fn eval(&self, env: &SharedEnv) -> Value;
}

impl<'a> Eval for Expression<'a> {
    fn eval(&self, _: &SharedEnv) -> Value {
        match self {
            Expression::Int(n) => Value::Int(*n),
            Expression::Float(n) => Value::Float(*n),
            Expression::Bool(b) => Value::Bool(*b),
            Expression::Char(ch) => Value::Char(*ch),
            Expression::Str(s) => Value::Str(s),
            Expression::Identifier(ident) => todo!(),
            Expression::Prefix { operator, operand } => {
                todo!()
            }
            Expression::Infix {
                operator,
                left,
                right,
            } => {
                todo!()
            }
            _ => todo!(),
        }
    }
}

impl<'a> Eval for Statement<'a> {
    fn eval(&self, env: &SharedEnv) -> Value {
        todo!()
    }
}

impl<'a> Eval for Vec<Statement<'a>> {
    fn eval(&self, env: &SharedEnv) -> Value {
        todo!()
    }
}

impl<'a> Eval for Program<'a> {
    fn eval(&self, env: &SharedEnv) -> Value {
        todo!()
    }
}
