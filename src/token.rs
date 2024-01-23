use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

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
    Super,
    This,
    True,
    Var,
    While,

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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Double(f64),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => std::fmt::Display::fmt(&s, f),
            Value::Double(d) => std::fmt::Display::fmt(&d, f),
            Value::Bool(b) => std::fmt::Display::fmt(&b, f),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl TokenKind {
    pub fn same_kind(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
