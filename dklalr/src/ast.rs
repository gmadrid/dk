pub struct ProgramNode(pub Vec<StmtNode>);

pub struct StmtsNode(pub Vec<StmtNode>);

pub enum StmtNode {
    Assign(String, CallNode),
    Call(CallNode)
}

pub enum StmtTailNode {
    Assign(CallNode),
    Call(Vec<ArgNode>),
}

pub struct CallNode(pub String, pub Vec<ArgNode>);

pub struct CallTailNode(pub Vec<ArgNode>);

pub struct ArgsNode(pub Vec<ArgNode>);

// 0 = the value to pass as the parameter
// 1 = if is_some(), then it's a named parameter
pub struct ArgNode(pub ValueNode, pub Option<String>);

pub struct ArgTailNode(pub Option<ValueNode>);

pub enum ValueNode {
    Ident(String),
    Number(i32),
    String(String),
    Bool(bool),
}

// TODO: is it better to add an Ident node?

pub struct BoolNode(pub bool);
