use super::nodes::*;
use super::values::*;

type RetType = Node;

pub struct Interpreter {
    ast: RetType,
}

impl Interpreter {
    pub fn new(ast: RetType) -> Self {
        Interpreter { ast }
    }

    pub fn eval(&self) -> Value {
        self.walk(&self.ast)
    }

    fn walk(&self, node: &RetType) -> Value {
        match node {
            Node::Int(int_node) => self.walk_int_node(int_node),
            Node::Double(double_node) => self.walk_double_node(double_node),
            Node::BinaryOp(bin_op_node) => self.walk_binary_op_node(bin_op_node),
        }
    }

    fn walk_int_node(&self, node: &IntNode) -> Value {
        Value::IntValue(node.value)
    }

    fn walk_double_node(&self, node: &DoubleNode) -> Value {
        Value::DoubleValue(node.value)
    }
    fn walk_binary_op_node(&self, node: &BinaryOpNode) -> Value {
        let left = self.walk(&node.left);
        let right = self.walk(&node.right);

        match node.op {
            Operation::Addition => self.perform_op(left, right, |res, x| res + x),
            Operation::Subtraction => self.perform_op(left, right, |res, x| res - x),
            Operation::Multiply => self.perform_op(left, right, |res, x| res * x),
            Operation::Divide => self.perform_op(left, right, |res, x| res / x),
            Operation::Power => self.perform_op(left, right, |res, x| res.powf(x)),
            Operation::Modulus => self.perform_op(left, right, |res, x| res % x),
        }
    }

    fn perform_op(&self, left: Value, right: Value, op: fn(f64, f64) -> f64) -> Value {
        let mut res;
        // i+i
        let is_left_int = match left {
            Value::IntValue(i) => {
                res = i as f64;
                true
            }
            Value::DoubleValue(d) => {
                res = d;
                false
            }
            _ => panic!("Invalid Type"),
        };
        let is_right_int = match right {
            Value::IntValue(i) => {
                res = op(res, i as f64);
                true
            }
            Value::DoubleValue(d) => {
                res = op(res, d);
                false
            }
            _ => panic!("Invalid Type"),
        };
        if is_left_int && is_right_int {
            Value::IntValue(res as i64)
        } else {
            Value::DoubleValue(res)
        }
    }
}
