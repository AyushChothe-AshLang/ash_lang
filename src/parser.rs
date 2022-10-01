use super::nodes::*;
use super::tokens::{Token, TokenType};
// Lexer
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn curr(&self) -> &Token {
        self.tokens.iter().nth(self.pos).unwrap()
    }

    fn next(&mut self) -> () {
        if self.pos < self.tokens.len() && self.curr().token_type != TokenType::EOF {
            self.pos += 1;
        } else {
            panic!("Reached EOF")
        }
    }

    fn eat(&mut self, token_type: TokenType) -> () {
        let tkn = self.curr().token_type.clone();
        if tkn == token_type {
            self.next();
        } else {
            panic!("Invalid Syntax: Expected {:?} found {:?}", token_type, tkn);
        }
    }

    pub fn parse(&mut self) -> Box<dyn Node> {
        self.expression()
    }

    fn expression(&mut self) -> Box<dyn Node> {
        let mut res = self.factor();

        while self.pos < self.tokens.len()
            && [TokenType::Plus, TokenType::Minus].contains(&self.curr().token_type)
        {
            if self.curr().token_type == TokenType::Plus {
                self.next();
                res = Box::new(BinaryOpNode::plus(res, self.factor()));
            } else if self.curr().token_type == TokenType::Minus {
                self.next();
                res = Box::new(BinaryOpNode::minus(res, self.factor()));
            }
        }

        res
    }
    fn factor(&mut self) -> Box<dyn Node> {
        let mut res = self.power();

        while self.pos < self.tokens.len()
            && [TokenType::Multiply, TokenType::Divide, TokenType::Modulus]
                .contains(&self.curr().token_type)
        {
            if self.curr().token_type == TokenType::Multiply {
                self.next();
                res = Box::new(BinaryOpNode::multiply(res, self.power()));
            } else if self.curr().token_type == TokenType::Divide {
                self.next();
                res = Box::new(BinaryOpNode::divide(res, self.power()));
            } else if self.curr().token_type == TokenType::Modulus {
                self.next();
                res = Box::new(BinaryOpNode::modulus(res, self.power()));
            }
        }

        res
    }
    fn power(&mut self) -> Box<dyn Node> {
        let mut res = self.atom();

        while self.pos < self.tokens.len() && [TokenType::Power].contains(&self.curr().token_type) {
            if self.curr().token_type == TokenType::Power {
                self.next();
                res = Box::new(BinaryOpNode::power(res, self.atom()));
            }
        }

        res
    }
    fn atom(&mut self) -> Box<dyn Node> {
        match self.curr().token_type {
            TokenType::LParam => {
                self.eat(TokenType::LParam);
                let res = self.expression();
                self.eat(TokenType::RParam);
                res
            }
            TokenType::Number => {
                let res = Box::new(NumberNode {
                    value: self.curr().value.parse::<f64>().unwrap(),
                });
                self.next();
                res
            }
            _ => panic!("Invalid Syntax!"),
        }
    }
}
