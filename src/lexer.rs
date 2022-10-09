use crate::{
    tokens::{PosRange, Position},
    utils::utils::is_keyword,
};

use super::tokens::Token;
// Lexer
pub struct Lexer {
    code: String,
    pos: usize,
    line: i64,
    col: i64,
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

    fn get_pos(&self) -> Position {
        Position::new(self.line, self.col)
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
        let from = self.get_pos();

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
        let to = self.get_pos();

        if dots != 0 {
            Token::Double(num.parse::<f64>().unwrap(), PosRange::new(from, Some(to)))
        } else {
            Token::Int(num.parse::<i64>().unwrap(), PosRange::new(from, Some(to)))
        }
    }

    fn parse_identifier(&mut self) -> Token {
        let mut id = String::from("");
        let from = self.get_pos();
        while self.pos < self.code.len()
            && (('a'..='z').contains(&self.curr().to_ascii_lowercase())
                || ('0'..='9').contains(&self.curr().to_ascii_lowercase()))
        {
            id.push(self.curr());
            self.next();
        }
        let to = self.get_pos();
        if let Some(keyord) = is_keyword(id.as_str(), PosRange::new(from.clone(), Some(to.clone())))
        {
            keyord
        } else {
            Token::Identifier(id, PosRange::new(from, Some(to)))
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
                    tokens.push(Token::Plus(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '-' => {
                    tokens.push(Token::Minus(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '*' => {
                    tokens.push(Token::Multiply(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '/' => {
                    tokens.push(Token::Divide(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '^' => {
                    tokens.push(Token::Power(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '%' => {
                    tokens.push(Token::Modulus(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '=' => {
                    let from = self.get_pos();
                    if self.lookahed() == '=' {
                        self.next();
                        let to = self.get_pos();
                        tokens.push(Token::DoubleEquals(PosRange::new(from, Some(to))));
                        self.next();
                    } else {
                        tokens.push(Token::Equals(PosRange::new(from, None)));
                        self.next()
                    }
                }
                '!' => {
                    let from = self.get_pos();
                    if self.lookahed() == '=' {
                        self.next();

                        let to = self.get_pos();
                        tokens.push(Token::NotEquals(PosRange::new(from, Some(to))));
                        self.next();
                    } else {
                        tokens.push(Token::Not(PosRange::new(from, None)));
                        self.next()
                    }
                }
                '<' => {
                    let from = self.get_pos();
                    if self.lookahed() == '=' {
                        self.next();

                        let to = self.get_pos();
                        tokens.push(Token::LessThanEq(PosRange::new(from, Some(to))));
                        self.next();
                    } else {
                        tokens.push(Token::LessThan(PosRange::new(from, None)));
                        self.next()
                    }
                }
                '>' => {
                    let from = self.get_pos();
                    if self.lookahed() == '=' {
                        self.next();

                        let to = self.get_pos();
                        tokens.push(Token::GreaterThanEq(PosRange::new(from, Some(to))));
                        self.next();
                    } else {
                        tokens.push(Token::GreaterThan(PosRange::new(from, None)));
                        self.next()
                    }
                }
                '(' => {
                    tokens.push(Token::LParan(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                ')' => {
                    tokens.push(Token::RParan(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '{' => {
                    tokens.push(Token::LBrace(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '}' => {
                    tokens.push(Token::RBrace(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '[' => {
                    tokens.push(Token::LSquare(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                ']' => {
                    tokens.push(Token::RSquare(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                ',' => {
                    tokens.push(Token::Comma(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                ';' => {
                    tokens.push(Token::Semicolon(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                _ => panic!("Invalid Token [{}:{}]:'{}'", self.line, self.col, c),
            }
        }
        // Add EOF
        tokens.push(Token::EOF(PosRange::new(self.get_pos(), None)));
        // Return tokens
        tokens
    }
}
