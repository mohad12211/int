use std::collections::HashMap;

use crate::{token::Token, value::Value, IntResult};

#[derive(Debug, Default)]
pub struct Environment {
    pub ids: Vec<usize>,
}

impl Environment {
    pub fn new(ids: Vec<usize>) -> Self {
        Self { ids }
    }

    pub fn get(
        &self,
        name: &Token,
        environments: &mut [HashMap<String, Value>],
    ) -> Result<Value, IntResult> {
        for &id in self.ids.iter().rev() {
            if let Some(value) = environments[id].get(&name.lexeme) {
                return Ok(value.clone());
            }
        }

        Err(IntResult::Error {
            message: format!("Undefined variable `{}`.", name.lexeme),
            token: Some(name.clone()),
        })
    }

    pub fn assign(
        &mut self,
        name: &Token,
        value: Value,
        environments: &mut [HashMap<String, Value>],
    ) -> Result<Value, IntResult> {
        for &id in self.ids.iter().rev() {
            if let Some(old_value) = environments[id].get_mut(&name.lexeme) {
                *old_value = value.clone();
                return Ok(value);
            }
        }
        Err(IntResult::Error {
            message: format!("Undefined variable `{}`.", name.lexeme),
            token: Some(name.clone()),
        })
    }

    pub fn define(
        &mut self,
        name: String,
        value: Value,
        environments: &mut [HashMap<String, Value>],
    ) {
        environments[*self.ids.last().unwrap()].insert(name, value);
    }
}
