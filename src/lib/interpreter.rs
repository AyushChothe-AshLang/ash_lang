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
    ast: Node,
    builtin: Rc<HashMap<String, BuiltInFn>>,
}

impl Interpreter {
    pub fn new(ast: Node) -> Self {
        let builtin: HashMap<String, BuiltInFn> = HashMap::from([
            (String::from("print"), ash_print as BuiltInFn),
            (String::from("println"), ash_println as BuiltInFn),
            (String::from("input"), ash_input as BuiltInFn),
            (String::from("int"), ash_int as BuiltInFn),
            (String::from("double"), ash_double as BuiltInFn),
            (String::from("str"), ash_str as BuiltInFn),
            (String::from("list"), ash_list as BuiltInFn),
            (String::from("min"), ash_min as BuiltInFn),
            (String::from("max"), ash_max as BuiltInFn),
            (String::from("get"), ash_get as BuiltInFn),
            (String::from("set"), ash_set as BuiltInFn),
            (String::from("len"), ash_len as BuiltInFn),
            (String::from("pop"), ash_pop as BuiltInFn),
            (String::from("keys"), ash_keys as BuiltInFn),
            (String::from("has"), ash_has as BuiltInFn),
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
            Node::String(_node) => self.walk_string_node(_node),
            Node::List(_node) => self.walk_list_node(_node, scope),
            Node::Map(_node) => self.walk_map_node(_node, scope),
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
            Node::Break => Value::Break,
            Node::Continue => Value::Continue,
            Node::ElifStatement(_) => panic!("This can't happen"),
        }
    }

    fn walk_int_node(&self, node: &IntNode) -> Value {
        Value::IntValue(node.value)
    }

    fn walk_double_node(&self, node: &DoubleNode) -> Value {
        Value::DoubleValue(node.value)
    }

    fn walk_boolean_node(&self, node: &BooleanNode) -> Value {
        Value::BooleanValue(node.value)
    }

    fn walk_string_node(&self, node: &StringNode) -> Value {
        Value::StringValue(node.value.to_owned())
    }

    fn walk_list_node(&mut self, node: &ListNode, scope: &mut ScopePtr) -> Value {
        Value::ListValue(
            node.elements
                .clone()
                .iter_mut()
                .map(|e| self.walk(e, scope))
                .collect(),
        )
    }

    fn walk_map_node(&mut self, node: &MapNode, scope: &mut ScopePtr) -> Value {
        Value::MapValue(
            node.elements
                .clone()
                .iter_mut()
                .map(|(k, v)| (self.walk(&mut k.clone(), scope), self.walk(v, scope)))
                .collect(),
        )
    }

    fn walk_return_node(&mut self, node: &mut ReturnNode, scope: &mut ScopePtr) -> Value {
        let mut res = Value::None;
        if let Some(val) = &mut (node.res) {
            res = self.walk(val, scope);
        }
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
                } else if res == Value::Break || res == Value::Continue {
                    return res;
                }
            }
        } else {
            for stmt in node.value.iter_mut() {
                res = self.walk(stmt, scope);
                if let Value::ReturnValue(_) = res {
                    return res;
                } else if res == Value::Break || res == Value::Continue {
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
        self.perform_op(left, right, node.op)
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
        match &left {
            Value::BooleanValue(_left) => match &right {
                Value::BooleanValue(_right) => Value::BooleanValue(*_left || *_right),
                _ => panic!("Invalid Operands"),
            },
            Value::StringValue(_left) => match &right {
                Value::StringValue(_right) => {
                    if _left == "" {
                        right
                    } else {
                        left
                    }
                }
                _ => panic!("Invalid Operands"),
            },
            Value::ListValue(_left) => match &right {
                Value::ListValue(_right) => {
                    if _left.len() == 0 {
                        right
                    } else {
                        left
                    }
                }
                _ => panic!("Invalid Operands"),
            },
            _ => panic!("Invalid Operands"),
        }
    }

    fn perform_op(&self, left: Value, right: Value, op: Arithmetic) -> Value {
        match left {
            Value::IntValue(l) => match right {
                Value::IntValue(r) => match op {
                    Arithmetic::Addition => Value::IntValue(l + r),
                    Arithmetic::Subtraction => Value::IntValue(l - r),
                    Arithmetic::Multiply => Value::IntValue(l * r),
                    Arithmetic::Power => Value::IntValue(l.pow(r as u32)),
                    Arithmetic::Modulus => Value::IntValue(l % r),
                    Arithmetic::Divide => {
                        let res = l as f64 / r as f64;
                        if res.fract() == 0.0 {
                            Value::IntValue(res as i64)
                        } else {
                            Value::DoubleValue(OrderedFloat(res))
                        }
                    }
                    Arithmetic::TildeDivide => {
                        Value::IntValue((l as f64 / r as f64).floor() as i64)
                    }
                    Arithmetic::PowerDivide => Value::IntValue((l as f64 / r as f64).ceil() as i64),
                },
                Value::DoubleValue(r) => match op {
                    Arithmetic::Addition => Value::DoubleValue(OrderedFloat(l as f64 + r.0)),
                    Arithmetic::Subtraction => Value::DoubleValue(OrderedFloat(l as f64 - r.0)),
                    Arithmetic::Multiply => Value::DoubleValue(OrderedFloat(l as f64 * r.0)),
                    Arithmetic::Divide => Value::DoubleValue(OrderedFloat(l as f64 / r.0)),
                    Arithmetic::Power => Value::DoubleValue(OrderedFloat((l as f64).powf(r.0))),
                    Arithmetic::Modulus => Value::DoubleValue(OrderedFloat(l as f64 % r.0)),
                    Arithmetic::TildeDivide => Value::IntValue((l as f64 / r.0).floor() as i64),
                    Arithmetic::PowerDivide => Value::IntValue((l as f64 / r.0).ceil() as i64),
                },
                _ => panic!("Invalid Operands"),
            },
            Value::DoubleValue(l) => match right {
                Value::IntValue(r) => match op {
                    Arithmetic::Addition => Value::DoubleValue(OrderedFloat(l.0 + r as f64)),
                    Arithmetic::Subtraction => Value::DoubleValue(OrderedFloat(l.0 - r as f64)),
                    Arithmetic::Multiply => Value::DoubleValue(OrderedFloat(l.0 * r as f64)),
                    Arithmetic::Divide => Value::DoubleValue(OrderedFloat(l.0 / r as f64)),
                    Arithmetic::Power => Value::DoubleValue(OrderedFloat((l.0).powf(r as f64))),
                    Arithmetic::Modulus => Value::DoubleValue(OrderedFloat(l.0 % r as f64)),
                    Arithmetic::TildeDivide => Value::IntValue((l.0 / r as f64).floor() as i64),
                    Arithmetic::PowerDivide => Value::IntValue((l.0 / r as f64).ceil() as i64),
                },
                Value::DoubleValue(r) => match op {
                    Arithmetic::Addition => Value::DoubleValue(OrderedFloat(l.0 + r.0)),
                    Arithmetic::Subtraction => Value::DoubleValue(OrderedFloat(l.0 - r.0)),
                    Arithmetic::Multiply => Value::DoubleValue(OrderedFloat(l.0 * r.0)),
                    Arithmetic::Divide => Value::DoubleValue(OrderedFloat(l.0 / r.0)),
                    Arithmetic::Power => Value::DoubleValue(OrderedFloat((l.0).powf(r.0))),
                    Arithmetic::Modulus => Value::DoubleValue(OrderedFloat(l.0 % r.0)),
                    Arithmetic::TildeDivide => Value::IntValue((l / r.0).floor() as i64),
                    Arithmetic::PowerDivide => Value::IntValue((l / r.0).ceil() as i64),
                },
                _ => panic!("Invalid Operands"),
            },
            Value::StringValue(l) => match right {
                Value::IntValue(r) => match op {
                    Arithmetic::Multiply => {
                        let mut res = String::new();
                        for _ in 0..r {
                            res.push_str(l.as_str());
                        }
                        Value::StringValue(res)
                    }
                    _ => panic!("Invalid Operands"),
                },
                Value::StringValue(r) => match op {
                    Arithmetic::Addition => Value::StringValue(l + r.as_str()),
                    _ => panic!("Invalid Operands"),
                },

                _ => panic!("Invalid Operands"),
            },
            Value::ListValue(l) => match right {
                Value::IntValue(i) => match op {
                    Arithmetic::Multiply => {
                        let mut res = vec![];
                        for _ in 0..i {
                            res.extend(l.clone());
                        }
                        Value::ListValue(res)
                    }
                    _ => panic!("Invalid Operands"),
                },
                Value::ListValue(r) => match op {
                    Arithmetic::Addition => Value::ListValue([l, r].concat()),
                    _ => panic!("Invalid Operands"),
                },
                _ => panic!("Invalid Operands"),
            },
            Value::MapValue(l) => match right {
                Value::MapValue(r) => match op {
                    Arithmetic::Addition => {
                        let mut lc = l.clone();
                        for (k, v) in r.iter() {
                            lc.insert(k.clone(), v.clone());
                        }
                        Value::MapValue(lc)
                    }
                    _ => panic!("Invalid Operands"),
                },
                _ => panic!("Invalid Operands"),
            },
            _ => panic!("Invalid Operands"),
        }
    }

    fn walk_assignment_node(&mut self, node: &mut AssignmentNode, scope: &mut ScopePtr) -> Value {
        let id = &node.id;
        let left = self.walk(
            &mut Node::Identifier(IdentifierNode {
                value: id.to_owned(),
            }),
            scope,
        );
        let right = self.walk(&mut node.value, scope);

        scope.borrow_mut().set_symbol(
            id,
            match node.assign_type {
                Assignment::Equals => right,
                Assignment::PlusEq => self.perform_op(left, right, Arithmetic::Addition),
                Assignment::MinusEq => self.perform_op(left, right, Arithmetic::Subtraction),
                Assignment::MultiplyEq => self.perform_op(left, right, Arithmetic::Multiply),
                Assignment::DivideEq => self.perform_op(left, right, Arithmetic::Divide),
                Assignment::ModulusEq => self.perform_op(left, right, Arithmetic::Modulus),
                Assignment::PowerEq => self.perform_op(left, right, Arithmetic::Power),
                Assignment::TildeDivideEq => self.perform_op(left, right, Arithmetic::TildeDivide),
                Assignment::PowerDivideEq => self.perform_op(left, right, Arithmetic::PowerDivide),
            },
        );
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
        } else {
            let _fn = scope.borrow().get_function(id).clone();

            // AshLang Function
            let mut vals = vec![];
            for arg in node.args.iter_mut() {
                vals.push(self.walk(arg, scope));
            }

            if vals.len() != _fn.borrow().params.len() {
                panic!("Invalid number of arguments to function")
            }

            // Return Memo Value if CFn
            if _fn.borrow().memo.is_some() {
                if _fn.borrow().contains_key(&vals) {
                    return _fn.borrow().get_cache(&vals);
                }
            }

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

            // Store Memo Value in CFn
            if _fn.borrow().memo.is_some() {
                _fn.borrow().set_cache(vals, res.clone());
            }

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
            } else if res == Value::Break {
                break;
            } else if res == Value::Continue {
                continue;
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
