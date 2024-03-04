use std::process::exit;

use crate::token::{Token, TokenKind};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof(self.line));
    }

    fn scan_token(&mut self) {
        use TokenKind::*;
        let char = self.consume();
        match char {
            b'[' => self.add_token(LeftBracket),
            b']' => self.add_token(RightBracket),
            b'(' => self.add_token(LeftParen),
            b')' => self.add_token(RightParen),
            b'{' => self.add_token(LeftBrace),
            b'}' => self.add_token(RightBrace),
            b',' => self.add_token(Comma),
            b'.' => self.add_token(Dot),
            b'-' => self.add_token(Minus),
            b'+' => self.add_token(Plus),
            b';' => self.add_token(Semicolon),
            b'*' => self.add_token(Star),
            b'?' => self.add_token(Question),
            b':' => self.add_token(Colon),
            b'!' => {
                if self.try_consume(b'=') {
                    self.add_token(BangEqual);
                } else {
                    self.add_token(Bang);
                }
            }
            b'=' => {
                if self.try_consume(b'=') {
                    self.add_token(EqualEqual);
                } else {
                    self.add_token(Equal);
                }
            }
            b'<' => {
                if self.try_consume(b'=') {
                    self.add_token(LessEqual);
                } else {
                    self.add_token(Less);
                }
            }
            b'>' => {
                if self.try_consume(b'=') {
                    self.add_token(GreaterEqual);
                } else {
                    self.add_token(Greater);
                }
            }
            b'/' => {
                if self.try_consume(b'/') {
                    while self.peek().is_some_and(|c| c != b'\n') {
                        self.consume();
                    }
                } else if self.try_consume(b'*') {
                    while self.peek().is_some_and(|c| c != b'*')
                        || self.peek_next().is_some_and(|c| c != b'/')
                    {
                        if self.peek() == Some(b'\n') {
                            self.line += 1;
                        }
                        self.consume();
                    }

                    if !self.try_consume(b'*') || !self.try_consume(b'/') {
                        println!("Unterminated block comment at line {}.", self.line);
                        exit(1);
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            b' ' | b'\r' | b'\t' => {}
            b'\n' => self.line += 1,
            b'"' => self.consume_string_literal(),
            c if c.is_ascii_digit() => self.consume_number_literal(),
            c if c.is_ascii_alphabetic() => self.consume_identifer(),
            _ => {
                // TODO: better error handling
                println!("Unexpected Character at line {}", self.line);
                exit(1);
            }
        };
    }

    fn consume_hex_literal(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
            self.consume();
        }

        self.add_token(TokenKind::Number);
    }

    fn consume_identifer(&mut self) {
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_')
        {
            self.consume();
        }
        let text = self.source[self.start..self.current].to_string();
        self.add_token(Self::get_keyword(&text).unwrap_or(TokenKind::Identifier));
    }

    fn consume_number_literal(&mut self) {
        if self.peek().is_some_and(|c| c == b'X' || c == b'x') {
            self.consume();
            self.consume_hex_literal();
            return;
        }
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.consume();
        }

        if self.peek() == Some(b'.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            // consume the `.`
            self.consume();

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.consume();
            }
        }

        self.add_token(TokenKind::Number);
    }

    fn consume_string_literal(&mut self) {
        while self.peek().is_some_and(|c| c != b'"') {
            if self.peek() == Some(b'\n') {
                self.line += 1;
            }
            self.consume();
        }

        if !self.try_consume(b'"') {
            // TODO: better error handling
            println!("Unterminated String at line {}", self.line);
            exit(1);
        }

        self.add_token(TokenKind::String);
    }

    fn consume(&mut self) -> u8 {
        let char = self.source.as_bytes()[self.current];
        self.current += 1;
        char
    }

    fn peek(&self) -> Option<u8> {
        self.source.as_bytes().get(self.current).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.source.as_bytes().get(self.current + 1).copied()
    }

    fn try_consume(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.consume();
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens
            .push(Token::new(kind, (self.start, self.current), self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn get_keyword(str: &str) -> Option<TokenKind> {
        use TokenKind::*;
        match str {
            "and" => Some(And),
            "class" => Some(Class),
            "else" => Some(Else),
            "false" => Some(False),
            "for" => Some(For),
            "fun" => Some(Fun),
            "if" => Some(If),
            "nil" => Some(Nil),
            "or" => Some(Or),
            "print" => Some(Print),
            "return" => Some(Return),
            "super" => Some(Super),
            "this" => Some(This),
            "true" => Some(True),
            "var" => Some(Var),
            "while" => Some(While),
            "break" => Some(Break),
            "continue" => Some(Continue),
            "append" => Some(Append),
            "insert" => Some(Insert),
            "delete" => Some(Delete),
            _ => None,
        }
    }
}
