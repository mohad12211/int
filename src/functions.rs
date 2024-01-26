use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::{interpreter::Interpreter, statement::Stmt, token::Token, value::Value, IntResult};

#[derive(Clone)]
pub struct Callable {
    pub fun: Rc<dyn IntCallable>,
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.fun, &other.fun)
    }
}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fun.name())
    }
}

pub trait IntCallable {
    fn arity(&self) -> usize;
    fn name(&self) -> String;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntResult>;
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self { name, params, body }
    }
}

impl IntCallable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn name(&self) -> String {
        format!("<fn {} >", self.name.lexeme)
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, IntResult> {
        let mut values = HashMap::new();
        for (token, argument) in self.params.clone().into_iter().zip(arguments) {
            values.insert(token.lexeme, argument);
        }
        match interpreter.execute_block(self.body.clone(), &[0], values) {
            Ok(()) => Ok(Value::Nil),
            Err(IntResult::ReturnValue(value)) => Ok(value),
            Err(err @ IntResult::Error { .. }) => Err(err),
        }
    }
}
