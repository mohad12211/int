use ahash::AHashMap as HashMap;
use std::{iter::once, mem};

use crate::{
    environment::Environment,
    expression::Expr,
    native_functions::{ArrayWithLen, DeepClone, Len, NativeClock, ReadToString, ToNum, ToString},
    parser::Parser,
    raylib::{
        BeginDrawing, CheckCollisionCircleRec, CheckCollisionRecs, ClearBackground, DrawCircle,
        DrawFPS, DrawRectangle, DrawRectangleRec, DrawText, EndDrawing, GetFrameTime, InitWindow,
        IsKeyDown, KeyboardKey, SetTargetFPS, WindowShouldClose,
    },
    scanner::Scanner,
    statement::Stmt,
    token::{Token, TokenKind},
    value::{Object, Value},
    IntError, WithToken,
};

pub struct Interpreter {
    environments: Vec<HashMap<String, Value>>,
    environment: Environment,
    source: String,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut globals = HashMap::new();
        globals.insert("clock".into(), Value::new_fun(NativeClock));
        globals.insert("len".into(), Value::new_fun(Len));
        globals.insert("array".into(), Value::new_fun(ArrayWithLen));
        globals.insert("clone".into(), Value::new_fun(DeepClone));
        globals.insert("str".into(), Value::new_fun(ToString));
        globals.insert("num".into(), Value::new_fun(ToNum));
        globals.insert("read_to_string".into(), Value::new_fun(ReadToString));
        globals.insert("InitWindow".into(), Value::new_fun(InitWindow));
        globals.insert(
            "WindowShouldClose".into(),
            Value::new_fun(WindowShouldClose),
        );
        globals.insert("BeginDrawing".into(), Value::new_fun(BeginDrawing));
        globals.insert("EndDrawing".into(), Value::new_fun(EndDrawing));
        globals.insert("ClearBackground".into(), Value::new_fun(ClearBackground));
        globals.insert("DrawText".into(), Value::new_fun(DrawText));
        globals.insert("SetTargetFPS".into(), Value::new_fun(SetTargetFPS));
        globals.insert("DrawRectangle".into(), Value::new_fun(DrawRectangle));
        globals.insert("DrawCircle".into(), Value::new_fun(DrawCircle));
        globals.insert("DrawRectangleRec".into(), Value::new_fun(DrawRectangleRec));
        globals.insert("GetFrameTime".into(), Value::new_fun(GetFrameTime));
        globals.insert("DrawFPS".into(), Value::new_fun(DrawFPS));
        globals.insert("IsKeyDown".into(), Value::new_fun(IsKeyDown));
        globals.insert(
            "CheckCollisionRecs".into(),
            Value::new_fun(CheckCollisionRecs),
        );
        globals.insert(
            "CheckCollisionCircleRec".into(),
            Value::new_fun(CheckCollisionCircleRec),
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
            source: String::new(),
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
                        (
                            Value::Object(Object::String(left)),
                            Value::Object(Object::String(right)),
                        ) => Ok(Value::new_string(
                            left.borrow().clone() + right.borrow().as_ref(),
                        )),
                        (Value::Object(Object::String(left)), Value::Double(right)) => Ok(
                            Value::new_string(left.borrow().clone() + &right.to_string()),
                        ),
                        (Value::Double(left), Value::Object(Object::String(right))) => Ok(
                            Value::new_string(left.to_string() + right.borrow().as_ref()),
                        ),
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
            Expr::Variable { name } => {
                let lexeme = self.lexeme(&name);
                self.environment
                    .get(lexeme, &self.environments)
                    .ok_or(IntError::Error {
                        message: format!("Undefined variable `{}`.", self.lexeme(&name)),
                        token: Some(name.as_ref().clone()),
                    })
            }
            Expr::Assign { name, expression } => {
                let value = self.evalute(expression)?;
                // HACK: fucking borrow checker
                let lexeme = self.lexeme(&name).to_string();
                self.environment
                    .assign(&lexeme, value, &mut self.environments)
                    .ok_or(IntError::Error {
                        message: format!("Undefined variable `{}`.", self.lexeme(&name)),
                        token: Some(name.as_ref().clone()),
                    })
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

                let fun = callee.get_fun().with_token(paren)?;
                if fun.0.arity() != arguments.len() {
                    return Err(IntError::Error {
                        message: format!(
                            "Expected {} arguments, got {}",
                            fun.0.arity(),
                            arguments.len()
                        ),
                        token: Some((paren.as_ref()).clone()),
                    });
                }
                fun.0.call(self, arguments)
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
                    map.insert(self.lexeme(&token).to_string(), value);
                }
                Ok(Value::new_struct(map))
            }
            Expr::StructGet { target, name } => {
                let value = self.evalute(target)?;
                let map = value.get_struct().with_token(name)?;
                let map = map.borrow();
                let value = map.get(self.lexeme(name)).unwrap_or(&Value::Nil);
                Ok(value.clone())
            }
            Expr::StructSet {
                target,
                name,
                value,
            } => {
                let target = self.evalute(target)?;
                let map = target.get_struct().with_token(name)?;
                let value = self.evalute(value)?;
                map.borrow_mut()
                    .insert(self.lexeme(name).to_string(), value.clone());
                Ok(value)
            }
            Expr::Array { elements } => {
                let mut vec = Vec::new();
                for element in elements.as_ref() {
                    let value = self.evalute(element)?;
                    vec.push(value);
                }
                Ok(Value::new_array(vec))
            }
            Expr::IndexGet {
                array,
                bracket,
                index,
            } => match self.evalute(array)? {
                Value::Object(Object::String(string)) => {
                    let index = self.evalute(index)?.double().with_token(bracket)? as usize;
                    let chars: Vec<_> = string.borrow().chars().collect();
                    Ok(Value::new_string(
                        chars
                            .get(index)
                            .ok_or(IntError::Error {
                                message: format!(
                                    "index `{index}` is out of bound `{size}`",
                                    size = chars.len()
                                ),
                                token: Some(bracket.as_ref().clone()),
                            })?
                            .to_string(),
                    ))
                }
                Value::Object(Object::Struct(map)) => {
                    let key = self.evalute(index)?;
                    let key = key.get_string().with_token(bracket)?.borrow();
                    Ok(map
                        .borrow()
                        .get(key.as_str())
                        .unwrap_or(&Value::Nil)
                        .clone())
                }
                Value::Object(Object::Array(array)) => {
                    let array = array.borrow();
                    let index = self.evalute(index)?.double().with_token(bracket)? as usize;
                    match array.get(index) {
                        Some(value) => Ok(value.clone()),
                        None => Err(IntError::Error {
                            message: format!(
                                "index `{index}` is out of bound `{len}`",
                                len = array.len()
                            ),
                            token: Some(bracket.as_ref().clone()),
                        }),
                    }
                }
                _ => Err(IntError::Error {
                    message: "Index operator can only be used on arrays, structs or strings".into(),
                    token: Some(bracket.as_ref().clone()),
                }),
            },
            Expr::IndexSet {
                array,
                bracket,
                index,
                value,
            } => match self.evalute(array)? {
                Value::Object(Object::Array(array)) => {
                    let mut array = array.borrow_mut();
                    let index = self.evalute(index)?.double().with_token(bracket)? as usize;
                    let value = self.evalute(value)?;
                    let Some(old_value) = array.get_mut(index) else {
                        return Err(IntError::Error {
                            message: format!(
                                "index `{index}` is out of bound `{size}`",
                                size = array.len()
                            ),
                            token: Some(bracket.as_ref().clone()),
                        });
                    };
                    *old_value = value.clone();
                    Ok(value)
                }
                Value::Object(Object::String(string)) => {
                    let index = self.evalute(index)?.double().with_token(bracket)? as usize;
                    let value = self.evalute(value)?;
                    let str_value = value.get_string().with_token(bracket)?;
                    // FIX: will crash if out of range
                    string.borrow_mut().replace_range(
                        index..(index + str_value.borrow().len()),
                        str_value.borrow().as_str(),
                    );
                    Ok(value.clone())
                }
                Value::Object(Object::Struct(map)) => {
                    let key = self.evalute(index)?;
                    let value = self.evalute(value)?;
                    let key = key.get_string().with_token(bracket)?.borrow();
                    map.borrow_mut()
                        .insert(key.as_str().to_string(), value.clone());
                    Ok(value)
                }
                _ => Err(IntError::Error {
                    message: "Index operator can only be used on arrays, structs or strings".into(),
                    token: Some(bracket.as_ref().clone()),
                }),
            },
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
                self.environment.define(
                    self.lexeme(&name).to_string(),
                    value,
                    &mut self.environments,
                );
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
                    fun.name.clone(),
                    Value::new_fun(fun.as_ref().clone()),
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
                        Ok(()) | Err(IntError::Continue(_)) => {}
                        Err(IntError::Break(_)) => return Ok(()),
                        Err(err) => return Err(err),
                    }

                    if let Some(increment) = increment.as_ref() {
                        self.evalute(increment)?;
                    }
                }
                Ok(())
            }
            Stmt::Continue { keyword } => Err(IntError::Continue(keyword.as_ref().clone())),
            Stmt::Append {
                paren,
                array,
                expression,
            } => {
                let expression = self.evalute(expression)?;
                match self.evalute(array)? {
                    Value::Object(Object::Array(array)) => {
                        array.borrow_mut().push(expression);
                        Ok(())
                    }
                    Value::Object(Object::String(string)) => {
                        string
                            .borrow_mut()
                            .push_str(expression.get_string()?.borrow().as_str());
                        Ok(())
                    }
                    _ => Err(IntError::Error {
                        message: "Invalid argument to append".into(),
                        token: Some(paren.as_ref().clone()),
                    }),
                }
                // let array = array.get_array().with_token(paren)?;
                // array.borrow_mut().push(expression);
            }
            Stmt::Insert {
                paren,
                array,
                index,
                expression,
            } => {
                let array = self.evalute(array)?;
                let expression = self.evalute(expression)?;
                let mut vec = array.get_array().with_token(paren)?.borrow_mut();
                let index = self.evalute(index)?.double().with_token(paren)? as usize;
                if index > vec.len() {
                    return Err(IntError::Error {
                        message: format!(
                            "index `{index}` is out of bound `{size}`",
                            size = vec.len()
                        ),
                        token: Some(paren.as_ref().clone()),
                    });
                }
                vec.insert(index, expression);
                Ok(())
            }
            Stmt::Delete {
                paren,
                array,
                index,
            } => {
                let array = self.evalute(array)?;
                let mut array = array.get_array().with_token(paren)?.borrow_mut();
                let index = self.evalute(index)?.double().with_token(paren)? as usize;
                if index >= array.len() {
                    return Err(IntError::Error {
                        message: format!(
                            "index `{index}` is out of bound `{size}`",
                            size = array.len()
                        ),
                        token: Some(paren.as_ref().clone()),
                    });
                }
                array.remove(index);
                Ok(())
            }
        }
    }

    pub fn interpret(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        scanner.scan();
        let mut parser = Parser::new(scanner);
        parser.parse();
        let statements = parser.statements;
        self.source = parser.source;

        for statement in &statements {
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
                            self.lexeme(&token),
                            token.line,
                            message
                        ),
                        None => println!("Error interpreting `{message}`"),
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

    pub fn lexeme(&self, token: &Token) -> &str {
        &self.source[token.span.start..token.span.end]
    }
}
