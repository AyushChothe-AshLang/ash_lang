use std::fmt::Debug;

// Node
#[derive(Debug)]
pub enum Node {
    Int(IntNode),
    Double(DoubleNode),
    BinaryOpNumber(BinaryOpNumberNode),
    BinaryOpBoolean(BinaryOpBooleanNode),
}

// IntNode
#[derive(Debug)]
pub struct IntNode {
    pub value: i64,
}

// DoubleNode
#[derive(Debug)]
pub struct DoubleNode {
    pub value: f64,
}

// Arithmetic Enum
#[derive(Debug)]
pub enum Arithmetic {
    Addition,    // +
    Subtraction, // -
    Multiply,    // *
    Divide,      // /
    Power,       // ^
    Modulus,     // %
}
// Comparison Enum
#[derive(Debug)]
pub enum Comparison {
    DoubleEquals,  // ==
    NotEquals,     // !=
    LessThan,      // <
    LessThanEq,    // <=
    GreaterThan,   // >
    GreaterThanEq, // >=
}
// BinaryOpNumberNode
#[derive(Debug)]
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
#[derive(Debug)]
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
