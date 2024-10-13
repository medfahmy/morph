use crate::{Token, TokenKind, TokenKind::*};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
    line: usize,
    column: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let kind = self.curr().map(|curr| match curr {
            ch if ch.is_alphabetic() || ch == '_' => self.read_symbol(),
            ch if ch.is_numeric() => self.read_number(),
            '\'' => self.read_char(),
            '"' => self.read_string(),
            _ => self.read_operator(),
        });

        self.bump();

        kind.map(|kind| Token {
            kind,
            line: self.line,
            column: self.column,
        })
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input
            , cursor: 0 
        , line: 1,
        column: 1}
    }

    fn curr(&self) -> Option<char> {
        self.input.chars().nth(self.cursor)
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.cursor + 1)
    }

    fn bump(&mut self) {
        if self.cursor < self.input.len() {
            self.cursor += 1;

            if let Some('\n') = self.curr() {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
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

    fn read_symbol(&mut self) -> TokenKind<'a> {
        if let Some('_') = self.curr() {
            if self.peek().is_none() || self.peek().is_some_and(|peek| peek.is_whitespace()) {
                return Underscore;
            }
        }

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
            "spawn" => Spawn,
            _ => Identifier(symbol),
        }
    }

    fn read_number(&mut self) -> TokenKind<'a> {
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

    fn read_char(&mut self) -> TokenKind<'a> {
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

    fn read_string(&mut self) -> TokenKind<'a> {
        self.bump();
        let start = self.cursor;

        while let Some(ch) = self.peek() {
            self.bump();

            if ch == '"' {
                break;
            }
        }

        let symbol = &self.input[start..self.cursor];

        Str(symbol)
    }

    fn read_operator(&mut self) -> TokenKind<'a> {
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
                '=' => {
                    if let Some('>') = self.peek() {
                        self.bump();
                        ArrowRight
                    } else if let Some('=') = self.peek() {
                        self.bump();
                        Equal
                    } else {
                        Assign
                    }
                }
                '-' => {
                    if let Some('>') = self.peek() {
                        self.bump();
                        Arrow
                    } else if let Some('=') = self.peek() {
                        self.bump();
                        MinusAssign
                    } else {
                        Minus
                    }
                }
                '/' => {
                    if let Some('=') = self.peek() {
                        self.bump();
                        DivideAssign
                    } else {
                        Divide
                    }
                }
                '%' => {
                    if let Some('=') = self.peek() {
                        self.bump();
                        ModuloAssign
                    } else {
                        Modulo
                    }
                }
                '<' => {
                    if let Some('<') = self.peek() {
                        self.bump();

                        if let Some('=') = self.peek() {
                            self.bump();
                            LeftShiftAssign
                        } else {
                            LeftShift
                        }
                    } else if let Some('=') = self.peek() {
                        self.bump();
                        LessEqual
                    } else {
                        LessThan
                    }
                }
                '>' => {
                    if let Some('>') = self.peek() {
                        self.bump();

                        if let Some('=') = self.peek() {
                            self.bump();
                            RightShiftAssign
                        } else {
                            RightShift
                        }
                    } else if let Some('=') = self.peek() {
                        self.bump();
                        GreaterEqual
                    } else {
                        GreaterThan
                    }
                }
                '!' => {
                    if let Some('=') = self.peek() {
                        self.bump();
                        NotEqual
                    } else {
                        Not
                    }
                }
                '&' => {
                    if let Some('&') = self.peek() {
                        self.bump();
                        And
                    } else if let Some('=') = self.peek() {
                        self.bump();
                        BitAndAssign
                    } else {
                        BitAnd
                    }
                }
                '|' => {
                    if let Some('|') = self.peek() {
                        self.bump();
                        Or
                    } else if let Some('=') = self.peek() {
                        self.bump();
                        BitOrAssign
                    } else {
                        BitOr
                    }
                }
                '*' => {
                    if let Some('*') = self.peek() {
                        self.bump();

                        if let Some('=') = self.peek() {
                            self.bump();
                            PowerAssign
                        } else {
                            Power
                        }
                    } else if let Some('=') = self.peek() {
                        self.bump();
                        MultiplyAssign
                    } else {
                        Multiply
                    }
                }
                '^' => {
                    if let Some('=') = self.peek() {
                        self.bump();
                        BitXorAssign
                    } else {
                        BitXor
                    }
                }
                ':' => {
                    if let Some(':') = self.peek() {
                        self.bump();
                        DoubleColon
                    } else {
                        Colon
                    }
                }
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
