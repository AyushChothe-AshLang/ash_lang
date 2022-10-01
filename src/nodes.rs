use super::values::*;

// Node
pub trait Node: std::fmt::Debug {
    fn get_type(&self) -> NodeType;
    fn get_value(&self) -> Box<dyn Value<f64>>;
}

//NodeType
pub enum NodeType {
    Number,
    BinaryOpNode,
}

// NumberNode
#[derive(Debug)]
pub struct NumberNode {
    pub value: f64,
}
impl Node for NumberNode {
    fn get_type(&self) -> NodeType {
        NodeType::Number
    }

    fn get_value(&self) -> Box<dyn Value<f64>> {
        Box::new(NumberValue::new(self.value))
    }
}

// BinaryOpNode
#[derive(Debug)]
pub struct BinaryOpNode {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub op: String,
}

impl BinaryOpNode {
    fn new(left: Box<dyn Node>, right: Box<dyn Node>, op: String) -> BinaryOpNode {
        BinaryOpNode { left, right, op }
    }
    pub fn plus(left: Box<dyn Node>, right: Box<dyn Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, String::from("+"))
    }
    pub fn minus(left: Box<dyn Node>, right: Box<dyn Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, String::from("-"))
    }
    pub fn multiply(left: Box<dyn Node>, right: Box<dyn Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, String::from("*"))
    }
    pub fn divide(left: Box<dyn Node>, right: Box<dyn Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, String::from("/"))
    }
    pub fn power(left: Box<dyn Node>, right: Box<dyn Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, String::from("^"))
    }
    pub fn modulus(left: Box<dyn Node>, right: Box<dyn Node>) -> BinaryOpNode {
        BinaryOpNode::new(left, right, String::from("%"))
    }
}
impl Node for BinaryOpNode {
    fn get_type(&self) -> NodeType {
        NodeType::BinaryOpNode
    }
    fn get_value(&self) -> Box<dyn Value<f64>> {
        let op = self.op.clone();
        if op == "+" {
            Box::new(NumberValue::new(
                self.left.get_value().get_literal() + self.right.get_value().get_literal(),
            ))
        } else if op == "-" {
            Box::new(NumberValue::new(
                self.left.get_value().get_literal() - self.right.get_value().get_literal(),
            ))
        } else if op == "*" {
            Box::new(NumberValue::new(
                self.left.get_value().get_literal() * self.right.get_value().get_literal(),
            ))
        } else if op == "/" {
            Box::new(NumberValue::new(
                self.left.get_value().get_literal() / self.right.get_value().get_literal(),
            ))
        } else if op == "^" {
            Box::new(NumberValue::new(
                self.left
                    .get_value()
                    .get_literal()
                    .powf(self.right.get_value().get_literal()),
            ))
        } else if op == "%" {
            Box::new(NumberValue::new(
                self.left.get_value().get_literal() % self.right.get_value().get_literal(),
            ))
        } else {
            panic!("Invalid Binary Operator")
        }
    }
}
