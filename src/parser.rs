use crate::*;
use Token::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token<'a>>,
    peek: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(input),
            curr: None,
            peek: None,
        };

        parser.advance();
        parser.advance();

        parser
    }

    pub fn parse(&mut self) -> Program<'a> {
        let mut program = Program::new();

        while let Some(token) = &self.curr {
            let result = match token {
                 Let => self.parse_let_statement(),
                 Return => self.parse_return_statement(),
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
}
