// TokenType
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i64),      // 10
    Double(f64),   // 3.14
    Plus,          // +
    Minus,         // -
    Multiply,      // *
    Divide,        // /
    Power,         // ^
    Modulus,       // %
    Not,           // !
    Equals,        // =
    DoubleEquals,  // ==
    NotEquals,     // !=
    LessThan,      // <
    LessThanEq,    // <=
    GreaterThan,   // >
    GreaterThanEq, // >=
    LParam,        // (
    RParam,        // )
    EOF,           // End of File
}
