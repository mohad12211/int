use ahash::AHashMap as HashMap;
use std::{cell::RefCell, iter::once, mem, rc::Rc};

use crate::{
    environment::Environment,
    expression::Expr,
    functions::Callable,
    native_functions::NativeClock,
    raylib::{
        BeginDrawing, CheckCollisionRecs, ClearBackground, DrawFPS, DrawRectangle,
        DrawRectangleRec, DrawText, EndDrawing, GetFrameTime, InitWindow, IsKeyDown, KeyboardKey,
        SetTargetFPS, WindowShouldClose,
    },
    statement::Stmt,
    token::TokenKind,
    value::Value,
    IntError, WithToken,
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
        globals.insert(
            "InitWindow".into(),
            Value::Fun(Callable {
                fun: Rc::new(InitWindow),
            }),
        );
        globals.insert(
            "WindowShouldClose".into(),
            Value::Fun(Callable {
                fun: Rc::new(WindowShouldClose),
            }),
        );
        globals.insert(
            "BeginDrawing".into(),
            Value::Fun(Callable {
                fun: Rc::new(BeginDrawing),
            }),
        );
        globals.insert(
            "EndDrawing".into(),
            Value::Fun(Callable {
                fun: Rc::new(EndDrawing),
            }),
        );
        globals.insert(
            "ClearBackground".into(),
            Value::Fun(Callable {
                fun: Rc::new(ClearBackground),
            }),
        );
        globals.insert(
            "DrawText".into(),
            Value::Fun(Callable {
                fun: Rc::new(DrawText),
            }),
        );
        globals.insert(
            "SetTargetFPS".into(),
            Value::Fun(Callable {
                fun: Rc::new(SetTargetFPS),
            }),
        );
        globals.insert(
            "DrawRectangle".into(),
            Value::Fun(Callable {
                fun: Rc::new(DrawRectangle),
            }),
        );
        globals.insert(
            "DrawRectangleRec".into(),
            Value::Fun(Callable {
                fun: Rc::new(DrawRectangleRec),
            }),
        );
        globals.insert(
            "GetFrameTime".into(),
            Value::Fun(Callable {
                fun: Rc::new(GetFrameTime),
            }),
        );
        globals.insert(
            "DrawFPS".into(),
            Value::Fun(Callable {
                fun: Rc::new(DrawFPS),
            }),
        );
        globals.insert(
            "IsKeyDown".into(),
            Value::Fun(Callable {
                fun: Rc::new(IsKeyDown),
            }),
        );
        globals.insert(
            "CheckCollisionRecs".into(),
            Value::Fun(Callable {
                fun: Rc::new(CheckCollisionRecs),
            }),
        );
        globals.insert(
            "KEY_S".into(),
            Value::Double(KeyboardKey::KEY_S as u32 as f64),
        );
        globals.insert(
            "KEY_W".into(),
            Value::Double(KeyboardKey::KEY_W as u32 as f64),
        );

        globals.insert(
            "KEY_UP".into(),
            Value::Double(KeyboardKey::KEY_UP as u32 as f64),
        );
        globals.insert(
            "KEY_DOWN".into(),
            Value::Double(KeyboardKey::KEY_DOWN as u32 as f64),
        );
        Self {
            environments: vec![globals],
            environment: Environment::new(vec![0]),
        }
    }
}

impl Interpreter {
    fn evalute(&mut self, expression: &Expr) -> Result<Value, IntError> {
        match expression {
            Expr::Unary { operator, right } => {
                let right = self.evalute(right)?;
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
                let left = self.evalute(left)?;
                let right = self.evalute(right)?;
                match operator.kind {
                    TokenKind::Minus => Ok(Value::Double(
                        left.double().with_token(operator)?
                            - right.double().with_token(operator)?,
                    )),
                    TokenKind::Slash => Ok(Value::Double(
                        left.double().with_token(operator)?
                            / right.double().with_token(operator)?,
                    )),
                    TokenKind::Star => Ok(Value::Double(
                        left.double().with_token(operator)?
                            * right.double().with_token(operator)?,
                    )),
                    TokenKind::Plus => match (left, right) {
                        (Value::Str(left), Value::Str(right)) => Ok(Value::Str(left + &right)),
                        (Value::Str(left), Value::Double(right)) => {
                            Ok(Value::Str(left + &right.to_string()))
                        }
                        (Value::Double(left), Value::Str(right)) => {
                            Ok(Value::Str(left.to_string() + &right))
                        }
                        (Value::Double(left), Value::Double(right)) => {
                            Ok(Value::Double(left + right))
                        }
                        _ => Err(IntError::Error {
                            message: "One of the operands must be a string and a double".into(),
                            token: Some(operator.as_ref().clone()),
                        }),
                    },
                    TokenKind::BangEqual => Ok(Value::Bool(left.ne(&right))),
                    TokenKind::EqualEqual => Ok(Value::Bool(left.eq(&right))),
                    TokenKind::Greater => Ok(Value::Bool(
                        left.double().with_token(operator)?
                            > right.double().with_token(operator)?,
                    )),
                    TokenKind::GreaterEqual => Ok(Value::Bool(
                        left.double().with_token(operator)?
                            >= right.double().with_token(operator)?,
                    )),
                    TokenKind::Less => Ok(Value::Bool(
                        left.double().with_token(operator)?
                            < right.double().with_token(operator)?,
                    )),
                    TokenKind::LessEqual => Ok(Value::Bool(
                        left.double().with_token(operator)?
                            <= right.double().with_token(operator)?,
                    )),
                    TokenKind::Comma => Ok(right),
                    _ => unreachable!("Invalid binary operator: {operator:?}"),
                }
            }
            Expr::Grouping { expression } => self.evalute(expression),
            Expr::Literal { value } => Ok(value.as_ref().clone()),
            Expr::Variable { name } => self.environment.get(name, &mut self.environments),
            Expr::Assign { name, expression } => {
                let value = self.evalute(expression)?;
                self.environment.assign(name, value, &mut self.environments)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evalute(left)?;
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

                self.evalute(right)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evalute(callee)?;
                let arguments = arguments
                    .iter()
                    .map(|arg| self.evalute(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                let fun = callee.fun().with_token(paren)?;
                if fun.fun.arity() != arguments.len() {
                    return Err(IntError::Error {
                        message: format!(
                            "Expected {} arguments, got {}",
                            arguments.len(),
                            fun.fun.arity()
                        ),
                        token: Some((paren.as_ref()).clone()),
                    });
                }
                fun.fun.call(self, arguments)
            }
            Expr::Ternary {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evalute(condition)?.is_truthy() {
                    Ok(self.evalute(then_branch)?)
                } else {
                    Ok(self.evalute(else_branch)?)
                }
            }
            Expr::Struct { fields } => {
                let mut map = HashMap::new();
                for (token, expr) in fields.as_ref() {
                    let value = self.evalute(expr)?;
                    map.insert(token.lexeme.clone(), value);
                }
                Ok(Value::Struct(Rc::new(RefCell::new(map))))
            }
            Expr::Get { target, name } => {
                let value = self.evalute(target)?;
                let Value::Struct(map) = value else {
                    return Err(IntError::Error {
                        message: "Only structs have fields.".into(),
                        token: Some((name.as_ref()).clone()),
                    });
                };
                let map = map.borrow();
                let Some(value) = map.get(&name.as_ref().lexeme) else {
                    return Err(IntError::Error {
                        message: format!("Undefined property: `{}`.", name.as_ref().lexeme),
                        token: Some((name.as_ref()).clone()),
                    });
                };
                Ok(value.clone())
            }
            Expr::Set {
                target,
                name,
                value,
            } => {
                let target = self.evalute(target)?;
                let Value::Struct(map) = target else {
                    return Err(IntError::Error {
                        message: "Only structs have fields.".into(),
                        token: Some((name.as_ref()).clone()),
                    });
                };
                let value = self.evalute(value)?;
                map.borrow_mut()
                    .insert(name.as_ref().lexeme.clone(), value.clone());
                Ok(value)
            }
            Expr::Array { elements } => {
                let mut vec = Vec::new();
                for element in elements.as_ref() {
                    let value = self.evalute(element)?;
                    vec.push(value);
                }
                Ok(Value::Array(Rc::new(RefCell::new(vec))))
            }
        }
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), IntError> {
        match statement {
            Stmt::Print { expression } => {
                let value = self.evalute(expression)?;
                println!("{value}");
                Ok(())
            }
            Stmt::Expression { expression } => self.evalute(expression).map(|_| {}),
            Stmt::Var { name, initializer } => {
                let value = self.evalute(initializer)?;
                self.environment
                    .define(name.lexeme.clone(), value, &mut self.environments);
                Ok(())
            }
            Stmt::Block { statements } => {
                self.execute_block(statements, &self.environment.ids.clone(), HashMap::new())?;
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evalute(condition)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch.as_ref() {
                    self.execute(else_branch)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.evalute(condition)?.is_truthy() {
                    match self.execute(body) {
                        Ok(()) => {}
                        Err(IntError::Break(_)) => return Ok(()),
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            Stmt::Function { fun } => {
                self.environment.define(
                    fun.name.lexeme.clone(),
                    Value::Fun(Callable {
                        fun: Rc::new(fun.as_ref().clone()),
                    }),
                    &mut self.environments,
                );
                Ok(())
            }
            Stmt::Return { keyword, value } => {
                let return_value = self.evalute(value)?;
                Err(IntError::ReturnValue(
                    return_value,
                    (keyword.as_ref()).clone(),
                ))
            }
            Stmt::Break { keyword } => Err(IntError::Break(keyword.as_ref().clone())),
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                if let Some(initializer) = initializer.as_ref() {
                    self.execute(initializer)?;
                }
                while self.evalute(condition)?.is_truthy() {
                    match self.execute(body) {
                        Ok(_) => {}
                        Err(IntError::Break(_)) => return Ok(()),
                        Err(IntError::Continue(_)) => {}
                        Err(err) => return Err(err),
                    }

                    if let Some(increment) = increment.as_ref() {
                        self.evalute(increment)?;
                    }
                }
                Ok(())
            }
            Stmt::Continue { keyword } => Err(IntError::Continue(keyword.as_ref().clone())),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for statement in statements {
            match self.execute(statement) {
                Ok(()) => {}
                Err(IntError::ReturnValue(_, keyword)) => {
                    println!(
                        "Error interpreting: Top level return is not allowed. At line: {}",
                        keyword.line
                    );
                    return;
                }
                Err(IntError::Error { message, token }) => {
                    match token {
                        Some(token) => println!(
                            "Error interpreting `{}` at line {}: {}",
                            token.lexeme, token.line, message
                        ),
                        None => println!("Error interpreting `{}`", message),
                    };
                    return;
                }
                Err(IntError::Break(keyword)) => {
                    println!(
                        "Error interpreting: break is only allowed in loops. At line: {}",
                        keyword.line
                    );
                    return;
                }
                Err(IntError::Continue(keyword)) => {
                    println!(
                        "Error interpreting: continue is only allowed in loops. At line: {}",
                        keyword.line
                    );
                    return;
                }
            }
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        enclosing_ids: &[usize],
        values: HashMap<String, Value>,
    ) -> Result<(), IntError> {
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
        for statement in statements {
            match self.execute(statement) {
                Ok(()) => {}
                Err(err) => {
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
