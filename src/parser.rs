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
    pub fn parse(input: &'a str) -> Result<Program<'a>> {
        let mut parser = Self {
            lexer: Lexer::new(input),
            curr: None,
            peek: None,
        };
        parser.bump();
        parser.bump();
        let mut program = Program::default();

        while parser.curr().is_some() {
            parser.parse_statement().map(|statement| {
                program.push(statement)
            });
        }

        Ok(program)
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

    fn parse_statement(&mut self) -> Result<Statement<'a>> {
        if let Some(token) = self.curr() {
            match token.kind {
                Identifier => self.parse_declaration(),
                Return => self.parse_return_statement(),
                Loop | For | While => self.parse_loop_statement(),
                Function => self.parse_function_statement(),
                Spawn => self.parse_spawn_statement(),
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }

    fn parse_expression(&mut self) -> Result<Expression<'a>> {
        todo!()
    }

    fn parse_declaration(&mut self) -> Result<Statement<'a>> {
        match self.peek() {
            Some(Token { kind: Assign, .. }) => {
        self.bump();
        self.parse_expression().map(|expression| Statement::Expr(expression))
            },
            Some(Token { kind: Pipe, .. }) => self.parse_type_signature(),
            _ => Err(self.peek_error("Expected binding or type signature")),
        }
    }

    fn parse_type_signature(&mut self) -> Result<Statement<'a>> {
        self.bump();
        todo!()
    }

    fn parse_return_statement(&mut self) -> Result<Statement<'a>> {
        todo!()
    }

    fn parse_loop_statement(&mut self) -> Result<Statement<'a>> {
        todo!()
    }

    fn parse_function_statement(&mut self) -> Result<Statement<'a>> {
        todo!()
    }

    fn parse_spawn_statement(&mut self) -> Result<Statement<'a>> {
        todo!()
    }
}
