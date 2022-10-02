use std::fmt::Debug;

// Node
#[derive(Debug)]
pub enum Node {
    Int(IntNode),
    Double(DoubleNode),
    BinaryOp(BinaryOpNode),
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

// Operation Enum
#[derive(Debug)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiply,
    Divide,
    Power,
    Modulus,
}

// BinaryOpNode
#[derive(Debug)]
pub struct BinaryOpNode {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub op: Operation,
}

impl BinaryOpNode {
    fn new(left: Box<Node>, right: Box<Node>, op: Operation) -> BinaryOpNode {
        BinaryOpNode { left, right, op }
    }
    pub fn plus(left: Box<Node>, right: Box<Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, Operation::Addition)
    }
    pub fn minus(left: Box<Node>, right: Box<Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, Operation::Subtraction)
    }
    pub fn multiply(left: Box<Node>, right: Box<Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, Operation::Multiply)
    }
    pub fn divide(left: Box<Node>, right: Box<Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, Operation::Divide)
    }
    pub fn power(left: Box<Node>, right: Box<Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, Operation::Power)
    }
    pub fn modulus(left: Box<Node>, right: Box<Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, Operation::Modulus)
    }
}
