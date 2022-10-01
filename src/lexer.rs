use super::tokens::{Token, TokenType};
// Lexer
pub struct Lexer {
    code: String,
    pos: usize,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer { code, pos: 0 }
    }

    fn curr(&self) -> char {
        self.code.chars().nth(self.pos).unwrap()
    }

    fn next(&mut self) -> () {
        if self.pos < self.code.len() {
            self.pos += 1;
        } else {
            panic!("Reached EOF")
        }
    }
    fn add_token(&mut self, token_type: TokenType, value: String) -> Token {
        let token = Token { token_type, value };
        self.next();
        token
    }

    fn parse_number(&mut self) -> Token {
        let mut num = String::from("");
        let mut dots = 0;
        while self.pos < self.code.len() && "0123456789.".contains(self.curr()) {
            if self.curr() == '.' {
                dots += 1;
            }
            if dots < 2 {
                num.push(self.curr());
                self.next();
            } else {
                break;
            }
        }
        Token {
            token_type: TokenType::Number,
            value: num,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.pos < self.code.len() {
            let c = self.curr();
            match c {
                ' ' | '\n' | '\t' | '\r' => self.next(),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    tokens.push(self.parse_number())
                }
                '+' => tokens.push(self.add_token(TokenType::Plus, String::from(c))),
                '-' => tokens.push(self.add_token(TokenType::Minus, String::from(c))),
                '*' => tokens.push(self.add_token(TokenType::Multiply, String::from(c))),
                '/' => tokens.push(self.add_token(TokenType::Divide, String::from(c))),
                '^' => tokens.push(self.add_token(TokenType::Power, String::from(c))),
                '%' => tokens.push(self.add_token(TokenType::Modulus, String::from(c))),
                '(' => tokens.push(self.add_token(TokenType::LParam, String::from(c))),
                ')' => tokens.push(self.add_token(TokenType::RParam, String::from(c))),
                _ => panic!("Invalid Token {}", c),
            }
        }
        // Add EOF
        tokens.push(Token {
            token_type: TokenType::EOF,
            value: String::from(""),
        });
        // Return tokens
        tokens
    }
}
