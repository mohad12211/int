use std::{collections::HashMap, iter::once, mem, rc::Rc};

use crate::{
    environment::Environment, expression::Expr, functions::Callable, native_functions::NativeClock,
    statement::Stmt, token::TokenKind, value::Value, IntResult, WithToken,
};

pub struct Interpreter {
    environments: Vec<HashMap<String, Value>>,
    environment: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut globals = HashMap::new();
        globals.insert(
            "clock".into(),
            Value::Fun(Callable {
                fun: Rc::new(NativeClock),
            }),
        );
        Self {
            environments: vec![globals],
            environment: Environment::new(vec![0]),
        }
    }
}

impl Interpreter {
    fn evalute(&mut self, expression: Expr) -> Result<Value, IntResult> {
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
                            Err(IntResult::Error {
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
                    TokenKind::Comma => Ok(right),
                    _ => unreachable!("Invalid binary operator: {operator:?}"),
                }
            }
            Expr::Grouping { expression } => self.evalute(*expression),
            Expr::Literal { value } => Ok(*value),
            Expr::Variable { name } => self.environment.get(&name, &mut self.environments),
            Expr::Assign { name, expression } => {
                let value = self.evalute(*expression)?;
                self.environment
                    .assign(&name, value, &mut self.environments)
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
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evalute(*callee)?;
                let arguments = arguments
                    .into_iter()
                    .map(|arg| self.evalute(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                let fun = callee.fun().with_token(paren.clone())?;
                if fun.fun.arity() != arguments.len() {
                    return Err(IntResult::Error {
                        message: format!(
                            "Expected {} arguments, got {}",
                            arguments.len(),
                            fun.fun.arity()
                        ),
                        token: Some(*paren),
                    });
                }
                fun.fun.call(self, arguments)
            }
        }
    }

    fn execute(&mut self, statement: Stmt) -> Result<(), IntResult> {
        match statement {
            Stmt::Print { expression } => {
                let value = self.evalute(*expression)?;
                println!("{value}");
                Ok(())
            }
            Stmt::Expression { expression } => self.evalute(*expression).map(|_| {}),
            Stmt::Var { name, initializer } => {
                let value = self.evalute(*initializer)?;
                self.environment
                    .define(name.lexeme, value, &mut self.environments);
                Ok(())
            }
            Stmt::Block { statements } => {
                self.execute_block(*statements, &self.environment.ids.clone(), HashMap::new())?;
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
            Stmt::Function { fun } => {
                self.environment.define(
                    fun.name.lexeme.clone(),
                    Value::Fun(Callable { fun: Rc::new(*fun) }),
                    &mut self.environments,
                );
                Ok(())
            }
            Stmt::Return { keyword, value } => {
                let return_value = self.evalute(*value)?;
                Err(IntResult::ReturnValue(return_value, *keyword))
            }
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements.clone() {
            match self.execute(statement.clone()) {
                Ok(()) => {}
                Err(IntResult::ReturnValue(_, keyword)) => {
                    println!(
                        "Error interpreting: Top level return is not allowed. At line: {}",
                        keyword.line
                    );
                    return;
                }
                Err(IntResult::Error { message, token }) => {
                    match token {
                        Some(token) => println!(
                            "Error interpreting `{}` at line {}: {}",
                            token.lexeme, token.line, message
                        ),
                        None => println!("Error interpreting `{}`", message),
                    };
                }
            }
        }
    }

    pub fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        enclosing_ids: &[usize],
        values: HashMap<String, Value>,
    ) -> Result<(), IntResult> {
        self.environments.push(values);
        let mut environment = Environment::new(
            enclosing_ids
                .iter()
                .chain(once(&(self.environments.len() - 1)))
                .copied()
                .collect(),
        );
        mem::swap(&mut environment, &mut self.environment);
        let mut result = Ok(());
        for statement in statements.clone() {
            match self.execute(statement) {
                Ok(()) => {}
                Err(return_value @ IntResult::ReturnValue(_, _)) => {
                    result = Err(return_value);
                    break;
                }
                Err(err @ IntResult::Error { .. }) => {
                    result = Err(err);
                    break;
                }
            }
        }

        mem::swap(&mut environment, &mut self.environment);
        self.environments.pop();
        result
    }
}
