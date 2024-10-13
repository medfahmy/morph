use crate::*;
use TokenKind::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token<'a>>,
    peek: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn parse(input: &'a str) -> Program<'a> {
        let mut parser = Self {
            lexer: Lexer::new(input),
            curr: None,
            peek: None,
        };

        parser.advance();
        parser.advance();

        let mut program = Program::new();

        while let Some(token) = &parser.curr {
            let result = match token.kind {
                Let => parser.parse_let_statement(),
                Return => parser.parse_return_statement(),
                Loop | For | While => parser.parse_loop_statement(),
                Function => parser.parse_function_statement(),
                Spawn => parser.parse_spawn_statement(),
                _ => todo!(),
            };

            program.push(result);
        }

        program
    }

    fn advance(&mut self) {
        self.curr = self.peek.take();
        self.peek = self.lexer.next();
    }

    fn parse_let_statement(&mut self) -> ParseResult<'a> {
        todo!()
    }

    pub fn parse_return_statement(&mut self) -> ParseResult<'a> {
        todo!()
    }

    pub fn parse_loop_statement(&mut self) -> ParseResult<'a> {
        todo!()
    }

    pub fn parse_function_statement(&mut self) -> ParseResult<'a> {
        todo!()
    }

    pub fn parse_spawn_statement(&mut self) -> ParseResult<'a> {
        todo!()
    }
}
