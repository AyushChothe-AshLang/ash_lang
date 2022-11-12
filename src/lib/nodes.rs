#![allow(clippy::new_ret_no_self)]
use ordered_float::OrderedFloat;

use crate::values::Value;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
};

// Node
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Int(IntNode),
    Double(DoubleNode),
    Boolean(BooleanNode),
    String(StringNode),
    Comment(CommentNode),
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
    Break,
    Continue,
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

// CommentNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentNode {
    pub value: String,
}
// ListNode
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListNode {
    pub elements: Vec<Node>,
}

// MapNode
#[derive(Debug, Clone, Eq)]
pub struct MapNode {
    pub elements: HashMap<Node, Node>,
}

impl PartialEq for MapNode {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
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
impl Display for UnaryArithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryArithmetic::Plus => write!(f, "+"),
            UnaryArithmetic::Minus => write!(f, "-"),
        }
    }
}

// UnaryOperator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Not,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "!"),
        }
    }
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

impl Display for Arithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arithmetic::Addition => write!(f, "+"),
            Arithmetic::Subtraction => write!(f, "-"),
            Arithmetic::Multiply => write!(f, "*"),
            Arithmetic::Divide => write!(f, "/"),
            Arithmetic::Power => write!(f, "^"),
            Arithmetic::Modulus => write!(f, "%"),
            Arithmetic::TildeDivide => write!(f, "~/"),
            Arithmetic::PowerDivide => write!(f, "^/"),
        }
    }
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

impl Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comparison::DoubleEquals => write!(f, "=="),
            Comparison::NotEquals => write!(f, "!="),
            Comparison::LessThan => write!(f, "<"),
            Comparison::LessThanEq => write!(f, "<="),
            Comparison::GreaterThan => write!(f, ">"),
            Comparison::GreaterThanEq => write!(f, ">="),
            Comparison::And => write!(f, "&"),
            Comparison::Or => write!(f, "|"),
        }
    }
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

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assignment::Equals => write!(f, "="),
            Assignment::PlusEq => write!(f, "+="),
            Assignment::MinusEq => write!(f, "-="),
            Assignment::MultiplyEq => write!(f, "*="),
            Assignment::DivideEq => write!(f, "/="),
            Assignment::ModulusEq => write!(f, "%/"),
            Assignment::PowerEq => write!(f, "^="),
            Assignment::TildeDivideEq => write!(f, "~/="),
            Assignment::PowerDivideEq => write!(f, "^/="),
        }
    }
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
#[derive(Debug, Clone, Eq)]
pub struct FunctionDeclarationNode {
    pub id: String,
    pub params: Vec<String>,
    pub body: Box<Node>,
    pub memo: Option<RefCell<HashMap<Vec<Value>, Value>>>,
}

impl PartialEq for FunctionDeclarationNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.params == other.params
            && self.body == other.body
            && self.memo == other.memo
    }
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
    pub res: Option<Box<Node>>,
}
impl ReturnNode {
    pub fn new(res: Option<Box<Node>>) -> Node {
        Node::Return(ReturnNode { res })
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Int(_i) => write!(f, "{}", _i.value),
            Node::Double(_d) => write!(f, "{}", _d.value),
            Node::Boolean(_b) => write!(f, "{}", _b.value),
            Node::String(_s) => write!(f, "\"{}\"", _s.value),
            Node::Comment(_s) => write!(f, "// {}", _s.value.trim_start()),
            Node::Identifier(_id) => write!(f, "{}", _id.value),
            Node::UnaryNumber(_un) => write!(f, "{}{}", _un.op, _un.value),
            Node::UnaryBoolean(_ub) => write!(f, "{}{}", _ub.op, _ub.value),
            Node::BinaryOpNumber(_bon) => write!(f, "{} {} {}", _bon.left, _bon.op, _bon.right),
            Node::BinaryOpBoolean(_bob) => write!(f, "{} {} {}", _bob.left, _bob.op, _bob.right),
            Node::Assignment(_a) => write!(f, "{} {} {}", _a.id, _a.assign_type, _a.value),
            Node::Declaration(_dec) => write!(f, "let {} = {};", _dec.id, _dec.value),
            Node::MultiDeclaration(_mdec) => {
                write!(f, "let ")?;
                write!(
                    f,
                    "{}",
                    _mdec
                        .declarations
                        .iter()
                        .map(|_d| {
                            if let Node::Declaration(_dec) = _d {
                                return format!("{} = {}", _dec.id, _dec.value);
                            }
                            String::new()
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
                write!(f, ";")
            }
            Node::BlockStatement(_blk) => {
                // for _n in &_blk.value {
                //     if let &Node::FunctionCall(_) = _n {
                //         writeln!(f, "\n{};", _n)?;
                //     } else {
                //         writeln!(f, "\n{}", _n)?;
                //     }
                // }
                // Ok(())
                panic!("Default BlockStatement");
            }
            Node::FunctionDeclaration(_fnd) => write!(f, "fn {}() {{{}}}", _fnd.id, _fnd.body),
            Node::FunctionCall(_fnc) => {
                write!(f, "{}(", _fnc.id)?;
                write!(
                    f,
                    "{}",
                    _fnc.args
                        .iter()
                        .map(|_a| { format!("{}", _a) })
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
                write!(f, ")")
            }
            Node::WhileLoop(_) => todo!("WhileLoop"),
            Node::IfStatement(_) => todo!("IfStatement"),
            Node::ElifStatement(_) => todo!("ElifStatement"),
            Node::Return(_rtn) => {
                if let Some(res) = &_rtn.res {
                    write!(f, "return {};", res)
                } else {
                    write!(f, "return;")
                }
            }
            Node::Break => write!(f, "break;"),
            Node::Continue => write!(f, "continue;"),
            Node::List(_l) => {
                write!(f, "[")?;
                write!(
                    f,
                    "{}",
                    _l.elements
                        .iter()
                        .map(|_e| { format!("{}", _e) })
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
                write!(f, "]")
            }
            Node::Map(_m) => {
                write!(f, "{{")?;
                write!(
                    f,
                    "{}",
                    _m.elements
                        .iter()
                        .map(|_e| { format!("{}:{}", _e.0, _e.1) })
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
                write!(f, "}}")
            }
        }
    }
}
