// TokenType
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i64, PosRange),      // 10
    Double(f64),             // 3.14
    Identifier(String),      // Variables
    Plus(PosRange),          // +
    Minus(PosRange),         // -
    Multiply(PosRange),      // *
    Divide(PosRange),        // /
    Power(PosRange),         // ^
    Modulus(PosRange),       // %
    Not(PosRange),           // !
    Equals(PosRange),        // =
    DoubleEquals(PosRange),  // ==
    NotEquals(PosRange),     // !=
    LessThan(PosRange),      // <
    LessThanEq(PosRange),    // <=
    GreaterThan(PosRange),   // >
    GreaterThanEq(PosRange), // >=
    LParan(PosRange),        // (
    RParan(PosRange),        // )
    LBrace(PosRange),        // {
    RBrace(PosRange),        // }
    LSquare(PosRange),       // [
    RSquare(PosRange),       // ]
    Comma(PosRange),         // ;
    Semicolon(PosRange),     // ;
    EOF(PosRange),           // End of File
    //Keywords
    WhileK(PosRange),    // while
    IfK(PosRange),       // if
    ElifK(PosRange),     // elif
    ElseK(PosRange),     // else
    FnK(PosRange),       // fn
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
    pub fn new(line: i64, column: i64) -> Self {
        Position { line, column }
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
