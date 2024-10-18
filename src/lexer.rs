use crate::{Token, TokenKind, TokenKind::*};

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
    line: usize,
    column: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let line = self.line;
        let column = self.column;

        let next = self.curr().map(|curr| match curr {
            ch if ch.is_alphabetic() || ch == '_' => self.read_word(),
            ch if ch.is_numeric() => self.read_number(),
            '\'' => self.read_char(),
            '"' => self.read_string(),
            _ => self.read_operator(),
        });

        self.bump();

        next.map(|(literal, kind)| Token {
            literal,
            kind,
            line,
            column,
        })
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    fn curr(&self) -> Option<char> {
        self.source.chars().nth(self.cursor)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.cursor + 1)
    }

    fn bump(&mut self) {
        if self.cursor < self.source.len() {
            self.cursor += 1;

            if let Some('\n') = self.peek() {
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

    fn read_word(&mut self) -> (&'a str, TokenKind) {
        let start = self.cursor;

        if let Some('_') = self.curr() {
            if self.peek().is_none() || self.peek().is_some_and(|peek| peek.is_whitespace()) {
                return (&self.source[start..start + 1], Underscore);
            }
        }

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.bump();
            } else {
                break;
            }
        }

        let word = &self.source[start..self.cursor + 1];

        let kind = match word {
            "true" | "false" => Bool,
            "use" => Use,
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
            "impl" => Impl,
            "trait" => Trait,
            "where" => Where,
            "as" => As,
            "const" => Const,
            "spawn" => Spawn,
            "derive" => Derive,
            _ => Ident,
        };

        (word, kind)
    }

    fn read_number(&mut self) -> (&'a str, TokenKind) {
        let start = self.cursor;

        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                self.bump();
            } else {
                break;
            }
        }

        let number_str = &self.source[start..self.cursor + 1];
        let kind = if number_str.contains(".") { Float } else { Int };
        (number_str, kind)
    }

    fn read_char(&mut self) -> (&'a str, TokenKind) {
        let start = self.cursor;
        self.bump();
        let ch = &self.source[self.cursor..self.cursor + 1];

        if self.curr().is_some() {
            self.bump();

            if let Some('\'') = self.curr() {
                self.bump();
                (ch, Char)
            } else {
                (&self.source[start..self.cursor + 1], Unknown)
            }
        } else {
            (&self.source[start..self.cursor + 1], Unknown)
        }
    }

    fn read_string(&mut self) -> (&'a str, TokenKind) {
        self.bump();
        let start = self.cursor;

        while let Some(ch) = self.peek() {
            self.bump();

            if ch == '\n' {
                return (&self.source[start..self.cursor + 1], UntermDoubleQuote);
            }

            if ch == '"' {
                break;
            }
        }

        if self.peek().is_none() {
            return (&self.source[start..self.cursor + 1], UntermDoubleQuote);
        }

        (&self.source[start..self.cursor], Str)
    }

    fn read_operator(&mut self) -> (&'a str, TokenKind) {
        let start = self.cursor;
        let kind = if let Some(curr) = self.curr() {
            match curr {
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
                        DoubleArrow
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
                        Ampersand
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
                        Pipe
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
                        Caret
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
                '~' => Tilde,
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
            }
        } else {
            Unknown
        };

        let literal = &self.source[start..self.cursor + 1];
        (literal, kind)
    }
}
