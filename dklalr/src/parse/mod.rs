/// A module for all of the machinery to build the AST for interpretation/evaluation.
/// Exports a single function, `parse()`, and all of the types needed to use it.
///
///
///
///

use crate::Error;
use fehler::throws;
use lalrpop_util::lalrpop_mod;

mod ast;
mod checkable;
mod lexer;

lalrpop_mod!(parser, "/parse/parser.rs");

pub use ast::{ArgNode, ArgsNode, CallNode, ProgramNode, StmtNode, ValueNode};
pub use lexer::{LexicalError, Token};
pub use parser::ProgramParser;

#[throws]
pub fn parse(input: &str) -> ProgramNode {
    let lexer = lexer::Lexer::new(input);
    parser::ProgramParser::new().parse(lexer)?
}
