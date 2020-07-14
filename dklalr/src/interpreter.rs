use crate::builtins;
use crate::context::Context;
use crate::parse::{ArgsNode, CallNode, ProgramNode, StmtNode};
use crate::value::Value;
use crate::Error;
use fehler::throws;

#[throws]
pub fn interpret(root: ProgramNode) {
    let mut interpreter = Interpreter::default();
    for stmt in root.0 {
        interpreter.eval_stmt(stmt)?;
    }
}

#[derive(Debug, Default)]
struct Interpreter {
    context: Context,
}

impl Interpreter {
    #[throws]
    fn eval_stmt(&mut self, stmt: StmtNode) {
        match stmt {
            StmtNode::Assign(variable, call) => {
                self.assign(variable, call)?;
            }
            StmtNode::Call(call) => {
                self.call(call)?;
            }
        }
    }

    #[throws]
    fn assign(&mut self, variable: String, call: CallNode) {
        let value = self.call(call)?;
        self.context.assign_variable(&variable, &value)?;
    }

    #[throws]
    fn call(&mut self, call: CallNode) -> Value {
        let CallNode(func, ArgsNode(args)) = call;
        builtins::call(&func, args, &self.context)?
    }
}
