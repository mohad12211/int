use ahash::AHashMap as HashMap;
use std::fmt::Debug;
use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::functions::IntCallable;

#[derive(Clone)]
pub struct Fun(pub Rc<dyn IntCallable>);
impl PartialEq for Fun {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Debug for Fun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    String(Rc<RefCell<String>>),
    Struct(Rc<RefCell<HashMap<String, Value>>>),
    Array(Rc<RefCell<Vec<Value>>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Double(f64),
    Bool(bool),
    Nil,
    Object(Object),
    Fun(Fun),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Double(d) => std::fmt::Display::fmt(&d, f),
            Value::Bool(b) => std::fmt::Display::fmt(&b, f),
            Value::Nil => write!(f, "nil"),
            // TODO: remove debug print
            Value::Object(object) => write!(f, "{:?}", object),
            Value::Fun(fun) => write!(f, "{}", fun.0.name()),
        }
    }
}

impl Value {
    pub fn new_fun(fun: impl IntCallable + 'static) -> Value {
        Value::Fun(Fun(Rc::new(fun)))
    }

    pub fn new_struct(structure: HashMap<String, Value>) -> Value {
        Value::Object(Object::Struct(Rc::new(RefCell::new(structure))))
    }

    pub fn new_string(string: String) -> Value {
        Value::Object(Object::String(Rc::new(RefCell::new(string))))
    }

    pub fn new_array(array: Vec<Value>) -> Value {
        Value::Object(Object::Array(Rc::new(RefCell::new(array))))
    }

    pub fn double(&self) -> Result<f64, String> {
        match self {
            Value::Double(value) => Ok(*value),
            _ => Err("Operand must be a number".into()),
        }
    }

    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Bool(false) | Value::Nil)
    }

    pub fn get_fun(self) -> Result<Fun, String> {
        match self {
            Value::Fun(f) => Ok(f),
            _ => Err("Operand must be a function".into()),
        }
    }

    pub fn get_array(&self) -> Result<&RefCell<Vec<Value>>, String> {
        match self {
            Value::Object(Object::Array(array)) => Ok(array),
            _ => Err("Operand must be an array".into()),
        }
    }

    pub fn get_struct(&self) -> Result<&RefCell<HashMap<String, Value>>, String> {
        match self {
            Value::Object(Object::Struct(map)) => Ok(map),
            _ => Err("Operand must be a struct".into()),
        }
    }

    pub fn get_string(&self) -> Result<&RefCell<String>, String> {
        match self {
            Value::Object(Object::String(string)) => Ok(string),
            _ => Err("Operand must be a string".into()),
        }
    }
}
