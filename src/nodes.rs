use crate::values::Value;
use std::{collections::HashMap, fmt::Debug};

// Node
#[derive(Debug, Clone)]
pub enum Node {
    Int(IntNode),
    Double(DoubleNode),
    Boolean(BooleanNode),
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
#[derive(Debug, Clone, Copy)]
pub struct IntNode {
    pub value: i64,
}

// DoubleNode
#[derive(Debug, Clone, Copy)]
pub struct DoubleNode {
    pub value: f64,
}
// BooleanNode
#[derive(Debug, Clone, Copy)]
pub struct BooleanNode {
    pub value: bool,
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
    //Logical
    And,
    Or,
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
    pub value: Vec<Node>,
}

impl BlockStatementNode {
    pub fn new(value: Vec<Node>) -> Node {
        Node::BlockStatement(BlockStatementNode { value })
    }
}

// FunctionCallNode
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct FunctionDeclarationNode {
    pub id: String,
    pub params: Vec<String>,
    pub body: Box<Node>,
    pub memo: HashMap<Vec<Value>, Value>,
}

impl FunctionDeclarationNode {
    pub fn new_fn(id: String, params: Vec<String>, body: Box<Node>) -> Node {
        Node::FunctionDeclaration(FunctionDeclarationNode {
            id,
            params,
            body,
            memo: HashMap::new(),
        })
    }
    pub fn new_cfn(id: String, params: Vec<String>, body: Box<Node>) -> Node {
        Node::FunctionDeclaration(FunctionDeclarationNode {
            id,
            params,
            body,
            memo: HashMap::new(),
        })
    }
    pub fn contains_key(&self, args: &Vec<Value>) -> bool {
        self.memo.contains_key(args)
    }
    pub fn set_cache(&mut self, args: Vec<Value>, res: Value) {
        self.memo.insert(args, res);
    }
    pub fn get_cache(&self, args: &Vec<Value>) -> Value {
        self.memo.get(args).unwrap().clone()
    }
}

// MultiDeclarationNode
#[derive(Debug, Clone)]
pub struct MultiDeclarationNode {
    pub declarations: Vec<Node>,
}
impl MultiDeclarationNode {
    pub fn new(declarations: Vec<Node>) -> Node {
        Node::MultiDeclaration(MultiDeclarationNode { declarations })
    }
}
// DeclarationNode
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub res: Box<Node>,
}
impl ReturnNode {
    pub fn new(res: Box<Node>) -> Node {
        Node::Return(ReturnNode { res })
    }
}
