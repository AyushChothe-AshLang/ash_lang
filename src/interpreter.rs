use std::collections::HashMap;

use super::built_in::*;

use super::nodes::*;
use super::scope::{Scope, ScopePtr};
use super::values::*;

type BuiltInFn = fn(Vec<Value>) -> Value;

pub struct Interpreter {
    ast: Box<Node>,
    global_scope: ScopePtr,
    builtin: HashMap<String, BuiltInFn>,
}

impl Interpreter {
    pub fn new(ast: Box<Node>) -> Self {
        let builtin: HashMap<String, BuiltInFn> = HashMap::from([
            (String::from("print"), ash_print as BuiltInFn),
            (String::from("println"), ash_println as BuiltInFn),
        ]);
        Interpreter {
            ast,
            builtin,
            global_scope: Scope::from(HashMap::new(), HashMap::new()),
        }
    }

    pub fn eval(&mut self) -> Value {
        self.walk(&mut self.ast.clone())
    }

    fn walk(&mut self, node: &mut Node) -> Value {
        match node {
            Node::Int(_node) => self.walk_int_node(_node),
            Node::Double(_node) => self.walk_double_node(_node),
            Node::BinaryOpNumber(_node) => self.walk_binary_op_number_node(_node),
            Node::BinaryOpBoolean(_node) => self.walk_binary_op_boolean_node(_node),
            Node::UnaryNumber(_node) => self.walk_unary_number_node(_node),
            Node::UnaryBoolean(_node) => self.walk_unary_boolean_node(_node),
            Node::Assignment(_node) => self.walk_assignment_node(_node),
            Node::Identifier(_node) => self.walk_identifier_node(_node),
            Node::BlockStatement(_node) => self.walk_block_statement_node(_node),
            Node::FunctionCall(_node) => self.walk_function_call_node(_node),
            Node::FunctionDeclaration(_node) => self.walk_function_declaration_node(_node),
        }
    }

    fn walk_int_node(&self, node: &IntNode) -> Value {
        Value::IntValue(node.value)
    }

    fn walk_double_node(&self, node: &DoubleNode) -> Value {
        Value::DoubleValue(node.value)
    }

    fn walk_identifier_node(&mut self, node: &IdentifierNode) -> Value {
        let key = node.value.clone();
        self.global_scope.borrow().get_symbol(key)
    }

    fn walk_block_statement_node(&mut self, node: &BlockStatementNode) -> Value {
        // let local = Scope::new(self.global_scope.clone());
        let mut res = Value::None;
        for mut stmt in node.value.clone() {
            res = self.walk(&mut *stmt);
        }
        res
    }

    fn walk_unary_number_node(&mut self, node: &mut UnaryNumberNode) -> Value {
        let res = self.walk(&mut node.value);
        match res {
            Value::IntValue(i) => match node.op {
                UnaryArithmetic::Plus => Value::IntValue(i),
                UnaryArithmetic::Minus => Value::IntValue(-i),
            },
            Value::DoubleValue(d) => match node.op {
                UnaryArithmetic::Plus => Value::DoubleValue(d),
                UnaryArithmetic::Minus => Value::DoubleValue(-d),
            },
            _ => panic!("Invalid Unary Operand!"),
        }
    }

    fn walk_unary_boolean_node(&mut self, node: &mut UnaryBooleanNode) -> Value {
        let res = self.walk(&mut node.value);
        match res {
            Value::BooleanValue(b) => match node.op {
                UnaryOperator::Not => Value::BooleanValue(!b),
            },
            _ => panic!("Invalid Unary Operand!"),
        }
    }

    fn walk_binary_op_boolean_node(&mut self, node: &mut BinaryOpBooleanNode) -> Value {
        let left = self.walk(&mut node.left);
        let right = self.walk(&mut node.right);

        match node.op {
            Comparison::DoubleEquals => Value::BooleanValue(left == right),
            Comparison::NotEquals => Value::BooleanValue(left != right),
            Comparison::LessThan => Value::BooleanValue(left < right),
            Comparison::LessThanEq => Value::BooleanValue(left <= right),
            Comparison::GreaterThan => Value::BooleanValue(left > right),
            Comparison::GreaterThanEq => Value::BooleanValue(left >= right),
        }
    }

    fn walk_binary_op_number_node(&mut self, node: &mut BinaryOpNumberNode) -> Value {
        let left = self.walk(&mut node.left);
        let right = self.walk(&mut node.right);
        match node.op {
            Arithmetic::Addition => self.perform_op(left, right, |res, x| res + x),
            Arithmetic::Subtraction => self.perform_op(left, right, |res, x| res - x),
            Arithmetic::Multiply => self.perform_op(left, right, |res, x| res * x),
            Arithmetic::Divide => self.perform_op(left, right, |res, x| res / x),
            Arithmetic::Power => self.perform_op(left, right, |res, x| res.powf(x)),
            Arithmetic::Modulus => self.perform_op(left, right, |res, x| res % x),
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
        if is_left_int && is_right_int && res.fract() == 0.0 {
            Value::IntValue(res as i64)
        } else {
            Value::DoubleValue(res)
        }
    }

    fn walk_assignment_node(&mut self, node: &mut AssignmentNode) -> Value {
        let id = node.id.clone();
        let value = self.walk(&mut node.value);
        self.global_scope.borrow_mut().declare_symbol(id, value);
        Value::None
    }

    fn walk_function_declaration_node(&mut self, node: &FunctionDeclarationNode) -> Value {
        let fn_id = node.id.clone();
        self.global_scope
            .borrow_mut()
            .declare_function(fn_id, node.clone());
        Value::None
    }

    fn walk_function_call_node(&mut self, node: &mut FunctionCallNode) -> Value {
        let id = node.id.clone();
        if let Some(_fn) = self.builtin.get(&id) {
            let mut vals = vec![];
            for mut arg in node.args.clone() {
                vals.push(self.walk(&mut arg));
            }
            return self.builtin.get(&id).unwrap()(vals);
        }

        // let _fn = self.global_scope.borrow().get_function(id);
        // let mut vals = vec![];
        // for mut arg in node.args.clone() {
        //     vals.push(self.walk(&mut arg));
        // }

        // _fn.params

        panic!("Function not Found")
    }
}
