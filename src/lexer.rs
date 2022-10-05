use crate::utils::utils::is_keyword;

use super::tokens::Token;
// Lexer
pub struct Lexer {
    code: String,
    pos: usize,
    line: i32,
    col: i32,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer {
            code,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn curr(&self) -> char {
        self.code.chars().nth(self.pos).unwrap()
    }

    fn next(&mut self) -> () {
        if self.pos < self.code.len() {
            self.pos += 1;
            self.col += 1;
        } else {
            panic!("Reached EOF")
        }
    }
    fn lookahed(&self) -> char {
        if (self.pos + 1) < self.code.len() {
            self.code.chars().nth(self.pos + 1).unwrap()
        } else {
            panic!("Reached EOF")
        }
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

        if dots != 0 {
            Token::Double(num.parse::<f64>().unwrap())
        } else {
            Token::Int(num.parse::<i64>().unwrap())
        }
    }

    fn parse_identifier(&mut self) -> Token {
        let mut id = String::from("");
        while self.pos < self.code.len()
            && (('a'..='z').contains(&self.curr().to_ascii_lowercase())
                || ('0'..='9').contains(&self.curr().to_ascii_lowercase()))
        {
            id.push(self.curr());
            self.next();
        }
        if let Some(keyord) = is_keyword(id.as_str()) {
            keyord
        } else {
            Token::Identifier(id)
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.pos < self.code.len() {
            let c = self.curr().to_ascii_lowercase();
            match c {
                ' ' | '\n' | '\t' | '\r' => {
                    if c == '\n' {
                        self.line += 1;
                        self.col = 0;
                    }
                    self.next()
                }
                '0'..='9' | '.' => tokens.push(self.parse_number()),
                'a'..='z' => tokens.push(self.parse_identifier()),
                '+' => {
                    tokens.push(Token::Plus);
                    self.next()
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.next()
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    self.next()
                }
                '/' => {
                    tokens.push(Token::Divide);
                    self.next()
                }
                '^' => {
                    tokens.push(Token::Power);
                    self.next()
                }
                '%' => {
                    tokens.push(Token::Modulus);
                    self.next()
                }
                '=' => {
                    if self.lookahed() == '=' {
                        self.next();
                        tokens.push(Token::DoubleEquals);
                        self.next();
                    } else {
                        tokens.push(Token::Equals);
                        self.next()
                    }
                }
                '!' => {
                    if self.lookahed() == '=' {
                        self.next();
                        tokens.push(Token::NotEquals);
                        self.next();
                    } else {
                        tokens.push(Token::Not);
                        self.next()
                    }
                }
                '<' => {
                    if self.lookahed() == '=' {
                        self.next();
                        tokens.push(Token::LessThanEq);
                        self.next();
                    } else {
                        tokens.push(Token::LessThan);
                        self.next()
                    }
                }
                '>' => {
                    if self.lookahed() == '=' {
                        self.next();
                        tokens.push(Token::GreaterThanEq);
                        self.next();
                    } else {
                        tokens.push(Token::GreaterThan);
                        self.next()
                    }
                }
                '(' => {
                    tokens.push(Token::LParan);
                    self.next()
                }
                ')' => {
                    tokens.push(Token::RParan);
                    self.next()
                }
                '{' => {
                    tokens.push(Token::LBrace);
                    self.next()
                }
                '}' => {
                    tokens.push(Token::RBrace);
                    self.next()
                }
                '[' => {
                    tokens.push(Token::LSquare);
                    self.next()
                }
                ']' => {
                    tokens.push(Token::RSquare);
                    self.next()
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.next()
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.next()
                }
                _ => panic!("Invalid Token {}", c),
            }
        }
        // Add EOF
        tokens.push(Token::EOF);
        // Return tokens
        tokens
    }
}
