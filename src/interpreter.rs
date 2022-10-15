use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::built_in::*;

use super::nodes::*;
use super::scope::{Scope, ScopePtr};
use super::values::*;

type BuiltInFn = fn(Vec<Value>) -> Value;

pub struct Interpreter {
    ast: Box<Node>,
    builtin: Rc<HashMap<String, BuiltInFn>>,
}

impl Interpreter {
    pub fn new(ast: Box<Node>) -> Self {
        let builtin: HashMap<String, BuiltInFn> = HashMap::from([
            (String::from("print"), ash_print as BuiltInFn),
            (String::from("println"), ash_println as BuiltInFn),
            (String::from("min"), ash_min as BuiltInFn),
            (String::from("max"), ash_max as BuiltInFn),
        ]);
        Interpreter {
            ast,
            builtin: Rc::new(builtin),
        }
    }

    pub fn eval(&mut self) -> Value {
        let mut global_scope = Scope::from(HashMap::new(), HashMap::new());
        self.walk(&mut self.ast.to_owned(), &mut global_scope)
    }

    fn walk(&mut self, node: &mut Node, scope: &mut ScopePtr) -> Value {
        match node {
            Node::Int(_node) => self.walk_int_node(_node),
            Node::Double(_node) => self.walk_double_node(_node),
            Node::Boolean(_node) => self.walk_boolean_node(_node),
            Node::BinaryOpNumber(_node) => self.walk_binary_op_number_node(_node, scope),
            Node::BinaryOpBoolean(_node) => self.walk_binary_op_boolean_node(_node, scope),
            Node::UnaryNumber(_node) => self.walk_unary_number_node(_node, scope),
            Node::UnaryBoolean(_node) => self.walk_unary_boolean_node(_node, scope),
            Node::Assignment(_node) => self.walk_assignment_node(_node, scope),
            Node::Identifier(_node) => self.walk_identifier_node(_node, scope),
            Node::BlockStatement(_node) => self.walk_block_statement_node(_node, scope, true),
            Node::FunctionCall(_node) => self.walk_function_call_node(_node, scope),
            Node::FunctionDeclaration(_node) => self.walk_function_declaration_node(_node, scope),
            Node::MultiDeclaration(_node) => self.walk_multi_declaration_node(_node, scope),
            Node::Declaration(_node) => self.walk_declaration_node(_node, scope),
            Node::WhileLoop(_node) => self.walk_while_loop_node(_node, scope),
            Node::IfStatement(_node) => self.walk_if_statement_node(_node, scope),
            Node::Return(_node) => self.walk_return_node(_node, scope),
            Node::ElifStatement(_) => panic!("This can't happen"),
        }
    }

    fn walk_int_node(&self, node: &IntNode) -> Value {
        Value::IntValue(node.value)
    }

    fn walk_double_node(&self, node: &DoubleNode) -> Value {
        Value::DoubleValue(OrderedFloat(node.value))
    }

    fn walk_boolean_node(&self, node: &BooleanNode) -> Value {
        Value::BooleanValue(node.value)
    }

    fn walk_return_node(&mut self, node: &mut ReturnNode, scope: &mut ScopePtr) -> Value {
        let res = self.walk(&mut node.res, scope);
        Value::ReturnValue(Box::new(res))
    }

    fn walk_identifier_node(&mut self, node: &IdentifierNode, scope: &mut ScopePtr) -> Value {
        let key = &node.value;
        scope.borrow().get_symbol(key)
    }

    fn walk_block_statement_node(
        &mut self,
        node: &mut BlockStatementNode,
        scope: &mut ScopePtr,
        create_scope: bool,
    ) -> Value {
        let mut res = Value::None;
        if create_scope {
            let local = &mut Scope::new(scope.clone());
            for stmt in node.value.iter_mut() {
                res = self.walk(stmt, local);
                if let Value::ReturnValue(_) = res {
                    return res;
                }
            }
        } else {
            for stmt in node.value.iter_mut() {
                res = self.walk(stmt, scope);
                if let Value::ReturnValue(_) = res {
                    return res;
                }
            }
        }
        res
    }

    fn walk_unary_number_node(
        &mut self,
        node: &mut UnaryNumberNode,
        scope: &mut ScopePtr,
    ) -> Value {
        let res = self.walk(&mut node.value, scope);
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

    fn walk_unary_boolean_node(
        &mut self,
        node: &mut UnaryBooleanNode,
        scope: &mut ScopePtr,
    ) -> Value {
        let res = self.walk(&mut node.value, scope);
        match res {
            Value::BooleanValue(b) => match node.op {
                UnaryOperator::Not => Value::BooleanValue(!b),
            },
            _ => panic!("Invalid Unary Operand!"),
        }
    }

    fn walk_binary_op_boolean_node(
        &mut self,
        node: &mut BinaryOpBooleanNode,
        scope: &mut ScopePtr,
    ) -> Value {
        let left = self.walk(&mut node.left, scope);
        let right = self.walk(&mut node.right, scope);

        match node.op {
            Comparison::DoubleEquals => Value::BooleanValue(left == right),
            Comparison::NotEquals => Value::BooleanValue(left != right),
            Comparison::LessThan => Value::BooleanValue(left < right),
            Comparison::LessThanEq => Value::BooleanValue(left <= right),
            Comparison::GreaterThan => Value::BooleanValue(left > right),
            Comparison::GreaterThanEq => Value::BooleanValue(left >= right),
            Comparison::And => self.and(left, right),
            Comparison::Or => self.or(left, right),
        }
    }

    fn walk_binary_op_number_node(
        &mut self,
        node: &mut BinaryOpNumberNode,
        scope: &mut ScopePtr,
    ) -> Value {
        let left = self.walk(&mut node.left, scope);
        let right = self.walk(&mut node.right, scope);
        match node.op {
            Arithmetic::Addition => self.perform_op(left, right, |res, x| res + x),
            Arithmetic::Subtraction => self.perform_op(left, right, |res, x| res - x),
            Arithmetic::Multiply => self.perform_op(left, right, |res, x| res * x),
            Arithmetic::Divide => self.perform_op(left, right, |res, x| res / x),
            Arithmetic::Power => self.perform_op(left, right, |res, x| res.powf(x)),
            Arithmetic::Modulus => self.perform_op(left, right, |res, x| res % x),
        }
    }

    fn and(&self, left: Value, right: Value) -> Value {
        match left {
            Value::BooleanValue(_left) => match right {
                Value::BooleanValue(_right) => Value::BooleanValue(_left && _right),
                _ => panic!("Invalid Operands"),
            },
            _ => panic!("Invalid Operands"),
        }
    }

    fn or(&self, left: Value, right: Value) -> Value {
        match left {
            Value::BooleanValue(_left) => match right {
                Value::BooleanValue(_right) => Value::BooleanValue(_left || _right),
                _ => panic!("Invalid Operands"),
            },
            _ => panic!("Invalid Operands"),
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
                res = d.0;
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
                res = op(res, d.0);
                false
            }
            _ => panic!("Invalid Type"),
        };
        if is_left_int && is_right_int && res.fract() == 0.0 {
            Value::IntValue(res as i64)
        } else {
            Value::DoubleValue(OrderedFloat(res))
        }
    }

    fn walk_assignment_node(&mut self, node: &mut AssignmentNode, scope: &mut ScopePtr) -> Value {
        let id = &node.id;
        let value = self.walk(&mut node.value, scope);
        scope.borrow_mut().set_symbol(id, value);
        Value::None
    }

    fn walk_multi_declaration_node(
        &mut self,
        node: &mut MultiDeclarationNode,
        scope: &mut ScopePtr,
    ) -> Value {
        for dec in node.declarations.iter_mut() {
            self.walk(dec, scope);
        }

        Value::None
    }
    fn walk_declaration_node(&mut self, node: &mut DeclarationNode, scope: &mut ScopePtr) -> Value {
        let id = &node.id;
        let value = self.walk(&mut node.value, scope);
        scope.borrow_mut().declare_symbol(id.to_owned(), value);
        Value::None
    }

    fn walk_function_declaration_node(
        &mut self,
        node: &FunctionDeclarationNode,
        scope: &mut ScopePtr,
    ) -> Value {
        let fn_id = &node.id;
        scope
            .borrow_mut()
            .declare_function(fn_id.to_owned(), Rc::new(RefCell::new(node.to_owned())));
        Value::None
    }

    fn walk_function_call_node(
        &mut self,
        node: &mut FunctionCallNode,
        scope: &mut ScopePtr,
    ) -> Value {
        // Builtin Function
        let id = &node.id;
        if let Some(_fn) = self.builtin.clone().get(id) {
            let mut vals = vec![];
            for arg in node.args.iter_mut() {
                let val = self.walk(arg, scope);
                vals.push(val);
            }
            return (_fn)(vals);
        }

        // AshLang Function
        let mut _fn = scope.borrow().get_function(id).clone();
        let mut vals = vec![];
        for arg in node.args.iter_mut() {
            vals.push(self.walk(arg, scope));
        }

        if vals.len() != _fn.borrow().params.len() {
            panic!("Invalid number of arguments to function")
        }

        if _fn.borrow().memo.contains_key(&vals) {
            _fn.borrow().memo.get(&vals).unwrap().clone()
        } else {
            let mut fn_scope = Scope::new(scope.clone());

            for (key, value) in _fn.borrow().params.iter().zip(vals.iter()) {
                fn_scope
                    .borrow_mut()
                    .declare_symbol(key.clone(), value.clone());
            }

            let mut res = self.walk_block_statement_node(
                match _fn.borrow().body.to_owned().as_mut() {
                    Node::BlockStatement(ref mut _node) => _node,
                    _ => panic!("Expected BlockStatement"),
                },
                &mut fn_scope,
                false,
            );

            if let Value::ReturnValue(_ret) = res {
                res = *_ret;
            }

            _fn.borrow_mut().memo.insert(vals, res.clone());

            res
        }
    }

    fn walk_while_loop_node(&mut self, node: &mut WhileLoopNode, scope: &mut ScopePtr) -> Value {
        while match self.walk(&mut node.condition, scope) {
            Value::BooleanValue(_b) => _b,
            _ => panic!("Invalid Type in While Condition"),
        } {
            let res = self.walk(&mut node.body, scope);
            if let Value::ReturnValue(_) = res {
                return res;
            }
        }
        Value::None
    }
    fn walk_if_statement_node(
        &mut self,
        node: &mut IfStatementNode,
        scope: &mut ScopePtr,
    ) -> Value {
        // Run If
        if match self.walk(&mut node.condition, scope) {
            Value::BooleanValue(_b) => _b,
            _ => panic!("Invalid Type in If Condition"),
        } {
            return self.walk(&mut node.true_block, scope);
        }

        // Run Elifs
        for elif in node.elif_blocks.iter_mut() {
            match *elif {
                Node::ElifStatement(ref mut _node) => {
                    if match self.walk(&mut _node.condition, scope) {
                        Value::BooleanValue(_b) => _b,
                        _ => todo!(),
                    } {
                        return self.walk(&mut _node.true_block, scope);
                    }
                }
                _ => panic!("Elif Node Expected"),
            }
        }

        // Run else
        if let Some(mut else_block) = node.else_block.as_mut() {
            return self.walk(&mut else_block, scope);
        }

        Value::None
    }
}
