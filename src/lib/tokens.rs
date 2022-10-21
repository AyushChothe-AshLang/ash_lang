use std::fmt::Display;

// TokenType
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i64, PosRange),           // 10
    Double(f64, PosRange),        // 3.14
    Boolean(bool, PosRange),      // 3.14
    String(String, PosRange),     // "AshLang"
    Identifier(String, PosRange), // Variables
    Plus(PosRange),               // +
    Minus(PosRange),              // -
    Multiply(PosRange),           // *
    Divide(PosRange),             // /
    Tilde(PosRange),              // ~
    TildeDivide(PosRange),        // ~/
    Power(PosRange),              // ^
    PowerDivide(PosRange),        // ^/
    Modulus(PosRange),            // %
    Not(PosRange),                // !
    Equals(PosRange),             // =
    PlusEq(PosRange),             // +=
    MinusEq(PosRange),            // -=
    MultiplyEq(PosRange),         // *=
    DivideEq(PosRange),           // /=
    TildeDivideEq(PosRange),      // ~/=
    PowerEq(PosRange),            // ^=
    PowerDivideEq(PosRange),      // ^/=
    ModulusEq(PosRange),          // %=
    DoubleEquals(PosRange),       // ==
    NotEquals(PosRange),          // !=
    And(PosRange),                // &
    Or(PosRange),                 // |
    LessThan(PosRange),           // <
    LessThanEq(PosRange),         // <=
    GreaterThan(PosRange),        // >
    GreaterThanEq(PosRange),      // >=
    LParan(PosRange),             // (
    RParan(PosRange),             // )
    LBrace(PosRange),             // {
    RBrace(PosRange),             // }
    LSquare(PosRange),            // [
    RSquare(PosRange),            // ]
    Comma(PosRange),              // ,
    Colon(PosRange),              // :
    Semicolon(PosRange),          // ;
    EOF(PosRange),                // End of File
    //Keywords
    WhileK(PosRange),    // while
    IfK(PosRange),       // if
    ElifK(PosRange),     // elif
    ElseK(PosRange),     // else
    FnK(PosRange),       // fn
    CFnK(PosRange),      // cfn
    LetK(PosRange),      // let
    BreakK(PosRange),    // break
    ContinueK(PosRange), // continue
    ReturnK(PosRange),   // return
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub line: i64,
    pub column: i64,
}
impl Position {
    pub fn from(line: i64, column: i64) -> Self {
        Position { line, column }
    }
    pub fn to(line: i64, column: i64) -> Option<Self> {
        Some(Position { line, column })
    }
    pub fn empty() -> Self {
        Position { line: 0, column: 0 }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PosRange {
    pub from: Position,
    pub to: Option<Position>,
}

impl PosRange {
    pub fn new(from: Position, to: Option<Position>) -> Self {
        PosRange { from, to }
    }
    pub fn empty() -> Self {
        PosRange {
            from: Position::empty(),
            to: None,
        }
    }
}

impl Token {
    pub fn get_name(&self) -> String {
        match self {
            Token::Int(_, _) => "Int",
            Token::Double(_, _) => "Double",
            Token::Boolean(_, _) => "Boolean",
            Token::String(_, _) => "String",
            Token::Identifier(_, _) => "Identifier",
            Token::Plus(_) => "Plus",
            Token::Minus(_) => "Minus",
            Token::Multiply(_) => "Multiply",
            Token::Divide(_) => "Divide",
            Token::Power(_) => "Power",
            Token::Modulus(_) => "Modulus",
            Token::Not(_) => "Not",
            Token::Equals(_) => "Equals",
            Token::DoubleEquals(_) => "DoubleEquals",
            Token::NotEquals(_) => "NotEquals",
            Token::LessThan(_) => "LessThan",
            Token::LessThanEq(_) => "LessThanEq",
            Token::GreaterThan(_) => "GreaterThan",
            Token::GreaterThanEq(_) => "GreaterThanEq",
            Token::LParan(_) => "LParan",
            Token::RParan(_) => "RParan",
            Token::LBrace(_) => "LBrace",
            Token::RBrace(_) => "RBrace",
            Token::LSquare(_) => "LSquare",
            Token::RSquare(_) => "RSquare",
            Token::Comma(_) => "Comma",
            Token::Colon(_) => "Colon",
            Token::Semicolon(_) => "Semicolon",
            Token::EOF(_) => "EOF",
            Token::WhileK(_) => "WhileK",
            Token::IfK(_) => "IfK",
            Token::ElifK(_) => "ElifK",
            Token::ElseK(_) => "ElseK",
            Token::FnK(_) => "FnK",
            Token::CFnK(_) => "CFnK",
            Token::LetK(_) => "LetK",
            Token::BreakK(_) => "BreakK",
            Token::ContinueK(_) => "ContinueK",
            Token::ReturnK(_) => "ReturnK",
            Token::And(_) => "And",
            Token::Or(_) => "Or",
            Token::TildeDivide(_) => "TildeDivide",
            Token::PowerDivide(_) => "PowerDivide",
            Token::PlusEq(_) => "PlusEq",
            Token::MinusEq(_) => "MinusEq",
            Token::MultiplyEq(_) => "MultiplyEq",
            Token::DivideEq(_) => "DivideEq",
            Token::TildeDivideEq(_) => "TildeDivideEq",
            Token::PowerEq(_) => "PowerEq",
            Token::PowerDivideEq(_) => "PowerDivideEq",
            Token::ModulusEq(_) => "ModulusEq",
            Token::Tilde(_) => "Tilde",
        }
        .to_string()
    }

    pub fn get_pos(&self) -> String {
        match self {
            Token::Int(_, pos) => pos.get_pos(),
            Token::Double(_, pos) => pos.get_pos(),
            Token::Boolean(_, pos) => pos.get_pos(),
            Token::String(_, pos) => pos.get_pos(),
            Token::Identifier(_, pos) => pos.get_pos(),
            Token::Plus(pos) => pos.get_pos(),
            Token::Minus(pos) => pos.get_pos(),
            Token::Multiply(pos) => pos.get_pos(),
            Token::Divide(pos) => pos.get_pos(),
            Token::Power(pos) => pos.get_pos(),
            Token::Modulus(pos) => pos.get_pos(),
            Token::Not(pos) => pos.get_pos(),
            Token::Equals(pos) => pos.get_pos(),
            Token::DoubleEquals(pos) => pos.get_pos(),
            Token::NotEquals(pos) => pos.get_pos(),
            Token::LessThan(pos) => pos.get_pos(),
            Token::LessThanEq(pos) => pos.get_pos(),
            Token::GreaterThan(pos) => pos.get_pos(),
            Token::GreaterThanEq(pos) => pos.get_pos(),
            Token::LParan(pos) => pos.get_pos(),
            Token::RParan(pos) => pos.get_pos(),
            Token::LBrace(pos) => pos.get_pos(),
            Token::RBrace(pos) => pos.get_pos(),
            Token::LSquare(pos) => pos.get_pos(),
            Token::RSquare(pos) => pos.get_pos(),
            Token::Comma(pos) => pos.get_pos(),
            Token::Colon(pos) => pos.get_pos(),
            Token::Semicolon(pos) => pos.get_pos(),
            Token::EOF(pos) => pos.get_pos(),
            Token::WhileK(pos) => pos.get_pos(),
            Token::IfK(pos) => pos.get_pos(),
            Token::ElifK(pos) => pos.get_pos(),
            Token::ElseK(pos) => pos.get_pos(),
            Token::FnK(pos) => pos.get_pos(),
            Token::CFnK(pos) => pos.get_pos(),
            Token::LetK(pos) => pos.get_pos(),
            Token::BreakK(pos) => pos.get_pos(),
            Token::ContinueK(pos) => pos.get_pos(),
            Token::ReturnK(pos) => pos.get_pos(),
            Token::And(pos) => pos.get_pos(),
            Token::Or(pos) => pos.get_pos(),
            Token::TildeDivide(pos) => pos.get_pos(),
            Token::PowerDivide(pos) => pos.get_pos(),
            Token::PlusEq(pos) => pos.get_pos(),
            Token::MinusEq(pos) => pos.get_pos(),
            Token::MultiplyEq(pos) => pos.get_pos(),
            Token::DivideEq(pos) => pos.get_pos(),
            Token::TildeDivideEq(pos) => pos.get_pos(),
            Token::PowerEq(pos) => pos.get_pos(),
            Token::PowerDivideEq(pos) => pos.get_pos(),
            Token::ModulusEq(pos) => pos.get_pos(),
            Token::Tilde(pos) => pos.get_pos(),
        }
    }
    pub fn set_pos(&mut self, pos_range: PosRange) -> Token {
        match self {
            Token::Int(_, pos) => pos.set_pos(pos_range),
            Token::Double(_, pos) => pos.set_pos(pos_range),
            Token::Boolean(_, pos) => pos.set_pos(pos_range),
            Token::String(_, pos) => pos.set_pos(pos_range),
            Token::Identifier(_, pos) => pos.set_pos(pos_range),
            Token::Plus(pos) => pos.set_pos(pos_range),
            Token::Minus(pos) => pos.set_pos(pos_range),
            Token::Multiply(pos) => pos.set_pos(pos_range),
            Token::Divide(pos) => pos.set_pos(pos_range),
            Token::Power(pos) => pos.set_pos(pos_range),
            Token::Modulus(pos) => pos.set_pos(pos_range),
            Token::Not(pos) => pos.set_pos(pos_range),
            Token::Equals(pos) => pos.set_pos(pos_range),
            Token::DoubleEquals(pos) => pos.set_pos(pos_range),
            Token::NotEquals(pos) => pos.set_pos(pos_range),
            Token::LessThan(pos) => pos.set_pos(pos_range),
            Token::LessThanEq(pos) => pos.set_pos(pos_range),
            Token::GreaterThan(pos) => pos.set_pos(pos_range),
            Token::GreaterThanEq(pos) => pos.set_pos(pos_range),
            Token::LParan(pos) => pos.set_pos(pos_range),
            Token::RParan(pos) => pos.set_pos(pos_range),
            Token::LBrace(pos) => pos.set_pos(pos_range),
            Token::RBrace(pos) => pos.set_pos(pos_range),
            Token::LSquare(pos) => pos.set_pos(pos_range),
            Token::RSquare(pos) => pos.set_pos(pos_range),
            Token::Comma(pos) => pos.set_pos(pos_range),
            Token::Colon(pos) => pos.set_pos(pos_range),
            Token::Semicolon(pos) => pos.set_pos(pos_range),
            Token::EOF(pos) => pos.set_pos(pos_range),
            Token::WhileK(pos) => pos.set_pos(pos_range),
            Token::IfK(pos) => pos.set_pos(pos_range),
            Token::ElifK(pos) => pos.set_pos(pos_range),
            Token::ElseK(pos) => pos.set_pos(pos_range),
            Token::FnK(pos) => pos.set_pos(pos_range),
            Token::CFnK(pos) => pos.set_pos(pos_range),
            Token::LetK(pos) => pos.set_pos(pos_range),
            Token::BreakK(pos) => pos.set_pos(pos_range),
            Token::ContinueK(pos) => pos.set_pos(pos_range),
            Token::ReturnK(pos) => pos.set_pos(pos_range),
            Token::And(pos) => pos.set_pos(pos_range),
            Token::Or(pos) => pos.set_pos(pos_range),
            Token::TildeDivide(pos) => pos.set_pos(pos_range),
            Token::PowerDivide(pos) => pos.set_pos(pos_range),
            Token::PlusEq(pos) => pos.set_pos(pos_range),
            Token::MinusEq(pos) => pos.set_pos(pos_range),
            Token::MultiplyEq(pos) => pos.set_pos(pos_range),
            Token::DivideEq(pos) => pos.set_pos(pos_range),
            Token::TildeDivideEq(pos) => pos.set_pos(pos_range),
            Token::PowerEq(pos) => pos.set_pos(pos_range),
            Token::PowerDivideEq(pos) => pos.set_pos(pos_range),
            Token::ModulusEq(pos) => pos.set_pos(pos_range),
            Token::Tilde(pos) => pos.set_pos(pos_range),
        };
        self.clone()
    }
}

impl Position {
    fn get_pos(&self) -> String {
        if self == &Self::empty() {
            "".to_string()
        } else {
            format!("[{}:{}]", self.line, self.column)
        }
    }

    // fn set_pos(&mut self, pos: Position) {
    //     self.line = pos.line;
    //     self.column = pos.column;
    // }
}

impl PosRange {
    fn get_pos(&self) -> String {
        if self == &Self::empty() {
            "".to_string()
        } else {
            if let Some(to) = self.to.clone() {
                format!("{}:{}", self.from.get_pos(), to.get_pos())
            } else {
                format!("{}", self.from.get_pos())
            }
        }
    }

    fn set_pos(&mut self, pos: PosRange) {
        self.from = pos.from;
        self.to = pos.to;
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
