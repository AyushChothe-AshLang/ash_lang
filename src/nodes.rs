use std::fmt::Debug;

// Node
#[derive(Debug, Clone)]
pub enum Node {
    Int(IntNode),
    Double(DoubleNode),
    Identifier(IdentifierNode),
    UnaryNumber(UnaryNumberNode),
    UnaryBoolean(UnaryBooleanNode),
    BinaryOpNumber(BinaryOpNumberNode),
    BinaryOpBoolean(BinaryOpBooleanNode),
    Assignment(AssignmentNode),
    BlockStatement(BlockStatementNode),
    FunctionCall(FunctionCallNode),
    FunctionDeclaration(FunctionDeclarationNode),
}

// IntNode
#[derive(Debug, Clone, Copy)]
pub struct IntNode {
    pub value: i64,
}

// DoubleNode
#[derive(Debug, Clone, Copy)]
pub struct DoubleNode {
    pub value: f64,
}
// IdentifierNode
#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub value: String,
}

// UnaryArithmetic
#[derive(Debug, Clone, Copy)]
pub enum UnaryArithmetic {
    Plus,
    Minus,
}
// UnaryOperator
#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Not,
}
// UnaryNumberNode
#[derive(Debug, Clone)]
pub struct UnaryNumberNode {
    pub op: UnaryArithmetic,
    pub value: Box<Node>,
}

impl UnaryNumberNode {
    fn new(value: Box<Node>, op: UnaryArithmetic) -> Node {
        Node::UnaryNumber(UnaryNumberNode { value, op })
    }
    pub fn plus(value: Box<Node>) -> Node {
        UnaryNumberNode::new(value, UnaryArithmetic::Plus)
    }
    pub fn minus(value: Box<Node>) -> Node {
        UnaryNumberNode::new(value, UnaryArithmetic::Minus)
    }
}
// UnaryBooleanNode
#[derive(Debug, Clone)]
pub struct UnaryBooleanNode {
    pub op: UnaryOperator,
    pub value: Box<Node>,
}

impl UnaryBooleanNode {
    fn new(value: Box<Node>, op: UnaryOperator) -> Node {
        Node::UnaryBoolean(UnaryBooleanNode { value, op })
    }
    pub fn not(value: Box<Node>) -> Node {
        UnaryBooleanNode::new(value, UnaryOperator::Not)
    }
}

// Arithmetic Enum
#[derive(Debug, Clone, Copy)]
pub enum Arithmetic {
    Addition,    // +
    Subtraction, // -
    Multiply,    // *
    Divide,      // /
    Power,       // ^
    Modulus,     // %
}
// Comparison Enum
#[derive(Debug, Clone, Copy)]
pub enum Comparison {
    DoubleEquals,  // ==
    NotEquals,     // !=
    LessThan,      // <
    LessThanEq,    // <=
    GreaterThan,   // >
    GreaterThanEq, // >=
}
// BinaryOpNumberNode
#[derive(Debug, Clone)]
pub struct BinaryOpNumberNode {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub op: Arithmetic,
}

impl BinaryOpNumberNode {
    fn new(left: Box<Node>, right: Box<Node>, op: Arithmetic) -> Node {
        Node::BinaryOpNumber(BinaryOpNumberNode { left, right, op })
    }
    pub fn plus(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::Addition)
    }
    pub fn minus(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::Subtraction)
    }
    pub fn multiply(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::Multiply)
    }
    pub fn divide(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::Divide)
    }
    pub fn power(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::Power)
    }
    pub fn modulus(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::Modulus)
    }
}
// BinaryOpBooleanNode
#[derive(Debug, Clone)]
pub struct BinaryOpBooleanNode {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub op: Comparison,
}

impl BinaryOpBooleanNode {
    fn new(left: Box<Node>, right: Box<Node>, op: Comparison) -> Node {
        Node::BinaryOpBoolean(BinaryOpBooleanNode { left, right, op })
    }
    pub fn lt(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpBooleanNode::new(left, right, Comparison::LessThan)
    }
    pub fn lte(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpBooleanNode::new(left, right, Comparison::LessThanEq)
    }
    pub fn gt(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpBooleanNode::new(left, right, Comparison::GreaterThan)
    }
    pub fn gte(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpBooleanNode::new(left, right, Comparison::GreaterThanEq)
    }
    pub fn deq(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpBooleanNode::new(left, right, Comparison::DoubleEquals)
    }
    pub fn neq(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpBooleanNode::new(left, right, Comparison::NotEquals)
    }
}

// AssignmentNode
#[derive(Debug, Clone)]
pub struct AssignmentNode {
    pub id: String,
    pub value: Box<Node>,
}

impl AssignmentNode {
    pub fn new(id: String, value: Box<Node>) -> Node {
        Node::Assignment(AssignmentNode { id, value })
    }
}

// BlockStatementNode
#[derive(Debug, Clone)]
pub struct BlockStatementNode {
    pub value: Vec<Box<Node>>,
}

impl BlockStatementNode {
    pub fn new(value: Vec<Box<Node>>) -> Node {
        Node::BlockStatement(BlockStatementNode { value })
    }
}

// FunctionCallNode
#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    pub id: String,
    pub args: Vec<Box<Node>>,
}

impl FunctionCallNode {
    pub fn new(id: String, args: Vec<Box<Node>>) -> Node {
        Node::FunctionCall(FunctionCallNode { id, args })
    }
}

// FunctionDeclarationNode
#[derive(Debug, Clone)]
pub struct FunctionDeclarationNode {
    pub id: String,
    pub params: Vec<String>,
    pub body: Box<Node>,
}

impl FunctionDeclarationNode {
    pub fn new(id: String, params: Vec<String>, body: Box<Node>) -> Node {
        Node::FunctionDeclaration(FunctionDeclarationNode { id, params, body })
    }
}
