use crate::{expression::Expr, token::TokenKind, value::Value, ParsingError, WithToken};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evalute(&mut self, expression: Expr) -> Result<Value, ParsingError> {
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
                            Err(ParsingError {
                                message: "Operands must be two numbers or two strings.".into(),
                                token: *operator,
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
        }
    }

    pub fn interpret(&mut self, expression: Expr) {
        let value = self.evalute(expression);
        match value {
            Ok(value) => println!("{value}",),
            Err(ParsingError { message, token }) => println!(
                "Error interpreting `{}` at line {}: {}",
                token.lexeme, token.line, message
            ),
        }
    }
}
