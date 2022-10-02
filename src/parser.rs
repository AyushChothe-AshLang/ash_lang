use super::nodes::*;
use super::tokens::Token;
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
        if self.pos < self.tokens.len() && self.curr() != &Token::EOF {
            self.pos += 1;
        } else {
            panic!("Reached EOF")
        }
    }

    fn eat(&mut self, token_type: &Token) -> () {
        let tkn = self.curr();
        if tkn == token_type {
            self.next();
        } else {
            panic!("Invalid Syntax: Expected {:?} found {:?}", token_type, tkn);
        }
    }

    pub fn parse(&mut self) -> Box<Node> {
        self.expression()
    }

    fn expression(&mut self) -> Box<Node> {
        let mut res = self.factor();

        while self.pos < self.tokens.len() && [Token::Plus, Token::Minus].contains(&self.curr()) {
            if self.curr() == &Token::Plus {
                self.next();
                res = Box::new(Node::BinaryOp(BinaryOpNode::plus(res, self.factor())));
            } else if self.curr() == &Token::Minus {
                self.next();
                res = Box::new(Node::BinaryOp(BinaryOpNode::minus(res, self.factor())));
            }
        }

        res
    }
    fn factor(&mut self) -> Box<Node> {
        let mut res = self.power();

        while self.pos < self.tokens.len()
            && [&Token::Multiply, &Token::Divide, &Token::Modulus].contains(&self.curr())
        {
            if self.curr() == &Token::Multiply {
                self.next();
                res = Box::new(Node::BinaryOp(BinaryOpNode::multiply(res, self.power())));
            } else if self.curr() == &Token::Divide {
                self.next();
                res = Box::new(Node::BinaryOp(BinaryOpNode::divide(res, self.power())));
            } else if self.curr() == &Token::Modulus {
                self.next();
                res = Box::new(Node::BinaryOp(BinaryOpNode::modulus(res, self.power())));
            }
        }

        res
    }
    fn power(&mut self) -> Box<Node> {
        let mut res = self.atom();

        while self.pos < self.tokens.len() && [&Token::Power].contains(&self.curr()) {
            if self.curr() == &Token::Power {
                self.next();
                res = Box::new(Node::BinaryOp(BinaryOpNode::power(res, self.atom())));
            }
        }

        res
    }
    fn atom(&mut self) -> Box<Node> {
        match self.curr() {
            Token::LParam => {
                self.eat(&Token::LParam);
                let res = self.expression();
                self.eat(&Token::RParam);
                res
            }
            Token::Int(num) => {
                let res = Box::new(Node::Int(IntNode { value: num.clone() }));
                self.next();
                res
            }
            Token::Double(num) => {
                let res = Box::new(Node::Double(DoubleNode { value: num.clone() }));
                self.next();
                res
            }
            _ => panic!("Invalid Syntax!"),
        }
    }
}
