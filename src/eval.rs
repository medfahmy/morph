use crate::{Expr, Program, Stmt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Env<'a> {
    symbol_table: HashMap<&'a str, Box<Expr<'a>>>,
    outer_scope: Option<Rc<RefCell<Env<'a>>>>,
}

impl<'a> Env<'a> {
    pub fn new() -> Rc<RefCell<Self>> {
        let env = Env {
            symbol_table: HashMap::new(),
            outer_scope: None,
        };

        Rc::new(RefCell::new(env))
    }
}

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
        body: Vec<Stmt<'a>>,
        outer_scope: Env<'a>,
    },
}

pub trait Eval {
    fn eval(&self, env: &Env) -> Value;
}

impl<'a> Eval for Expr<'a> {
    fn eval(&self, _: &Env) -> Value {
        match self {
            Expr::Int(n) => Value::Int(*n),
            Expr::Float(n) => Value::Float(*n),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::Char(ch) => Value::Char(*ch),
            Expr::Str(s) => Value::Str(s),
            Expr::Identifier(ident) => todo!(),
            Expr::Prefix { operator, operand } => {
                todo!()
            }
            Expr::Infix {
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

impl<'a> Eval for Stmt<'a> {
    fn eval(&self, env: &Env) -> Value {
        todo!()
    }
}

impl<'a> Eval for Vec<Stmt<'a>> {
    fn eval(&self, env: &Env) -> Value {
        todo!()
    }
}

impl<'a> Eval for Program<'a> {
    fn eval(&self, env: &Env) -> Value {
        todo!()
    }
}
