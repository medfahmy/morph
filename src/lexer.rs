use crate::Token::{self, *};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
    line: usize,
    row: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let next = self.curr().map(|curr| match curr {
            c if c.is_alphabetic() || c == '_' => self.read_lexeme(),
            c if c.is_numeric() => self.read_number(),
            '\'' => self.read_char(),
            '"' => self.read_str(),
            _ => self.read_symbol(),
        });

        self.bump();
        next
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            cursor: 0,
            line: 1,
            row: 1,
        }
    }

    fn curr(&mut self) -> Option<char> {
        self.input.chars().nth(self.cursor)
    }

    fn peek(&mut self) -> Option<char> {
        self.input.chars().nth(self.cursor + 1)
    }

    fn bump(&mut self) {
        if self.cursor <= self.input.len() {
            self.cursor += 1;

            if self.peek() == Some('\n') {
                self.line += 1;
                self.row = 1;
            } else {
                self.row += 1;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.curr() {
            if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn read_lexeme(&mut self) -> Token<'a> {
        let start = self.cursor;

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.bump();
            } else {
                break;
            }
        }

        let symbol = &self.input[start..self.cursor + 1];

        match symbol {
            "false" => Bool(false),
            "true" => Bool(true),
            "use" => Use,
            "let" => Let,
            "mut" => Mut,
            "if" => If,
            "else" => Else,
            "return" => Return,
            "for" => For,
            "while" => While,
            "loop" => Loop,
            "break" => Break,
            "continue" => Continue,
            "match" => Match,
            "pub" => Pub,
            "fn" => Function,
            "type" => Type,
            "struct" => Struct,
            "enum" => Enum,
            "impl" => Impl,
            "trait" => Trait,
            "where" => Where,
            "as" => As,
            "const" => Const,
            "static" => Static,
            _ => Identifier(symbol),
        }
    }

    fn read_number(&mut self) -> Token<'a> {
        let start = self.cursor;

        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                self.bump();
            } else {
                break;
            }
        }

        let number_str = &self.input[start..self.cursor + 1];

        if number_str.contains('.') {
            Float(number_str)
        } else {
            Int(number_str)
        }
    }

    fn read_char(&mut self) -> Token<'a> {
        self.bump();
        if let Some(ch) = self.curr() {
            self.bump();

            if let Some('\'') = self.curr() {
                self.bump();
                Char(ch)
            } else {
                Unknown
            }
        } else {
            Unknown
        }
    }

    fn read_str(&mut self) -> Token<'a> {
        self.bump();
        let start = self.cursor;

        while let Some(c) = self.curr() {
            if c.is_alphanumeric() || c == '_' {
                self.bump();
            } else {
                break;
            }
        }

        let symbol = &self.input[start..self.cursor];

        if let Some('"') = self.curr() {
            self.bump();
            Str(symbol)
        } else {
            Unknown
        }
    }

    fn extend_or(&mut self, peek: char, extended: Token<'a>, base: Token<'a>) -> Token<'a> {
        if let Some(peek) = self.peek() {
            self.bump();
            extended
        } else {
            base
        }
    }

    fn read_symbol(&mut self) -> Token<'a> {
        match self.curr() {
            Some(curr) => match curr {
                '+' => {
                    if let Some('=') = self.peek() {
                        self.bump();
                        PlusAssign
                    } else {
                        Plus
                    }
                }
                '=' => self.extend_or('=', Equal, Assign),
                '-' => self.extend_or('=', MinusAssign, Minus),
                '/' => self.extend_or('=', DivideAssign, Divide),
                '%' => self.extend_or('=', ModuloAssign, Modulo),
                '<' => {
                    if let Some('<') = self.peek() {
                        self.bump();
                        self.extend_or('=', LeftShiftAssign, LeftShift)
                    } else {
                        self.extend_or('=', LessEqual, LessThan)
                    }
                }
                '>' => {
                    if let Some('>') = self.peek() {
                        self.bump();
                        self.extend_or('=', RightShiftAssign, RightShift)
                    } else {
                        self.extend_or('=', GreaterEqual, GreaterThan)
                    }
                }
                '!' => self.extend_or('=', NotEqual, Not),
                '&' => {
                    if let Some('&') = self.peek() {
                        self.bump();
                        And
                    } else {
                        self.extend_or('=', BitAndAssign, BitAnd)
                    }
                }
                '|' => {
                    if let Some('|') = self.peek() {
                        self.bump();
                        Or
                    } else {
                        self.extend_or('=', BitOrAssign, BitOr)
                    }
                }
                '*' => {
                    if let Some('*') = self.peek() {
                        self.bump();
                        self.extend_or('=', PowerAssign, Power)
                    } else {
                        if let Some('=') = self.peek() {
                            self.bump();
                            MultiplyAssign
                        } else {
                            Multiply
                        }
                    }
                }
                '^' => self.extend_or('=', BitXorAssign, BitXor),
                ':' => self.extend_or(':', DoubleColon, Colon),
                '~' => BitNot,
                ';' => Semicolon,
                ',' => Comma,
                '.' => Dot,
                '(' => OpenParen,
                ')' => CloseParen,
                '{' => OpenBrace,
                '}' => CloseBrace,
                '[' => OpenBracket,
                ']' => CloseBracket,
                '@' => At,
                '?' => Question,
                _ => Unknown,
            },
            None => Unknown,
        }
    }
}
