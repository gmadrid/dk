use crate::value::Value;
use crate::Error;
use fehler::throws;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Context {
    variables: HashMap<String, Value>,
}

impl Context {
    #[throws]
    pub fn assign_variable(&mut self, name: &str, value: &Value) {
        println!("ASSIGNING VARIABLE: {} := {:?}", name, value);
        self.variables.insert(name.to_string(), value.clone());
    }

    #[throws]
    pub fn get_variable(&self, name: &str) -> &Value {
        // TODO: unwrap
        self.variables.get(name).unwrap()
    }
}
