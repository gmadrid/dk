pub trait Checkable {
    fn check(&self) {
        // Most things have nothing to check.
        // If there is a check, just panic! for now.
        // TODO: get rid of the panic.
    }
}

impl<T> Checkable for Vec<T>
where
    T: Checkable,
{
    fn check(&self) {
        for t in self.iter() {
            t.check();
        }
    }
}

impl<T> Checkable for Option<T>
where
    T: Checkable,
{
    fn check(&self) {
        self.as_ref().map(|t| t.check());
    }
}

impl Checkable for String {
    fn check(&self) {
        // TODO: implement a sanity check on the format of the string here?  Maybe?
    }
}

pub struct ProgramNode(pub Vec<StmtNode>);
impl Checkable for ProgramNode {
    fn check(&self) {
        for node in &self.0 {
            node.check();
        }
    }
}

pub struct StmtsNode(pub Vec<StmtNode>);

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

pub enum StmtTailNode {
    Assign(CallNode),
    Call(ArgsNode),
}

pub struct CallNode(pub String, pub ArgsNode);

impl Checkable for CallNode {
    fn check(&self) {
        self.0.check();
        self.1.check();
    }
}

pub struct CallTailNode(pub ArgsNode);

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
pub struct ArgNode(pub ValueNode, pub Option<String>);

impl Checkable for ArgNode {
    fn check(&self) {
        self.0.check();
        self.1.check();
    }
}

pub struct ArgTailNode(pub Option<ValueNode>);

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

// TODO: is it better to add an Ident node?

pub struct BoolNode(pub bool);
