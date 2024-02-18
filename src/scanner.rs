use std::process::exit;

use crate::{
    token::{Token, TokenKind},
    value::Value,
};

pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
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
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '?' => self.add_token(Question),
            ':' => self.add_token(Colon),
            '!' => {
                if self.try_consume('=') {
                    self.add_token(BangEqual)
                } else {
                    self.add_token(Bang)
                }
            }
            '=' => {
                if self.try_consume('=') {
                    self.add_token(EqualEqual)
                } else {
                    self.add_token(Equal)
                }
            }
            '<' => {
                if self.try_consume('=') {
                    self.add_token(LessEqual)
                } else {
                    self.add_token(Less)
                }
            }
            '>' => {
                if self.try_consume('=') {
                    self.add_token(GreaterEqual)
                } else {
                    self.add_token(Greater)
                }
            }
            '/' => {
                if self.try_consume('/') {
                    while self.peek().is_some_and(|c| c != '\n') {
                        self.consume();
                    }
                } else if self.try_consume('*') {
                    while self.peek().is_some_and(|c| c != '*')
                        || self.peek_next().is_some_and(|c| c != '/')
                    {
                        if self.peek() == Some('\n') {
                            self.line += 1;
                        }
                        self.consume();
                    }

                    if !self.try_consume('*') || !self.try_consume('/') {
                        println!("Unterminated block comment at line {}.", self.line);
                        exit(1);
                    }
                } else {
                    self.add_token(Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.consume_string_literal(),
            c if c.is_ascii_digit() => self.consume_number_literal(),
            c if c.is_alphabetic() => self.consume_identifer(),
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

        let value = self.source[(self.start + 2)..self.current]
            .iter()
            .collect::<String>();
        // TODO: this expect might crash on very large values
        let value =
            f64::from(u32::from_str_radix(&value, 16).expect("Should be valid hexadecimal"));
        self.add_token(TokenKind::Number(Value::Double(value)))
    }

    fn consume_identifer(&mut self) {
        while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
            self.consume();
        }
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.add_token(Self::get_keyword(&text).unwrap_or(TokenKind::Identifier))
    }

    fn consume_number_literal(&mut self) {
        if self.peek().is_some_and(|c| c == 'X' || c == 'x') {
            self.consume();
            self.consume_hex_literal();
            return;
        }
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.consume();
        }

        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            // consume the `.`
            self.consume();

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.consume();
            }
        }

        let value = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse()
            .expect("Should be a valid f64");
        self.add_token(TokenKind::Number(Value::Double(value)))
    }

    fn consume_string_literal(&mut self) {
        while self.peek().is_some_and(|c| c != '"') {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.consume();
        }

        if !self.try_consume('"') {
            // TODO: better error handling
            println!("Unterminated String at line {}", self.line);
            exit(1);
        }

        let value = self.source[(self.start + 1)..(self.current - 1)]
            .iter()
            .collect();
        self.add_token(TokenKind::String(Value::Str(value)))
    }

    fn consume(&mut self) -> char {
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn try_consume(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.consume();
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        let lexeme_text = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(kind, lexeme_text, self.line));
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
            _ => None,
        }
    }
}
