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

use fehler::throws;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

// TODO: you need errors!
type Error = ();

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
    name: &'static str,
    params: Vec<ParamDesc>,
    func: fn(&str) -> String,
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

fn foobar(s: &str) -> String {
    todo!("write a wrapper func, silly: {}", s)
}

lazy_static! {
    pub static ref BUILTINS: HashMap<&'static str, Builtin> = {
        let mut builtins = HashMap::default();

        builtin!(builtins, "pad", foobar, [
            param!("chart", ParamType::Chart),
            param!("pad_size", ParamType::Number, ParamValue::Number(1))
        ]);

        builtin!(
            builtins,
            "read",
            foobar,
            [param!("filename", ParamType::String)]
        );

        builtin!(builtins, "write", foobar, [
            param!("filename", ParamType::String),
            param!("chart", ParamType::Chart)
        ]);

        builtins
    };
}

struct Context {

}

impl Context {

}

#[throws]
fn eval_builtin(builtin: &Builtin, context: Context) {
    /*

     */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validates() {
        // Test a single param.
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(hsh, "foo", foobar, [param!("foo", ParamType::String)]);
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();

        // Test only no-defaults.
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(hsh, "foo", foobar, [
            param!("foo", ParamType::String),
            param!("bar", ParamType::String)
            ]);
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();

        // Test only defaults.
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(hsh, "foo", foobar, [
        param!("foo", ParamType::String, ParamValue::Number(2)),
        param!("bar", ParamType::String, ParamValue::Number(2)),
        param!("baz", ParamType::String, ParamValue::Number(2))
        ]);
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();

        // Test only defaults after no-defaults.
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(hsh, "foo", foobar, [
        param!("foo", ParamType::String),
        param!("baz", ParamType::String, ParamValue::Number(2)),
        param!("bar", ParamType::String, ParamValue::Number(2))
        ]);
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();
    }

    #[test]
    #[should_panic]
    fn test_defaults_before_non_defaults() {
        let mut hsh: HashMap<&'static str, Builtin> = HashMap::default();
        builtin!(hsh, "foo", foobar, [
        param!("foo", ParamType::String),
        param!("bar", ParamType::String, ParamValue::Number(2)),
        param!("baz", ParamType::String)
        ]);
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
        builtin!(hsh, "foo", foobar, [
            param!("dup", ParamType::String),
            param!("dup", ParamType::String)
            ]);
        let builtin = hsh.get("foo").unwrap();
        builtin.validate();
    }
}