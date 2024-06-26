use std::{fs, time::SystemTime};

use crate::{
    functions::IntCallable,
    interpreter::Interpreter,
    value::{Object, Value},
    IntError,
};

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

pub struct Len;

impl IntCallable for Len {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun len>")
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, IntError> {
        match &arguments[0] {
            Value::Object(Object::String(string)) => {
                Ok(Value::Double(string.borrow().len() as f64))
            }
            Value::Object(Object::Array(array)) => Ok(Value::Double(array.borrow().len() as f64)),
            Value::Object(Object::Struct(map)) => Ok(Value::Double(map.borrow().len() as f64)),
            _ => Err(IntError::Error {
                message: "Invalid argument to len".into(),
                token: None,
            }),
        }
        // Ok(Value::Double(array.borrow().len() as f64))
    }
}

pub struct ArrayWithLen;
impl IntCallable for ArrayWithLen {
    fn arity(&self) -> usize {
        2
    }

    fn name(&self) -> String {
        String::from("<fun array>")
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, IntError> {
        let len = arguments[0].double()? as usize;
        let value = &arguments[1];
        Ok(Value::new_array(
            (0..len).map(|_| value.deep_clone()).collect(),
        ))
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

pub struct ReadToString;
impl IntCallable for ReadToString {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun read_to_string>")
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, IntError> {
        let path = arguments[0].get_string()?;
        let data = fs::read_to_string(path.borrow().as_str());
        match data {
            Ok(data) => Ok(Value::new_string(data)),
            Err(_) => Ok(Value::Nil),
        }
    }
}

pub struct ToNum;
impl IntCallable for ToNum {
    fn arity(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("<fun num>")
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, IntError> {
        let str = arguments[0].get_string()?;
        Ok(str
            .borrow()
            .parse::<f64>()
            .map(|double| Value::Double(double))
            .unwrap_or(Value::Nil))
    }
}
