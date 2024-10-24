use crate::parser::{Expr, Stmt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Eval {
    fn eval(&self, env: &Env) -> Value;
}

pub struct Env {
    symbol_table: HashMap<String, Box<Expr>>,
    outer_scope: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Self>> {
        let env = Env {
            symbol_table: HashMap::new(),
            outer_scope: None,
        };

        Rc::new(RefCell::new(env))
    }
}

pub enum Value {
    Unit,
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Str(String),
    Return(Box<Value>),
    Error(String),
    Function {
        args: Vec<String>,
        body: Vec<Stmt>,
        outer_scope: Env,
    },
}
