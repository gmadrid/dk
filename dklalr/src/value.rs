use crate::context::Context;
use crate::parse::{ArgNode, ValueNode};
use crate::Error;
use dklib::Chart;
use fehler::{throw, throws};

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
            _ => throw!(Error::FoobarError(self.clone())),
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
