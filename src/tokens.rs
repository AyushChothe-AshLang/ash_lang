// TokenType
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulus,
    LParam,
    RParam,
    Int(i64),
    Double(f64),
    EOF,
}
