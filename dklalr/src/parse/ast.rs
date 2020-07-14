use super::checkable::Checkable;

#[derive(Debug)]
pub struct ProgramNode(pub Vec<StmtNode>);
impl Checkable for ProgramNode {
    fn check(&self) {
        for node in &self.0 {
            node.check();
        }
    }
}

#[derive(Debug)]
pub struct StmtsNode(pub Vec<StmtNode>);

#[derive(Debug)]
pub enum StmtNode {
    Assign(String, CallNode),
    Call(CallNode),
}

impl Checkable for StmtNode {
    fn check(&self) {
        match &self {
            StmtNode::Assign(s, c) => {
                s.check();
                c.check();
            }
            StmtNode::Call(c) => c.check(),
        }
    }
}

#[derive(Debug)]
pub enum StmtTailNode {
    Assign(CallNode),
    Call(ArgsNode),
}

#[derive(Debug)]
pub struct CallNode(pub String, pub ArgsNode);

impl Checkable for CallNode {
    fn check(&self) {
        self.0.check();
        self.1.check();
    }
}

#[derive(Debug)]
pub struct CallTailNode(pub ArgsNode);

#[derive(Debug)]
pub struct ArgsNode(pub Vec<ArgNode>);

impl Checkable for ArgsNode {
    fn check(&self) {
        self.0.check();

        // There are some additional checks you might want to make here.
        // 1. Required args.
        // 2. No positional args after a named arg.
    }
}

// 0 = the value to pass as the parameter
// 1 = if is_some(), then it's a named parameter
#[derive(Debug)]
pub struct ArgNode(pub ValueNode, pub Option<String>);

impl Checkable for ArgNode {
    fn check(&self) {
        self.0.check();
        self.1.check();
    }
}

#[derive(Debug)]
pub struct ArgTailNode(pub Option<ValueNode>);

#[derive(Debug)]
pub enum ValueNode {
    Ident(String),
    Number(i32),
    String(String),
    Bool(bool),
}

impl Checkable for ValueNode {
    fn check(&self) {
        // I think there is nothing extra here to check.
    }
}

#[derive(Debug)]
pub struct BoolNode(pub bool);

impl Checkable for BoolNode {
    fn check(&self) {
        // Nothing here to check on.
    }
}
