use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ExprRef(usize);

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct StmtRef(usize);

#[derive(Debug, Default)]
pub struct Ast<'a> {
    exprs: Vec<Expr<'a>>,
    stmts: Vec<Stmt>,
    parents: Vec<ExprRef>,
    levels: Vec<usize>,
}

impl<'a> Ast<'a> {
    pub fn add_expr(&mut self, expr: Expr) -> ExprRef {
        let idx = self.exprs.len();
        self.exprs.push(expr);
        ExprRef(idx)
    }

    pub fn add_stmt(&mut self, stmt: Stmt) -> StmtRef {
        let idx = self.stmts.len();
        self.stmts.push(stmt);
        StmtRef(idx)
    }

    pub fn get_expr(&self, expr_ref: ExprRef) -> Option<&'a Expr> {
        self.exprs.get(expr_ref.0)
    }

    pub fn get_stmt(&self, stmt_ref: StmtRef) -> Option<&'a Stmt> {
        self.stmts.get(stmt_ref.0)
    }
}

#[derive(Debug)]
pub enum Expr<'a> {
    Unit,
    Dash,
    Int(&'a str),
    Float(&'a str),
    Bool(&'a str),
    Char(&'a str),
    Str(&'a str),
    Ident(&'a str),
    Array(&'a str),
    Prefix {
        operator: Operator,
        operand: ExprRef,
    },
    Infix {
        operator: Operator,
        left: ExprRef,
        right: ExprRef,
    },
    If {
        condition: ExprRef,
        consequent: ExprRef,
        alternative: Option<ExprRef>,
    },
    Match {
        matched: ExprRef,
        arms: Vec<(ExprRef, Vec<StmtRef>)>,
    },
    Function {
        args: Vec<ExprRef>,
        body: Vec<StmtRef>,
    },
    Tuple,
    Block,
    Path,
    Index,
    Field,
    Closure,
    Method,
    Range,
    Macro,
}

#[derive(Debug)]
pub enum Stmt {
    Binding {
        ident: ExprRef,
        value: ExprRef,
    },
    Return {
        value: ExprRef,
    },
    Loop {
        expr: ExprRef,
        body: Vec<StmtRef>,
    },
    For {
        expr: ExprRef,
        body: Vec<StmtRef>,
    },
    While {
        expr: ExprRef,
        body: Vec<StmtRef>,
    },
    Call {
        function: ExprRef,
        args: Vec<ExprRef>,
    },
    Scope {
        body: Vec<StmtRef>,
    },
    Expr(ExprRef),
    Spawn(Vec<StmtRef>),
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

#[derive(Debug)]
pub struct TypeRef(usize);

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
    Custom(&'a str),
}

#[derive(Debug)]
pub struct TypedAst<'a> {
    ast: Ast<'a>,
    expr_types: HashMap<ExprRef, Type>,
}

impl<'a> TypedAst<'a> {
    pub fn get_type(&self, expr_ref: ExprRef) -> Option<&Type> {
        self.expr_types.get(&expr_ref)
    }

    pub fn infer_types(&mut self) -> Result<(), Vec<AstError>> {
        todo!()
    }
}
