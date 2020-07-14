use dklib::Chart;
use crate::ast::{ArgNode, ValueNode};
use crate::builtins::Builtin;
use crate::Error;
use assure::assure;
use fehler::{throw, throws};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Chart(Chart),
    String(String),

    NullValue,
}

impl Value {
    #[throws]
    pub fn from_arg(arg: &ArgNode, context: &Context) -> Value {
        let ArgNode(value_node, _) = arg;
        match value_node {
            ValueNode::String(s) => Value::String(s.clone()),
            ValueNode::Ident(ident) => {
                println!("FROM ARG IDENT: {}", ident);
                dbg!(context.get_variable(ident)?.clone())
            }
            _ => Value::NullValue,
        }
    }

    #[throws]
    pub fn as_string(&self) -> &str {
        match self {
            Value::String(s) => s.as_str(),
            _ => throw!(Error::FoobarError(self.clone()))
        }
    }

    #[throws]
    pub fn as_chart(&self) -> &Chart {
        match self {
            Value::Chart(chart) => chart,
            _ => throw!(Error::FoobarError(self.clone())),
        }
    }
}

#[derive(Debug, Default)]
pub struct Context {
    variables: HashMap<String, Value>
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