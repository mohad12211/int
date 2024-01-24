use std::process::exit;

use crate::{
    expression::{Binary, Expr, Grouping, Literal, Unary},
    token::{Token, TokenKind},
    value::Value,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! match_token {
    ($self:ident, $con:ident $kind:pat, $block:block) => {
        $con let Some(Token { kind: $kind, .. }) = $self.tokens.get($self.current) {
            $self.current += 1;
            $block
        }
    };
    ($self:ident, $con: ident $token:ident $kind:pat, $block:block) => {
        $con let Some($token @ Token {kind: $kind, .. }) = $self.tokens.get($self.current).cloned() {
            $self.current += 1;
            $block
        }
    };
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        match_token!(self, while operator TokenKind::BangEqual | TokenKind::EqualEqual, {
            let right = self.comparison();
            expr = Binary(expr, operator, right);
        });
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        match_token!(self, while operator TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual , {
            let right = self.term();
            expr = Binary(expr, operator, right);
        });
        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        match_token!(self, while operator TokenKind::Minus | TokenKind::Plus, {
            let right = self.factor();
            expr = Binary(expr, operator, right);
        });
        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        match_token!(self, while operator TokenKind::Slash | TokenKind::Star, {
            let right = self.unary();
            expr = Binary(expr, operator, right);
        });
        return expr;
    }

    fn unary(&mut self) -> Expr {
        match_token!(self, if operator TokenKind::Bang | TokenKind::Minus, {
            let right = self.unary();
            return Unary(operator, right);
        });
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        match_token!(self, if TokenKind::False, {
            return Literal(Value::Bool(false));
        });
        match_token!(self, if TokenKind::True, {
            return Literal(Value::Bool(true));
        });
        match_token!(self, if TokenKind::Nil, {
            return Literal(Value::Nil);
        });
        match_token!(self, if TokenKind::String(val) | TokenKind::Number(val), {
            return Literal(val.clone());
        });
        match_token!(self, if token TokenKind::LeftParen, {
            let expr = self.expression();
            if !self.consume(TokenKind::RightParen) {
                // TODO: better error handling
                println!("Unmatched delimiter: Expected `)` at line {}", token.line);
                exit(1);
            }
            return Grouping(expr);
        });

        // TODO: better error handling
        println!("Expected Expression");
        exit(1);
    }

    fn consume(&mut self, k: TokenKind) -> bool {
        self.current += 1;
        self.tokens
            .get(self.current - 1)
            .is_some_and(|token| token.kind.same_kind(&k))
    }
}
