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
            Token::Plus(_) => "+",
            Token::Minus(_) => "-",
            Token::Multiply(_) => "*",
            Token::Divide(_) => "/",
            Token::Power(_) => "^",
            Token::Modulus(_) => "%",
            Token::Not(_) => "!",
            Token::Equals(_) => "=",
            Token::DoubleEquals(_) => "==",
            Token::NotEquals(_) => "!=",
            Token::LessThan(_) => "<",
            Token::LessThanEq(_) => "<=",
            Token::GreaterThan(_) => ">",
            Token::GreaterThanEq(_) => ">=",
            Token::LParan(_) => "(",
            Token::RParan(_) => ")",
            Token::LBrace(_) => "{",
            Token::RBrace(_) => "}",
            Token::LSquare(_) => "[",
            Token::RSquare(_) => "]",
            Token::Comma(_) => ",",
            Token::Colon(_) => "colon",
            Token::Semicolon(_) => ";",
            Token::EOF(_) => "EOF",
            Token::WhileK(_) => "while",
            Token::IfK(_) => "if",
            Token::ElifK(_) => "elif",
            Token::ElseK(_) => "else",
            Token::FnK(_) => "fn",
            Token::CFnK(_) => "cfn",
            Token::LetK(_) => "let",
            Token::BreakK(_) => "break",
            Token::ContinueK(_) => "continue",
            Token::ReturnK(_) => "return",
            Token::And(_) => "&",
            Token::Or(_) => "|",
            Token::TildeDivide(_) => "~/",
            Token::PowerDivide(_) => "^/",
            Token::PlusEq(_) => "+=",
            Token::MinusEq(_) => "-=",
            Token::MultiplyEq(_) => "*=",
            Token::DivideEq(_) => "/=",
            Token::TildeDivideEq(_) => "~/=",
            Token::PowerEq(_) => "^=",
            Token::PowerDivideEq(_) => "^/=",
            Token::ModulusEq(_) => "%=",
            Token::Tilde(_) => "~",
        }
        .to_string()
    }

    pub fn get_pos(&self) -> String {
        match self {
            Token::Int(_, pos)
            | Token::Double(_, pos)
            | Token::Boolean(_, pos)
            | Token::String(_, pos)
            | Token::Identifier(_, pos)
            | Token::Plus(pos)
            | Token::Minus(pos)
            | Token::Multiply(pos)
            | Token::Divide(pos)
            | Token::Power(pos)
            | Token::Modulus(pos)
            | Token::Not(pos)
            | Token::Equals(pos)
            | Token::DoubleEquals(pos)
            | Token::NotEquals(pos)
            | Token::LessThan(pos)
            | Token::LessThanEq(pos)
            | Token::GreaterThan(pos)
            | Token::GreaterThanEq(pos)
            | Token::LParan(pos)
            | Token::RParan(pos)
            | Token::LBrace(pos)
            | Token::RBrace(pos)
            | Token::LSquare(pos)
            | Token::RSquare(pos)
            | Token::Comma(pos)
            | Token::Colon(pos)
            | Token::Semicolon(pos)
            | Token::EOF(pos)
            | Token::WhileK(pos)
            | Token::IfK(pos)
            | Token::ElifK(pos)
            | Token::ElseK(pos)
            | Token::FnK(pos)
            | Token::CFnK(pos)
            | Token::LetK(pos)
            | Token::BreakK(pos)
            | Token::ContinueK(pos)
            | Token::ReturnK(pos)
            | Token::And(pos)
            | Token::Or(pos)
            | Token::TildeDivide(pos)
            | Token::PowerDivide(pos)
            | Token::PlusEq(pos)
            | Token::MinusEq(pos)
            | Token::MultiplyEq(pos)
            | Token::DivideEq(pos)
            | Token::TildeDivideEq(pos)
            | Token::PowerEq(pos)
            | Token::PowerDivideEq(pos)
            | Token::ModulusEq(pos)
            | Token::Tilde(pos) => pos.get_pos(),
        }
    }
    pub fn set_pos(&mut self, pos_range: PosRange) -> Token {
        match self {
            Token::Int(_, pos)
            | Token::Double(_, pos)
            | Token::Boolean(_, pos)
            | Token::String(_, pos)
            | Token::Identifier(_, pos)
            | Token::Plus(pos)
            | Token::Minus(pos)
            | Token::Multiply(pos)
            | Token::Divide(pos)
            | Token::Power(pos)
            | Token::Modulus(pos)
            | Token::Not(pos)
            | Token::Equals(pos)
            | Token::DoubleEquals(pos)
            | Token::NotEquals(pos)
            | Token::LessThan(pos)
            | Token::LessThanEq(pos)
            | Token::GreaterThan(pos)
            | Token::GreaterThanEq(pos)
            | Token::LParan(pos)
            | Token::RParan(pos)
            | Token::LBrace(pos)
            | Token::RBrace(pos)
            | Token::LSquare(pos)
            | Token::RSquare(pos)
            | Token::Comma(pos)
            | Token::Colon(pos)
            | Token::Semicolon(pos)
            | Token::EOF(pos)
            | Token::WhileK(pos)
            | Token::IfK(pos)
            | Token::ElifK(pos)
            | Token::ElseK(pos)
            | Token::FnK(pos)
            | Token::CFnK(pos)
            | Token::LetK(pos)
            | Token::BreakK(pos)
            | Token::ContinueK(pos)
            | Token::ReturnK(pos)
            | Token::And(pos)
            | Token::Or(pos)
            | Token::TildeDivide(pos)
            | Token::PowerDivide(pos)
            | Token::PlusEq(pos)
            | Token::MinusEq(pos)
            | Token::MultiplyEq(pos)
            | Token::DivideEq(pos)
            | Token::TildeDivideEq(pos)
            | Token::PowerEq(pos)
            | Token::PowerDivideEq(pos)
            | Token::ModulusEq(pos)
            | Token::Tilde(pos) => pos.set_pos(pos_range),
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
