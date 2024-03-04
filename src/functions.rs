use crate::{interpreter::Interpreter, statement::Stmt, token::Token, value::Value, IntError};
use ahash::AHashMap as HashMap;
use std::fmt::Debug;

pub trait IntCallable {
    fn arity(&self) -> usize;
    fn name(&self) -> String;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>)
        -> Result<Value, IntError>;
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl Function {
    pub fn new(name: String, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self { name, params, body }
    }
}

impl IntCallable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn name(&self) -> String {
        format!("<fn {} >", self.name)
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntError> {
        let mut values = HashMap::new();
        for (token, argument) in self.params.clone().into_iter().zip(arguments) {
            values.insert(interpreter.lexeme(&token).to_string(), argument);
        }
        match interpreter.execute_block(&self.body, &[0], values) {
            Ok(()) => Ok(Value::Nil),
            Err(IntError::ReturnValue(value, _)) => Ok(value),
            Err(err @ IntError::Error { .. }) => Err(err),
            Err(IntError::Break(keyword)) => Err(IntError::Error {
                message: "break is only allowed in loops.".into(),
                token: Some(keyword),
            }),
            Err(IntError::Continue(keyword)) => Err(IntError::Error {
                message: "continue is only allowed in loops.".into(),
                token: Some(keyword),
            }),
        }
    }
}
