// TokenType
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i64),           // 10
    Double(f64),        // 3.14
    Identifier(String), // Variables
    Plus,               // +
    Minus,              // -
    Multiply,           // *
    Divide,             // /
    Power,              // ^
    Modulus,            // %
    Not,                // !
    Equals,             // =
    DoubleEquals,       // ==
    NotEquals,          // !=
    LessThan,           // <
    LessThanEq,         // <=
    GreaterThan,        // >
    GreaterThanEq,      // >=
    LParan,             // (
    RParan,             // )
    LBrace,             // {
    RBrace,             // }
    LSquare,            // [
    RSquare,            // ]
    Comma,              // ;
    Semicolon,          // ;
    EOF,                // End of File
    //Keywords
    WhileK,    // while
    IfK,       // if
    ElifK,     // elif
    ElseK,     // else
    FnK,       // fn
    LetK,      // let
    BreakK,    // break
    ContinueK, // continue
    ReturnK,   // return
}
