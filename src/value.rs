use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Double(f64),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => std::fmt::Display::fmt(&s, f),
            Value::Double(d) => std::fmt::Display::fmt(&d, f),
            Value::Bool(b) => std::fmt::Display::fmt(&b, f),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Value {
    pub fn str(self) -> Result<String, String> {
        match self {
            Value::Str(str) => Ok(str),
            _ => Err("Operand must be a String".into()),
        }
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
}
