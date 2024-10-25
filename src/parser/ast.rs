use crate::eval::{Env, Eval, Value};

#[derive(Debug)]
pub struct Ast {
    stmts: Vec<Stmt>,
    // new_types: Vec<Type>,
    // expr_types: HashMap<Expression, Type>,
}

impl Ast {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }

    pub fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    pub fn stmts(&self) -> &Vec<Stmt> {
        &self.stmts
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Binding(String, Expr),
    Return(Expr),
    For(Option<Expr>, Vec<Stmt>),
    Spawn(Vec<Stmt>),
    Select(Vec<Stmt>),
}

#[derive(Debug)]
pub enum Expr {
    Unit,
    Int(String),
    Float(String),
    Bool(String),
    Char(String),
    Str(String),
    Ident(String),
    Array(String),
    Map(String),
    Index(Box<Index>),
    Range(Box<Range>),
    Slice(Box<Slice>),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Conditional(Box<Conditional>),
    Match(Box<Match>),
    Function(Box<Function>),
    Call(Box<Call>),
    Field(Box<Field>),
    Scope(Vec<Stmt>),
}

#[derive(Debug)]
struct Index {
    target: Expr,
    index: Expr,
}

#[derive(Debug)]
struct Range {
    target: Expr,
    from: Expr,
    to: Expr,
}

#[derive(Debug)]
struct Slice {
    target: Expr,
    range: Expr,
}

#[derive(Debug)]
struct Unary {
    operator: Operator,
    operand: Expr,
}

#[derive(Debug)]
struct Binary {
    operator: Operator,
    left_operand: Expr,
    right_operand: Expr,
}

#[derive(Debug)]
struct Conditional {
    condition: Expr,
    consequent: Vec<Stmt>,
    alternative: Option<Vec<Stmt>>,
}

#[derive(Debug)]
struct Match {
    matched: Expr,
    arms: Vec<Expr>,
}

#[derive(Debug)]
struct Function {
    args: Vec<String>,
    body: Expr,
}

#[derive(Debug)]
struct Call {
    ident: String,
    args: Expr,
}

#[derive(Debug)]
struct Field {
    target: Expr,
    field: Expr,
}

#[derive(Debug)]
struct Method {
    target: Expr,
    method: Expr,
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

// #[derive(Debug)]
// pub enum Type {
//     Unit,
//     Bool,
//     Int,
//     Float,
//     Char,
//     String,
//     Tuple(Vec<Type>),
//     Function(Box<Type>, Box<Type>),
//     NewType(String),
// }

impl Eval for Expr {
    fn eval(&self, _: &Env) -> Value {
        todo!()
    }
}

impl Eval for Stmt {
    fn eval(&self, env: &Env) -> Value {
        todo!()
    }
}

impl Eval for Vec<Stmt> {
    fn eval(&self, env: &Env) -> Value {
        todo!()
    }
}

impl Eval for Ast {
    fn eval(&self, env: &Env) -> Value {
        todo!()
    }
}
