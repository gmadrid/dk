/*
   List of builtins. Each builtin func has:
   1) a name
   2) list of args:
      a) name
      b) type
      c) optional default value. If no default, it's required.
   3) return type - probably Chart

   E.g.:

   read(filename: String) -> Chart
     name: 'read'
     args: { name: 'filename', type: String, default: None }
     returns: Chart


*/

use assure::assure;
use crate::ast::ArgNode;
use crate::context::{Context, Value};
use crate::Error;
use dklib::Chart;
use fehler::{throw,throws};
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ParamValue {
    Number(i32),
}

#[derive(Debug)]
pub enum ParamType {
    Chart,
    Number,
    String,
}

#[derive(Debug)]
pub struct ParamDesc {
    param_name: &'static str,
    param_type: ParamType,
    default: Option<ParamValue>,
}

pub struct Builtin {
    pub name: &'static str,
    pub params: Vec<ParamDesc>,
    pub func: fn(&HashMap<&str, Value>) -> std::result::Result<Value, Error>,
}

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin {:?} ({:?})", self.name, self.params)
    }
}

impl Builtin {
    // validate can panic since it doesn't depend on user inputs. Any errors are developer errors.
    fn validate(&self) {
        let mut iter = self.params.iter();
        // Look for the first param that takes a default.
        if let Some(_) = iter.find(|pd| pd.default.is_some()) {
            // If we find a default, make sure that all params after it have a default.
            if iter.any(|pd| pd.default.is_none()) {
                panic!("All params with defaults must come at end of param list.");
            }
        }

        // Ensure there are no params with duplicate names.
        let mut param_names_set: HashSet<&str> = HashSet::default();
        for param in self.params.iter() {
            if param_names_set.contains(param.param_name) {
                panic!("Duplicate param found: {}", param.param_name);
            }
            param_names_set.insert(param.param_name);
        }
    }
}

macro_rules! param {
    ($name:expr, $type:path, $default:expr) => {
        ParamDesc {
            param_name: $name,
            param_type: $type,
            default: Some($default),
        }
    };
    ($name:expr, $type:path) => {
        ParamDesc {
            param_name: $name,
            param_type: $type,
            default: None,
        }
    };
}

// builtin! macro
//
// 1) simplifies the definition of builtins,
// 2) automatically adds the new builtin to the hashmap, checking for duplicates,
// 3) validates the param list.
//
// Misconfigured builtins will panic! as they are created.
//
// Use like:
//
//     let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
//     builtin!(hsh, "name", [ !param("param_name1", ParamType::String),
//                             !param("param_name2", ParamType::Number, ParamValue::Number(2)) ];
macro_rules! builtin {
    ($hsh:expr, $name:expr, $func:expr, [ $($p:expr),* ]) => {
        // This check can panic since it doesn't depend on user inputs.
        // Any duplicates are developer errors.
        assert!(!$hsh.contains_key($name));

        let builtin = Builtin {
                name: $name,
                params: vec![ $($p),* ],
                func: $func,
            };
        builtin.validate();
        $hsh.insert(
            $name, builtin
        );
    }
}

lazy_static! {
    pub static ref BUILTINS: HashMap<&'static str, Builtin> = {
        let mut builtins = HashMap::default();

        builtin!(
            builtins,
            "pad",
            wrap_pad,
            [
                param!("chart", ParamType::Chart),
                param!("pad_size", ParamType::Number, ParamValue::Number(1))
            ]
        );

        builtin!(
            builtins,
            "read",
            wrap_read,
            [param!("filename", ParamType::String)]
        );

        builtin!(
            builtins,
            "write",
            wrap_write,
            [
                param!("chart", ParamType::Chart),
                param!("filename", ParamType::String)
            ]
        );

        builtins
    };
}

#[throws]
fn wrap_pad(param_values: &HashMap<&str, Value>) -> Value {
    // TODO: add all of the arguments.
    let chart = param_values.get("chart").unwrap().as_chart()?;
    let padded = chart.pad('.')?;

    println!("PADDED:\n{}", padded.write_to_string()?);

    Value::Chart(padded)
}

#[throws]
fn wrap_read(param_values: &HashMap<&str, Value>) -> Value {
    // TODO: unwrap
    let filename = param_values.get("filename").unwrap().as_string()?;
    println!("read({:?})", filename);

    let chart = Chart::read_from_file(filename)?;

    println!("READ: \n{}", chart.write_to_string()?);

    Value::Chart(chart)
}

#[throws]
fn wrap_write(param_values: &HashMap<&str, Value>) -> Value {
    let filename = param_values.get("filename").unwrap().as_string()?;
    let chart = param_values.get("chart").unwrap().as_chart()?;

    chart.write_to_file(&filename)?;
    println!("WROTE TO {}", filename);

    Value::NullValue
}

struct Invocation<'a> {
    context: &'a Context,
    builtin: &'a Builtin,
    param_values: HashMap<&'a str, Value>
}

impl<'a> Invocation<'a> {
    fn new(context: &'a Context, builtin: &'a Builtin) -> Invocation<'a> {
        Invocation { context, builtin, param_values: Default::default() }
    }

    #[throws]
    fn assign_positional_param(&mut self, builtin: &Builtin, i: usize, arg: &ArgNode) {
        println!("assign_positional_param: {} {:?}", i, arg.0);
        assure!(i < builtin.params.len(),
            Error::TooManyArguments(builtin.name, builtin.params.len(), i));
        let param_desc = &builtin.params[i];
        self.param_values.insert(param_desc.param_name, Value::from_arg(arg, self.context)?);
    }

    #[throws]
    fn assign_named_param(&mut self, builtin: &Builtin, arg: &ArgNode) {
        if let ArgNode(value, Some(name)) = arg {
            let param = builtin.params.iter().find(|p| p.param_name == name);
            if let Some(param_desc) = param {
                self.param_values.insert(param_desc.param_name, Value::from_arg(arg, self.context)?);
            } else {
                throw!(Error::UnknownParam(builtin.name, name.clone()));
            }
        }
    }

    #[throws]
    fn check_params(&self) {
        println!("check_params: ");
        for param_value in &self.param_values {
            println!("\t{}, {:?}", param_value.0, param_value.1);
        }
    }

    #[throws]
    fn invoke(&self) -> Value {
        self.check_params()?;

        let value = (self.builtin.func)(&self.param_values)?;

        value
    }
}

#[throws]
pub fn call(name: &str, args: Vec<ArgNode>, context: &Context) -> Value {
    println!("calling: {}", name);
    let builtin = BUILTINS.get(name);
    if let Some(builtin) = builtin {
        let mut invocation = Invocation::new(context, builtin);
        let (positional, named): (Vec<_>, Vec<_>) = args.iter().partition(|a| a.1.is_none());
        for (i, arg) in positional.iter().enumerate() {
            invocation.assign_positional_param(&builtin, i, &arg)?;
        }
        for arg in named {
            invocation.assign_named_param(&builtin, &arg)?;
        }

        invocation.invoke()?
    } else {
        throw!(Error::UnknownFunc(name.to_string()));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn foobar(_: &str) -> String {
        "FOOBAR".to_string()
    }

    #[test]
    fn test_validates() {
        // Test a single param.
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(hsh, "foo", foobar, [param!("foo", ParamType::String)]);
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();

        // Test only no-defaults.
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(
            hsh,
            "foo",
            foobar,
            [
                param!("foo", ParamType::String),
                param!("bar", ParamType::String)
            ]
        );
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();

        // Test only defaults.
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(
            hsh,
            "foo",
            foobar,
            [
                param!("foo", ParamType::String, ParamValue::Number(2)),
                param!("bar", ParamType::String, ParamValue::Number(2)),
                param!("baz", ParamType::String, ParamValue::Number(2))
            ]
        );
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();

        // Test only defaults after no-defaults.
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(
            hsh,
            "foo",
            foobar,
            [
                param!("foo", ParamType::String),
                param!("baz", ParamType::String, ParamValue::Number(2)),
                param!("bar", ParamType::String, ParamValue::Number(2))
            ]
        );
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();
    }

    #[test]
    #[should_panic]
    fn test_defaults_before_non_defaults() {
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(
            hsh,
            "foo",
            foobar,
            [
                param!("foo", ParamType::String),
                param!("bar", ParamType::String, ParamValue::Number(2)),
                param!("baz", ParamType::String)
            ]
        );
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();
    }

    #[test]
    #[should_panic]
    fn test_no_duplicate_builtins() {
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(hsh, "foo", foobar, []);
        builtin!(hsh, "foo", foobar, []);
    }

    #[test]
    #[should_panic]
    fn test_no_duplicate_params_names() {
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(
            hsh,
            "foo",
            foobar,
            [
                param!("dup", ParamType::String),
                param!("dup", ParamType::String)
            ]
        );
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();
    }

    #[test]
    fn test_bad_func_name() {
        assert!(call("UNKNOWN", vec![]).is_err());
    }
}
