use std::fmt::Debug;

use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Question,
    Colon,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String(Value),
    Number(Value),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Break,
    Continue,
    Super,
    This,
    True,
    Var,
    While,
    Append,
    Insert,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn eof(line: usize) -> Self {
        Self {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            line,
        }
    }
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Token { kind, lexeme, line }
    }
}

impl TokenKind {
    pub fn same_kind(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
