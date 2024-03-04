use ahash::AHashMap as HashMap;

use crate::value::Value;

#[derive(Debug, Default)]
pub struct Environment {
    pub ids: Vec<usize>,
}

impl Environment {
    pub fn new(ids: Vec<usize>) -> Self {
        Self { ids }
    }

    pub fn get(&self, name: &str, environments: &[HashMap<String, Value>]) -> Option<Value> {
        for &id in self.ids.iter().rev() {
            if let Some(value) = environments[id].get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn assign(
        &mut self,
        name: &str,
        value: Value,
        environments: &mut [HashMap<String, Value>],
    ) -> Option<Value> {
        for &id in self.ids.iter().rev() {
            if let Some(old_value) = environments[id].get_mut(name) {
                *old_value = value.clone();
                return Some(value);
            }
        }
        None
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
