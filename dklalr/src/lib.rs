use fehler::throws;
use lalrpop_util::lalrpop_mod;
use crate::context::Value;

mod ast;
mod builtins;
mod context;
mod interpreter;
mod lexer;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DK Error: {source}")]
    DkError {
        #[from]
        source: dklib::Error,
    },

    #[error("Parse Error: {source}")]
    Parse {
        #[from]
        source: lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>,
    },

    #[error("Too many arguments provided to function \"{0}\". Expected {1}, got {2}.")]
    TooManyArguments(&'static str, usize, usize),

    #[error("GET RID OF THIS TODO")]
    FoobarError(Value),

    #[error("Unknown function name: {0}")]
    UnknownFunc(String),

    #[error("Unknown param name, \"{1}\", provided for call to \"{0}\"")]
    UnknownParam(&'static str, String),
}

lalrpop_mod!(pub parser);

#[throws]
pub fn run_string(input: &str) {
    let lexer = lexer::Lexer::new(input);
    let ast = parser::ProgramParser::new().parse(lexer)?;
    interpreter::interpret(ast)?;
}

pub fn parse_str(input: &str) -> ast::ProgramNode {
    let lexer = lexer::Lexer::new(input);
    // TODO: yeah this sucks.
    parser::ProgramParser::new().parse(lexer).unwrap()
}

// //#[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::ast::*;
//     use crate::lexer::Lexer;
//
//     macro_rules! test_value_variant {
//         ($parser:expr, $variant:path, $value:expr, $expected:expr) => {
//             if let $variant(actual) = $parser.parse(lex($value)).unwrap() {
//                 assert_eq!(actual, $expected);
//             } else {
//                 panic!("Unexpected variant found")
//             }
//         };
//     }
//
//     fn lex(str: &str) -> Lexer {
//         Lexer::new(str)
//     }
//
//     #[test]
//     fn test_call() {
//         let p = parser::CallParser::new();
//
//         let call = p.parse(lex(r#"read(true)"#)).unwrap();
//
//         let call = p.parse(lex(r#"read("foobar.knit")"#)).unwrap();
//
//         let callstr = p.parse(lex(r#"read("bam")"#)).unwrap();
//     }
//
//     #[test]
//     fn test_calltail() {
//         let p = parser::CallTailParser::new();
//
//         let tail = p.parse(lex("(345, 456, 567)")).unwrap();
//
//         let mixed = p.parse(lex(r#"("foobar", 345, true)"#)).unwrap();
//     }
//
//     #[test]
//     fn test_args() {
//         let p = parser::ArgsParser::new();
//
//         let pos = p.parse(lex("345, 456, 789")).unwrap();
//         // TODO: test this.
//         let mixed = p.parse(lex("foo=true, bar=456, bam=quux")).unwrap();
//
//         let quoted = p.parse(lex(r#""quoted", 345, true"#)).unwrap();
//     }
//
//     #[test]
//     fn test_arg() {
//         let p = parser::ArgParser::new();
//
//         let pos = p.parse(lex("345")).unwrap();
//         // TODO: actually write these tests.
//
//         let named = p.parse(lex("foo=345")).unwrap();
//
//         let str = p.parse(lex(r#""quoted""#)).unwrap();
//     }
//
//     #[test]
//     fn test_arg_tail() {
//         let p = parser::ArgTailParser::new();
//
//         let n = p.parse(lex("= foo")).unwrap();
//         assert!(n.0.is_some());
//
//         let n = p.parse(lex("")).unwrap();
//         assert!(n.0.is_none());
//     }
//
//     #[test]
//     fn test_value_ident() {
//         let parser = parser::ValueParser::new();
//
//         test_value_variant!(parser, ValueNode::Ident, "anident", "anident");
//         test_value_variant!(parser, ValueNode::Ident, "anIdent", "anIdent");
//         test_value_variant!(parser, ValueNode::Ident, "_anident", "_anident");
//         test_value_variant!(parser, ValueNode::Ident, "_9ident", "_9ident");
//         test_value_variant!(parser, ValueNode::Ident, "anIdent", "anIdent");
//         test_value_variant!(parser, ValueNode::Ident, "AnIdent", "AnIdent");
//         test_value_variant!(parser, ValueNode::Ident, "IDENT", "IDENT");
//
//         assert!(parser.parse(lex("1ident")).is_err());
//     }
//
//     #[test]
//     fn test_value_string() {
//         let parser = parser::ValueParser::new();
//
//         test_value_variant!(parser, ValueNode::String, r#""foo""#, "foo");
//     }
//
//     #[test]
//     fn test_value_number() {
//         let parser = parser::ValueParser::new();
//
//         test_value_variant!(parser, ValueNode::Number, "23", 23);
//         test_value_variant!(parser, ValueNode::Number, "0", 0);
//         test_value_variant!(parser, ValueNode::Number, "-15", -15);
//     }
//
//     #[test]
//     fn test_value_bool() {
//         let parser = parser::ValueParser::new();
//
//         test_value_variant!(parser, ValueNode::Bool, "true", true);
//         test_value_variant!(parser, ValueNode::Bool, "false", false);
//
//         // These will get parsed as idents, not bools.
//         test_value_variant!(parser, ValueNode::Ident, "True", "True");
//         test_value_variant!(parser, ValueNode::Ident, "FALSE", "FALSE");
//     }
//
//     #[test]
//     fn test_bool() {
//         let parser = parser::BoolParser::new();
//
//         let BoolNode(value) = parser.parse(lex("true")).unwrap();
//         assert_eq!(value, true);
//
//         let BoolNode(value) = parser.parse(lex("false")).unwrap();
//         assert_eq!(value, false);
//
//         assert!(parser.parse(lex("True")).is_err());
//     }
// }
