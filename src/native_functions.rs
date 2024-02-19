use std::time::SystemTime;

use crate::{functions::IntCallable, interpreter::Interpreter, value::Value, IntError};

pub struct NativeClock;

impl IntCallable for NativeClock {
    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> String {
        String::from("<fun clock>")
    }

    fn call(&self, _: &mut Interpreter, _: Vec<Value>) -> Result<Value, IntError> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(f) => Ok(Value::Double(f.as_millis() as f64)),
            Err(e) => Err(IntError::Error {
                message: format!("Clock native function error: {e}"),
                token: None,
            }),
        }
    }
}

pub struct ArrayLen;

impl IntCallable for ArrayLen {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun len>")
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, IntError> {
        let array = arguments[0].get_array()?;
        Ok(Value::Double(array.borrow().len() as f64))
    }
}

pub struct ArrayWithLen;
impl IntCallable for ArrayWithLen {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun Array>")
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, IntError> {
        let len = arguments[0].double()? as usize;
        Ok(Value::new_array(vec![Value::Nil; len]))
    }
}

pub struct DeepClone;
impl IntCallable for DeepClone {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun clone>")
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, IntError> {
        Ok(arguments[0].deep_clone())
    }
}

pub struct ToString;
impl IntCallable for ToString {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun str>")
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, IntError> {
        Ok(Value::new_string(format!("{value}", value = arguments[0])))
    }
}
