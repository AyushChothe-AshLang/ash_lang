use ordered_float::OrderedFloat;

use crate::values::Value;
use std::{cell::RefCell, collections::HashMap, fmt::Debug};

// Node
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Int(IntNode),
    Double(DoubleNode),
    Boolean(BooleanNode),
    String(StringNode),
    List(ListNode),
    Map(MapNode),
    Identifier(IdentifierNode),
    UnaryNumber(UnaryNumberNode),
    UnaryBoolean(UnaryBooleanNode),
    BinaryOpNumber(BinaryOpNumberNode),
    BinaryOpBoolean(BinaryOpBooleanNode),
    Assignment(AssignmentNode),
    MultiDeclaration(MultiDeclarationNode),
    Declaration(DeclarationNode),
    BlockStatement(BlockStatementNode),
    FunctionCall(FunctionCallNode),
    FunctionDeclaration(FunctionDeclarationNode),
    WhileLoop(WhileLoopNode),
    IfStatement(IfStatementNode),
    ElifStatement(ElifStatementNode),
    Return(ReturnNode),
}

// IntNode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntNode {
    pub value: i64,
}

// DoubleNode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DoubleNode {
    pub value: OrderedFloat<f64>,
}
// BooleanNode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BooleanNode {
    pub value: bool,
}
// StringNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringNode {
    pub value: String,
}
// ListNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListNode {
    pub elements: Vec<Node>,
}

// MapNode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapNode {
    pub elements: HashMap<Node, Node>,
}

impl std::hash::Hash for MapNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (k, v) in self.elements.iter() {
            (k, v).hash(state);
        }
    }
}

// IdentifierNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierNode {
    pub value: String,
}

// UnaryArithmetic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryArithmetic {
    Plus,
    Minus,
}
// UnaryOperator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Not,
}
// UnaryNumberNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arithmetic {
    Addition,    // +
    Subtraction, // -
    Multiply,    // *
    Divide,      // /
    Power,       // ^
    Modulus,     // %
    TildeDivide, // ~/
    PowerDivide, // ^/
}
// Comparison Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
    DoubleEquals,  // ==
    NotEquals,     // !=
    LessThan,      // <
    LessThanEq,    // <=
    GreaterThan,   // >
    GreaterThanEq, // >=
    //Logical
    And,
    Or,
}

// Assignment Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Assignment {
    Equals,        // =
    PlusEq,        // +=
    MinusEq,       // -=
    MultiplyEq,    // *=
    DivideEq,      // /=
    ModulusEq,     // %/
    PowerEq,       // ^=
    TildeDivideEq, // ~/=
    PowerDivideEq, // ^/=
}

// BinaryOpNumberNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub fn tilde_divide(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::TildeDivide)
    }
    pub fn power_divide(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::PowerDivide)
    }
    pub fn power(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::Power)
    }
    pub fn modulus(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpNumberNode::new(left, right, Arithmetic::Modulus)
    }
}
// BinaryOpBooleanNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub fn and(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpBooleanNode::new(left, right, Comparison::And)
    }
    pub fn or(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpBooleanNode::new(left, right, Comparison::Or)
    }
    pub fn neq(left: Box<Node>, right: Box<Node>) -> Node {
        BinaryOpBooleanNode::new(left, right, Comparison::NotEquals)
    }
}

// AssignmentNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignmentNode {
    pub id: String,
    pub value: Box<Node>,
    pub assign_type: Assignment,
}

impl AssignmentNode {
    pub fn new(id: String, value: Box<Node>, assign_type: Assignment) -> Node {
        Node::Assignment(AssignmentNode {
            id,
            value,
            assign_type,
        })
    }
}

// BlockStatementNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockStatementNode {
    pub value: Vec<Node>,
}

impl BlockStatementNode {
    pub fn new(value: Vec<Node>) -> Node {
        Node::BlockStatement(BlockStatementNode { value })
    }
}

// FunctionCallNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionCallNode {
    pub id: String,
    pub args: Vec<Node>,
}

impl FunctionCallNode {
    pub fn new(id: String, args: Vec<Node>) -> Node {
        Node::FunctionCall(FunctionCallNode { id, args })
    }
}

// FunctionDeclarationNode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDeclarationNode {
    pub id: String,
    pub params: Vec<String>,
    pub body: Box<Node>,
    pub memo: Option<RefCell<HashMap<Vec<Value>, Value>>>,
}
impl std::hash::Hash for FunctionDeclarationNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.params.hash(state);
        self.body.hash(state);
        core::mem::discriminant(&self.memo).hash(state);
    }
}

impl FunctionDeclarationNode {
    pub fn new_fn(id: String, params: Vec<String>, body: Box<Node>) -> Node {
        Node::FunctionDeclaration(FunctionDeclarationNode {
            id,
            params,
            body,
            memo: None,
        })
    }
    pub fn new_cfn(id: String, params: Vec<String>, body: Box<Node>) -> Node {
        Node::FunctionDeclaration(FunctionDeclarationNode {
            id,
            params,
            body,
            memo: Some(RefCell::new(HashMap::new())),
        })
    }
    pub fn contains_key(&self, args: &Vec<Value>) -> bool {
        self.memo.as_ref().unwrap().borrow().contains_key(args)
    }
    pub fn set_cache(&self, args: Vec<Value>, res: Value) {
        self.memo.as_ref().unwrap().borrow_mut().insert(args, res);
    }
    pub fn get_cache(&self, args: &Vec<Value>) -> Value {
        self.memo
            .as_ref()
            .unwrap()
            .borrow()
            .get(args)
            .unwrap()
            .clone()
    }
}

// MultiDeclarationNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultiDeclarationNode {
    pub declarations: Vec<Node>,
}
impl MultiDeclarationNode {
    pub fn new(declarations: Vec<Node>) -> Node {
        Node::MultiDeclaration(MultiDeclarationNode { declarations })
    }
}
// DeclarationNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclarationNode {
    pub id: String,
    pub value: Box<Node>,
}
impl DeclarationNode {
    pub fn new(id: String, value: Box<Node>) -> Node {
        Node::Declaration(DeclarationNode { id, value })
    }
}

// WhileLoopNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileLoopNode {
    pub condition: Box<Node>,
    pub body: Box<Node>,
}
impl WhileLoopNode {
    pub fn new(condition: Box<Node>, body: Box<Node>) -> Node {
        Node::WhileLoop(WhileLoopNode { condition, body })
    }
}

// IfStatementNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStatementNode {
    pub condition: Box<Node>,
    pub true_block: Box<Node>,
    pub elif_blocks: Vec<Node>,
    pub else_block: Option<Box<Node>>,
}
impl IfStatementNode {
    pub fn new(
        condition: Box<Node>,
        true_block: Box<Node>,
        elif_blocks: Vec<Node>,
        else_block: Option<Box<Node>>,
    ) -> Node {
        Node::IfStatement(IfStatementNode {
            condition,
            true_block,
            elif_blocks,
            else_block,
        })
    }
}

// ElifStatementNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElifStatementNode {
    pub condition: Box<Node>,
    pub true_block: Box<Node>,
}
impl ElifStatementNode {
    pub fn new(condition: Box<Node>, true_block: Box<Node>) -> Node {
        Node::ElifStatement(ElifStatementNode {
            condition,
            true_block,
        })
    }
}

// ReturnNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnNode {
    pub res: Box<Node>,
}
impl ReturnNode {
    pub fn new(res: Box<Node>) -> Node {
        Node::Return(ReturnNode { res })
    }
}
