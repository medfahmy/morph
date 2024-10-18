use crate::*;
use TokenKind::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token<'a>>,
    peek: Option<Token<'a>>,
}

#[derive(Debug)]
pub struct Error<'a> {
    pub message: &'static str,
    pub token: Option<Token<'a>>,
}

type Result<'a, T> = std::result::Result<T, Error<'a>>;

impl<'a> Parser<'a> {
    pub fn parse(input: &'a str) -> Result<Ast<'a>> {
        let mut parser = Self {
            lexer: Lexer::new(input),
            curr: None,
            peek: None,
        };
        parser.bump();
        parser.bump();
        let mut ast = Ast::default();

        while parser.curr().is_some() {
            parser.parse_stmt().map(|stmt| {
                ast.add_stmt(stmt)
            });
        }

        Ok(ast)
    }

    fn bump(&mut self) {
        self.curr = self.peek.take();
        self.peek = self.lexer.next();
    }

    fn curr(&self) -> Option<&Token<'a>> {
        self.curr.as_ref()
    }

    fn peek(&self) -> Option<&Token<'a>> {
        self.peek.as_ref()
    }

    fn peek_error(&mut self, message: &'static str) -> Error<'a> {
        self.bump();
        Error {
            message,
            token: self.curr.take(),
        }
    }

    fn parse_stmt(&mut self) -> Result<StmtRef> {
        if let Some(token) = self.curr() {
            match token.kind {
                Identifier => self.parse_declaration(),
                Return => self.parse_return_stmt(),
                Loop | For | While => self.parse_loop_stmt(),
                Function => self.parse_function_stmt(),
                Spawn => self.parse_spawn_stmt(),
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }

    fn parse_expr(&mut self) -> Result<ExprRef> {
        todo!()
    }

    fn parse_declaration(&mut self) -> Result<StmtRef> {
        match self.peek() {
            Some(Token { kind: Assign, .. }) => {
        self.bump();
        self.parse_expr().map(|expr| StmtRef::Expr(expr))
            },
            Some(Token { kind: Pipe, .. }) => self.parse_type_signature(),
            _ => Err(self.peek_error("Expected binding or type signature")),
        }
    }

    fn parse_type_signature(&mut self) -> Result<StmtRef> {
        self.bump();
        todo!()
    }

    fn parse_return_stmt(&mut self) -> Result<StmtRef> {
        todo!()
    }

    fn parse_loop_stmt(&mut self) -> Result<StmtRef> {
        todo!()
    }

    fn parse_function_stmt(&mut self) -> Result<StmtRef> {
        todo!()
    }

    fn parse_spawn_stmt(&mut self) -> Result<StmtRef> {
        todo!()
    }
}
