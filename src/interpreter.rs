use super::nodes::*;
use super::values::*;

pub struct Interpreter {
    ast: Box<dyn Node>,
}

impl Interpreter {
    pub fn new(ast: Box<dyn Node>) -> Self {
        Interpreter { ast }
    }

    pub fn eval(&self) -> Box<dyn Value<f64>> {
        self.walk(&self.ast)
    }

    fn walk(&self, node: &Box<dyn Node>) -> Box<dyn Value<f64>> {
        match node.get_type() {
            NodeType::Number => node.get_value(),
            NodeType::BinaryOpNode => node.get_value(),
        }
    }
}
