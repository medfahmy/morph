mod ast;
mod lexer;
mod list;
mod token;

pub use ast::*;
use lexer::*;
use list::*;
use token::*;

use TokenKind::*;

pub struct Parser {
    lexer: Lexer,
    curr: Option<Token>,
    peek: Option<Token>,
}

pub type ParseError = (&'static str, Option<Token>);
pub type Result<T> = std::result::Result<T, ParseError>;

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

        while let Some(start) = self.curr.take() {
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
            Ident => self.parse_ident(start.literal),
            Return => self.parse_return(),
            For => self.parse_for(),
            _ => todo!(),
        }
    }

    fn parse_ident(&mut self, ident: String) -> Result<Stmt> {
        match &self.peek {
            None => Ok(Stmt::Return(Expr::Ident(ident))),
            Some(Token { kind, .. }) => match kind {
                Assign => {
                    self.bump();
                    let expr = self.parse_expr()?;
                    match &self.peek {
                        Some(token) if token.kind == Semicolon => {
                            self.bump();
                            Ok(Stmt::Binding(ident, expr))
                        }
                        _ => Err(("Expected Semicolon", self.curr.take())),
                    }
                }
                Pipe => self.parse_type_signature(),
                Semicolon => {
                    self.bump();
                    Ok(Stmt::Expr(Expr::Ident(ident)))
                }
                _ => {
                    Err(("Expected binding, expression or type signature", self.peek.take()))
                }
            },
        }
    }

    fn parse_type_signature(&mut self) -> Result<Stmt> {
        todo!()
    }

    fn parse_return(&mut self) -> Result<Stmt> {
        let expr = self.parse_expr()?;

        match &self.peek {
            Some(token) if token.kind == Semicolon => {
                self.bump();
                Ok(Stmt::Return(expr))
            }
            _ => Err(("Expected Semicolon", self.curr.take())),
        }
    }

    fn parse_for(&mut self) -> Result<Stmt> {
        todo!()
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        // let expr = match &self.peek {
        //     Some(token) => match token.kind {
        //         Ident => {
        //             self.bump();
        //             Ok(Expr::Ident(self.curr.take().unwrap().literal))
        //         }
        //         _ => todo!(),
        //     }
        //     None => todo!(),
        // };

        // return expr;

        match self.parse_unary() {
            Ok(mut expr) => Ok(expr),
            Err(_) => Ok(Expr::Unit),
        }
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        if let Some(token) = &self.peek {
            self.bump();
            let literal = self.curr.take().unwrap().literal;
            match token.kind {
                Str => Ok(Expr::Str(literal)),
                Ident => Ok(Expr::Ident(literal)),
                _ => todo!()
            }
        } else {
            todo!()
        }
    }
}
