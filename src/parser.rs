use crate::{
    expression::{
        Assign, Binary, Call, Expr, Grouping, Literal, Logical, Ternary, Unary, Variable,
    },
    functions::Function,
    statement::{Block, Break, Expression, Function, If, Print, Return, Stmt, Var, While},
    token::{Token, TokenKind},
    value::Value,
    IntResult,
};

pub struct Parser {
    tokens: Vec<Token>,
    pub statements: Vec<Stmt>,
    current: usize,
}

// like the function match_token, used on patterns that carry data like String or Double.
// the second match is to bring the token that was matched into scope
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

    pub fn parse(&mut self) -> Result<(), ()> {
        let mut result = Ok(());
        while !self.is_at_end() {
            let statement = self.declaration();
            match statement {
                Ok(statement) => self.statements.push(statement),
                Err(IntResult::Error { message, token }) => {
                    result = Err(());
                    self.syncronize();
                    match token {
                        Some(token) => println!(
                            "{message} At token: `{}` at line: {}",
                            token.lexeme, token.line
                        ),
                        None => println!("{message}"),
                    }
                }
                Err(IntResult::ReturnValue(_, _)) => {
                    unreachable!("No return statements in parsing")
                }
                Err(IntResult::Break(_)) => unreachable!("No break statements in parsing"),
            }
        }
        result
    }

    fn declaration(&mut self) -> Result<Stmt, IntResult> {
        if self.match_token(TokenKind::Fun) {
            self.function("function")
        } else if self.match_token(TokenKind::Var) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, IntResult> {
        let name = self.consume(TokenKind::Identifier, &format!("Expected {kind} name."))?;
        self.consume(
            TokenKind::LeftParen,
            &format!("Expected `(` after {kind} name."),
        )?;

        let mut parameters = Vec::new();
        if let Some(token) = self.tokens.get(self.current) {
            if token.kind != TokenKind::RightParen {
                loop {
                    // TODO: limit parameters
                    parameters
                        .push(self.consume(TokenKind::Identifier, "Expected parameter name.")?);
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }
            }
        }

        self.consume(TokenKind::RightParen, "Expected `)` after parameters.")?;

        self.consume(
            TokenKind::LeftBrace,
            &format!("Expected `{{` before {kind} body."),
        )?;

        let body = self.block()?;
        Ok(Function(Function::new(name, parameters, body)))
    }

    fn var_declaration(&mut self) -> Result<Stmt, IntResult> {
        let name = self.consume(TokenKind::Identifier, "Expected a variable name")?;

        let mut initializer = Literal(Value::Nil);
        if self.match_token(TokenKind::Equal) {
            initializer = self.expression()?;
        }

        self.consume(
            TokenKind::Semicolon,
            "Expected `;` after variable declaration.",
        )?;

        Ok(Var(initializer, name))
    }

    fn statement(&mut self) -> Result<Stmt, IntResult> {
        if self.match_token(TokenKind::For) {
            return self.for_statement();
        }
        if self.match_token(TokenKind::If) {
            return self.if_statement();
        }
        if self.match_token(TokenKind::Print) {
            return self.print_statement();
        }
        match_token!(self, if keyword TokenKind::Return ,{
            return self.return_statement(keyword);
        });
        match_token!(self, if keyword TokenKind::Break, {
            return self.break_statement(keyword);
        });
        if self.match_token(TokenKind::While) {
            return self.while_statement();
        }
        if self.match_token(TokenKind::LeftBrace) {
            return Ok(Block(self.block()?));
        }

        self.expression_statement()
    }

    fn break_statement(&mut self, keyword: Token) -> Result<Stmt, IntResult> {
        self.consume(TokenKind::Semicolon, "Expected `;` after break.")?;
        Ok(Break(keyword))
    }

    fn return_statement(&mut self, keyword: Token) -> Result<Stmt, IntResult> {
        if self.match_token(TokenKind::Semicolon) {
            return Ok(Return(keyword, Literal(Value::Nil)));
        }
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expected `;` after return.")?;
        Ok(Return(keyword, value))
    }

    fn for_statement(&mut self) -> Result<Stmt, IntResult> {
        self.consume(TokenKind::LeftParen, "Expected `(` after 'for'.")?;
        let initializer = if self.match_token(TokenKind::Semicolon) {
            None
        } else if self.match_token(TokenKind::Var) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.match_token(TokenKind::Semicolon) {
            Literal(Value::Bool(true))
        } else {
            let condition = self.expression()?;
            self.consume(TokenKind::Semicolon, "Expected `;` after loop condition.")?;
            condition
        };

        let increment = if self.match_token(TokenKind::RightParen) {
            None
        } else {
            let increment = self.expression()?;
            self.consume(TokenKind::RightParen, "Expected `)` after for clauses.")?;
            Some(increment)
        };

        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Block(vec![body, Expression(increment)]);
        }
        body = While(condition, body);
        if let Some(initializer) = initializer {
            body = Block(vec![initializer, body]);
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, IntResult> {
        self.consume(TokenKind::LeftParen, "Expected `(` after `while`.")?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen, "Expected `(` after condition.")?;
        let body = self.statement()?;

        Ok(While(condition, body))
    }

    fn if_statement(&mut self) -> Result<Stmt, IntResult> {
        self.consume(TokenKind::LeftParen, "Expected `(` after `if`.")?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen, "Expected `)` after `if` condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch = None;
        if self.match_token(TokenKind::Else) {
            else_branch = Some(self.statement()?);
        };

        Ok(If(condition, then_branch, else_branch))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, IntResult> {
        let mut statements = Vec::new();

        while let Some(token) = self.tokens.get(self.current) {
            if token.kind == TokenKind::RightBrace || self.is_at_end() {
                break;
            }
            statements.push(self.declaration()?);
        }

        self.consume(TokenKind::RightBrace, "Expected `}` after block.")?;

        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, IntResult> {
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expected `;` after value.")?;
        Ok(Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, IntResult> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expected `;` after value.")?;
        Ok(Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, IntResult> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr, IntResult> {
        let mut expr = self.assignment()?;

        match_token!(self, while operator TokenKind::Comma, {
            let right = self.assignment()?;
            expr = Binary(expr, operator, right);
        });

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, IntResult> {
        let left = self.ternary()?;

        match_token!(self, if equals TokenKind::Equal, {
            let value = self.assignment()?;
            if let Expr::Variable { name } = left {
                return Ok(Assign(*name, value));
            } else {
                return Err(IntResult::Error { message: "Invalid assignment target".into(), token: Some(equals) });
            }
        });

        Ok(left)
    }

    fn ternary(&mut self) -> Result<Expr, IntResult> {
        let mut expr = self.or()?;
        if self.match_token(TokenKind::Question) {
            let then_branch = self.expression()?;
            self.consume(TokenKind::Colon, "Expected `:` after ternary condition")?;
            let else_branch = self.ternary()?;
            expr = Ternary(expr, then_branch, else_branch);
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, IntResult> {
        let mut expr = self.and()?;
        match_token!(self, while operator TokenKind::Or, {
            let right = self.and()?;
            expr = Logical(expr, operator, right);
        });

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, IntResult> {
        let mut expr = self.equality()?;
        match_token!(self, while operator TokenKind::And, {
            let right = self.equality()?;
            expr = Logical(expr, operator, right);
        });

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, IntResult> {
        let mut expr = self.comparison()?;
        match_token!(self, while operator TokenKind::BangEqual | TokenKind::EqualEqual, {
            let right = self.comparison()?;
            expr = Binary(expr, operator, right);
        });
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, IntResult> {
        let mut expr = self.term()?;
        match_token!(self, while operator TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual , {
            let right = self.term()?;
            expr = Binary(expr, operator, right);
        });
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, IntResult> {
        let mut expr = self.factor()?;
        match_token!(self, while operator TokenKind::Minus | TokenKind::Plus, {
            let right = self.factor()?;
            expr = Binary(expr, operator, right);
        });
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, IntResult> {
        let mut expr = self.unary()?;
        match_token!(self, while operator TokenKind::Slash | TokenKind::Star, {
            let right = self.unary()?;
            expr = Binary(expr, operator, right);
        });
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, IntResult> {
        match_token!(self, if operator TokenKind::Bang | TokenKind::Minus, {
            let right = self.unary()?;
            return Ok(Unary(operator, right));
        });
        self.call()
    }

    fn call(&mut self) -> Result<Expr, IntResult> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(TokenKind::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, IntResult> {
        let mut arguments = Vec::new();
        if let Some(token) = self.tokens.get(self.current) {
            if token.kind != TokenKind::RightParen {
                loop {
                    // TODO: add parameter limit
                    arguments.push(self.assignment()?);
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }
            }
        }

        let paren = self.consume(TokenKind::RightParen, "Expected `)` after arguments")?;

        Ok(Call(callee, paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, IntResult> {
        if self.match_token(TokenKind::False) {
            return Ok(Literal(Value::Bool(false)));
        }
        if self.match_token(TokenKind::True) {
            return Ok(Literal(Value::Bool(true)));
        }
        if self.match_token(TokenKind::Nil) {
            return Ok(Literal(Value::Nil));
        }
        match_token!(self, if TokenKind::String(val) | TokenKind::Number(val), {
            return Ok(Literal(val.clone()));
        });
        match_token!(self, if var TokenKind::Identifier, {
            return Ok(Variable(var));
        });
        if self.match_token(TokenKind::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenKind::RightParen, "Unmatched delimiter: Expected `)`")?;
            return Ok(Grouping(expr));
        }

        Err(IntResult::Error {
            message: "Expected Expression".into(),
            token: self.tokens.get(self.current).cloned(),
        })
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, IntResult> {
        let token = self.tokens.get(self.current).unwrap().clone();
        if token.kind == kind {
            self.current += 1;
            Ok(token)
        } else {
            Err(IntResult::Error {
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
    fn match_token(&mut self, kind: TokenKind) -> bool {
        if let Some(token) = self.tokens.get(self.current) {
            if token.kind == kind {
                self.current += 1;
                return true;
            }
        }

        false
    }
}
