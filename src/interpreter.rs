use std::collections::HashMap;

use crate::{
    expression::Expr,
    statement::Stmt,
    token::{Token, TokenKind},
    value::Value,
    Error, WithToken,
};

pub struct Interpreter {
    environments: Vec<HashMap<String, Value>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environments: vec![HashMap::new()],
        }
    }

    fn evalute(&mut self, expression: Expr) -> Result<Value, Error> {
        match expression {
            Expr::Unary { operator, right } => {
                let right = self.evalute(*right)?;
                match operator.kind {
                    TokenKind::Minus => {
                        let value = right.double().with_token(operator)?;
                        Ok(Value::Double(-value))
                    }
                    TokenKind::Bang => Ok(Value::Bool(!right.is_truthy())),
                    _ => unreachable!("Invalid unary operator: {operator:?}"),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evalute(*left)?;
                let right = self.evalute(*right)?;
                match operator.kind {
                    TokenKind::Minus => Ok(Value::Double(
                        left.double().with_token(&operator)?
                            - right.double().with_token(operator)?,
                    )),
                    TokenKind::Slash => Ok(Value::Double(
                        left.double().with_token(&operator)?
                            / right.double().with_token(operator)?,
                    )),
                    TokenKind::Star => Ok(Value::Double(
                        left.double().with_token(&operator)?
                            * right.double().with_token(operator)?,
                    )),
                    TokenKind::Plus => {
                        if let (Ok(left), Ok(right)) = (left.double(), right.double()) {
                            Ok(Value::Double(left + right))
                        } else if let (Ok(left), Ok(right)) = (left.str(), right.str()) {
                            Ok(Value::Str(left + &right))
                        } else {
                            Err(Error {
                                message: "Operands must be two numbers or two strings.".into(),
                                token: Some(*operator),
                            })
                        }
                    }
                    TokenKind::BangEqual => Ok(Value::Bool(left.ne(&right))),
                    TokenKind::EqualEqual => Ok(Value::Bool(left.eq(&right))),
                    TokenKind::Greater => Ok(Value::Bool(
                        left.double().with_token(&operator)?
                            > right.double().with_token(operator)?,
                    )),
                    TokenKind::GreaterEqual => Ok(Value::Bool(
                        left.double().with_token(&operator)?
                            >= right.double().with_token(operator)?,
                    )),
                    TokenKind::Less => Ok(Value::Bool(
                        left.double().with_token(&operator)?
                            < right.double().with_token(operator)?,
                    )),
                    TokenKind::LessEqual => Ok(Value::Bool(
                        left.double().with_token(&operator)?
                            <= right.double().with_token(operator)?,
                    )),
                    _ => unreachable!("Invalid binary operator: {operator:?}"),
                }
            }
            Expr::Grouping { expression } => self.evalute(*expression),
            Expr::Literal { value } => Ok(*value),
            Expr::Variable { name } => self.get(*name),
            Expr::Assign { name, expression } => {
                let value = self.evalute(*expression)?;
                self.assign(*name, value)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evalute(*left)?;
                match operator.kind {
                    TokenKind::And => {
                        if !left.is_truthy() {
                            return Ok(left);
                        }
                    }
                    TokenKind::Or => {
                        if left.is_truthy() {
                            return Ok(left);
                        }
                    }
                    _ => unreachable!("Invalid logical operator: {operator:?}"),
                }

                self.evalute(*right)
            }
        }
    }

    fn execute(&mut self, statement: Stmt) -> Result<(), Error> {
        match statement {
            Stmt::Print { expression } => {
                let value = self.evalute(*expression)?;
                println!("{value}");
                Ok(())
            }
            Stmt::Expression { expression } => self.evalute(*expression).map(|_| {}),
            Stmt::Var { name, initializer } => {
                let value = self.evalute(*initializer)?;
                self.define(*name, value);
                Ok(())
            }
            Stmt::Block { statements } => {
                self.environments.push(HashMap::new());
                self.interpret(*statements);
                self.environments.pop();
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evalute(*condition)?.is_truthy() {
                    self.execute(*then_branch)?;
                } else if let Some(else_branch) = *else_branch {
                    self.execute(else_branch)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.evalute(*condition.clone())?.is_truthy() {
                    self.execute(*body.clone())?;
                }
                Ok(())
            }
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            match self.execute(statement) {
                Ok(()) => {}
                Err(Error { message, token }) => match token {
                    Some(token) => println!(
                        "Error interpreting `{}` at line {}: {}",
                        token.lexeme, token.line, message
                    ),
                    None => println!("Error interpreting `{}`", message),
                },
            }
        }
    }

    fn get(&self, name: Token) -> Result<Value, Error> {
        for environment in self.environments.iter().rev() {
            if let Some(value) = environment.get(&name.lexeme) {
                return Ok(value.clone());
            }
        }
        Err(Error {
            message: format!("Undefined variable `{}`.", name.lexeme),
            token: Some(name),
        })
    }

    fn assign(&mut self, name: Token, value: Value) -> Result<Value, Error> {
        for environment in self.environments.iter_mut().rev() {
            if let Some(old_value) = environment.get_mut(&name.lexeme) {
                *old_value = value.clone();
                return Ok(value);
            }
        }
        Err(Error {
            message: format!("Undefined variable `{}`.", name.lexeme),
            token: Some(name),
        })
    }

    fn define(&mut self, name: Token, value: Value) {
        self.environments
            .last_mut()
            .expect("there should always be a global environment")
            .insert(name.lexeme, value);
    }
}
