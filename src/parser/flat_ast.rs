use crate::List;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression<'a> {
    Unit,
    Dash,
    Int(&'a str),
    Float(&'a str),
    Bool(&'a str),
    Char(&'a str),
    Str(&'a str),
    Ident(&'a str),
    Array(&'a str),
    Map(&'a str),
    Index(ExpressionRef), // expr[expr]
    Prefix {
        operator: Operator,
        operand: ExpressionRef,
    },
    Infix {
        operator: Operator,
        left: ExpressionRef,
        right: ExpressionRef,
    },
    If {
        condition: ExpressionRef,
        consequent: ExpressionRef,
        alternative: Option<ExpressionRef>,
    },
    Match {
        matched: ExpressionRef,
        arms: ExpressionRef,
    },
    Function {
        args: ExpressionRef,
        body: ExpressionRef,
    },
    Tuple,
    Block,
    Path,
    Field,
    Closure,
    Method,
    Range,
    Macro,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ExpressionRef {
    pub index: usize,
    pub len: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct StatementRef {
    pub index: usize,
    pub len: usize,
    pub kind: StatementKind,
}

#[derive(Debug)]
pub struct Statement<'a> {
    pub kind: StatementKind,
    pub exprs: &'a [Expression<'a>],
}

#[derive(Clone, Copy, Debug)]
pub enum StatementKind {
    Expr,
    Binding,
    Return,
    For,
    Call,
    Scope,
    Spawn,
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

#[derive(Clone, Copy, Debug)]
pub struct TypeRef {
    index: usize,
    len: usize,
}

#[derive(Debug)]
pub enum Type<'a> {
    Unit,
    Bool,
    Int,
    Float,
    Char,
    String,
    Tuple(Vec<TypeRef>),
    Function(TypeRef, TypeRef),
    NewType(&'a str),
}

#[derive(Debug)]
pub struct Ast<'a> {
    exprs: List<Expression<'a>>,
    stmts: List<StatementRef>,
    types: List<Type<'a>>,
    expr_types: HashMap<ExpressionRef, TypeRef>,
}

impl<'a> Ast<'a> {
    pub fn new() -> Self {
        Self {
            stmts: List::new(),
            exprs: List::new(),
            types: List::new(),
            expr_types: HashMap::new(),
        }
    }

    pub fn add_expression(&mut self, expr: Expression<'a>) -> ExpressionRef {
        self.exprs.push(expr);
        todo!()
    }

    pub fn add_statement(&mut self, kind: StatementKind) -> StatementRef {
        // self.stmts.push(stmt);
        todo!()
    }

    pub fn add_type(&mut self, typ: Type<'a>) {
        self.types.push(typ);
    }

    pub fn get_statement(&self, index: usize) -> &StatementRef {
        &self.stmts[index]
    }

    pub fn get_expression(&self, index: usize) -> &Expression<'a> {
        &self.exprs[index]
    }

    pub fn get_type(&self, expr_ref: ExpressionRef) -> Option<TypeRef> {
        self.expr_types.get(&expr_ref).copied()
    }

    pub fn infer_types(&mut self) -> Result<(), ()> {
        todo!()
    }
}

trait ToLisp {
    fn to_lisp(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl std::fmt::Display for dyn ToLisp + '_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_lisp(f)
    }
}
