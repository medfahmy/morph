mod ast;
mod errors;
mod lexer;
mod list;
mod token;

pub use ast::*;
use errors::*;
use lexer::*;
use list::*;
use token::*;

use TokenKind::*;

pub struct Parser {
    lexer: Lexer,
    curr: Option<Token>,
    peek: Option<Token>,
}

impl Parser {
    pub fn new(source: &'_ str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(source),
            curr: None,
            peek: None,
        };
        parser.bump();
        parser.bump();
        parser
    }

    pub fn parse(&mut self) -> Result<Ast> {
        let mut ast = Ast::new();

        while let Some(start) = self.curr.clone() {
            let stmt = self.parse_stmt(start)?;
            ast.push(stmt);
            self.bump();
        }

        Ok(ast)
    }

    fn bump(&mut self) {
        self.curr = self.peek.take();
        self.peek = self.lexer.next();
    }

    fn parse_stmt(&mut self, start: Token) -> Result<Stmt> {
        match start.kind {
            Ident => self.parse_ident(),
            _ => todo!(),
        }
    }

    fn parse_ident(&mut self) -> Result<Stmt> {
        let ident = self.curr.clone().unwrap().literal;

        if let Some(Token { kind, .. }) = self.peek.clone() {
            match kind {
                Assign => {
                    self.bump();
                    let expr = self.parse_expr()?;
                    Ok(Stmt::Binding(ident, expr))
                }
                Pipe => todo!(),
                _ => Err(Error(
                    "Expected binding or type signature",
                    self.peek.clone(),
                )),
            }
        } else {
            Ok(Stmt::Expr(Expr::Ident(ident)))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        if let Some(token) = self.peek.clone() {
            match token.kind {
                Ident => {
                    self.bump();
                    Ok(Expr::Ident(self.curr.clone().unwrap().literal))
                }
                _ => todo!(),
            }
        } else {
            Err(Error(
                "Expected expression after assignment",
                self.curr.clone(),
            ))
        }
    }
}
