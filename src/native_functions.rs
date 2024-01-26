use std::time::SystemTime;

use crate::{functions::IntCallable, value::Value, IntResult};

pub struct NativeClock;

impl IntCallable for NativeClock {
    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> String {
        String::from("<fun clock>")
    }

    fn call(
        &self,
        _: &mut crate::interpreter::Interpreter,
        _: Vec<crate::value::Value>,
    ) -> Result<Value, IntResult> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(f) => Ok(Value::Double(f.as_millis() as f64)),
            Err(e) => Err(IntResult::Error {
                message: format!("Clock native function error: {e}"),
                token: None,
            }),
        }
    }
}
