// Token
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

// TokenType
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulus,
    LParam,
    RParam,
    Number,
    EOF,
}
