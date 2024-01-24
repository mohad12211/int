use crate::{
    expression::{Assign, Binary, Expr, Grouping, Literal, Unary, Variable},
    statement::{Block, Expression, Print, Stmt, Var},
    token::{Token, TokenKind},
    value::Value,
    Error,
};

pub struct Parser {
    tokens: Vec<Token>,
    pub statements: Vec<Stmt>,
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
        Self {
            tokens,
            statements: Vec::new(),
            current: 0,
        }
    }

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            let statement = self.declaration();
            match statement {
                Ok(statement) => self.statements.push(statement),
                Err(Error { message, token }) => {
                    self.syncronize();
                    match token {
                        Some(token) => println!(
                            "{message}\nat token: `{}` at line: {}",
                            token.lexeme, token.line
                        ),
                        None => println!("{message}"),
                    }
                }
            }
        }
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        match_token!(self, if TokenKind::Var, {
            return self.var_declaration();
        });

        return self.statement();
    }

    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenKind::Identifier, "Expected a variable name")?;

        let mut initializer = Expr::Literal {
            value: Value::Nil.into(),
        };
        match_token!(self, if TokenKind::Equal, {
            initializer = self.expression()?;
        });

        self.consume(
            TokenKind::Semicolon,
            "Expected `;` after variable declaration.",
        )?;

        return Ok(Var(name, initializer));
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        match_token!(self, if TokenKind::Print, {
            return self.print_statement();
        });
        match_token!(self, if TokenKind::LeftBrace, {
            return self.block();
        });

        self.expression_statement()
    }

    fn block(&mut self) -> Result<Stmt, Error> {
        let mut statements = Vec::new();

        while let Some(token) = self.tokens.get(self.current) {
            if token.kind == TokenKind::RightBrace || self.is_at_end() {
                break;
            }
            statements.push(self.declaration()?);
        }

        self.consume(TokenKind::RightBrace, "Expected `}` after block.")?;

        Ok(Block(statements))
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expected `;` after value.")?;
        Ok(Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expected `;` after value.")?;
        Ok(Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let left = self.equality()?;

        match_token!(self, if equals TokenKind::Equal, {
            let value = self.assignment()?;
            if let Expr::Variable { name } = left {
                return Ok(Assign(*name, value));
            } else {
                return Err(Error { message: "Invalid assignment target".into(), token: Some(equals) });
            }
        });

        return Ok(left);
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;
        match_token!(self, while operator TokenKind::BangEqual | TokenKind::EqualEqual, {
            let right = self.comparison()?;
            expr = Binary(expr, operator, right);
        });
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        match_token!(self, while operator TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual , {
            let right = self.term()?;
            expr = Binary(expr, operator, right);
        });
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        match_token!(self, while operator TokenKind::Minus | TokenKind::Plus, {
            let right = self.factor()?;
            expr = Binary(expr, operator, right);
        });
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        match_token!(self, while operator TokenKind::Slash | TokenKind::Star, {
            let right = self.unary()?;
            expr = Binary(expr, operator, right);
        });
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        match_token!(self, if operator TokenKind::Bang | TokenKind::Minus, {
            let right = self.unary()?;
            return Ok(Unary(operator, right));
        });
        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        match_token!(self, if TokenKind::False, {
            return Ok(Literal(Value::Bool(false)));
        });
        match_token!(self, if TokenKind::True, {
            return Ok(Literal(Value::Bool(true)));
        });
        match_token!(self, if TokenKind::Nil, {
            return Ok(Literal(Value::Nil));
        });
        match_token!(self, if TokenKind::String(val) | TokenKind::Number(val), {
            return Ok(Literal(val.clone()));
        });
        match_token!(self, if var TokenKind::Identifier, {
            return Ok(Variable(var));
        });
        match_token!(self, if TokenKind::LeftParen, {
            let expr = self.expression()?;
            self.consume(TokenKind::RightParen, "Unmatched delimiter: Expected `)`")?;
            return Ok(Grouping(expr));
        });

        Err(Error {
            message: "Expected Expression".into(),
            token: None,
        })
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, Error> {
        let token = self.tokens.get(self.current).unwrap().clone();
        if token.kind == kind {
            self.current += 1;
            Ok(token)
        } else {
            Err(Error {
                message: message.into(),
                token: Some(token),
            })
        }
    }

    fn is_at_end(&mut self) -> bool {
        matches!(
            self.tokens.get(self.current),
            Some(Token {
                kind: TokenKind::Eof,
                ..
            }) | None
        )
    }

    fn syncronize(&mut self) {
        self.current += 1;

        while let Some(token) = self.tokens.get(self.current) {
            if matches!(
                self.tokens.get(self.current - 1),
                Some(Token {
                    kind: TokenKind::Semicolon,
                    ..
                })
            ) {
                return;
            }

            match token.kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::For
                | TokenKind::If
                | TokenKind::Print
                | TokenKind::Return
                | TokenKind::Var
                | TokenKind::While
                | TokenKind::Eof => return,
                _ => self.current += 1,
            }
        }
    }
}
