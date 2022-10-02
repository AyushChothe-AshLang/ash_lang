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
        let res = self.equality();
        if self.curr() != &Token::EOF {
            panic!("Invalid Syntax");
        }
        res
    }

    fn equality(&mut self) -> Box<Node> {
        let mut res = self.comparison();

        if [Token::DoubleEquals, Token::NotEquals].contains(&self.curr()) {
            if self.curr() == &Token::DoubleEquals {
                self.next();
                res = Box::new(BinaryOpBooleanNode::deq(res, self.comparison()));
            } else if self.curr() == &Token::NotEquals {
                self.next();
                res = Box::new(BinaryOpBooleanNode::neq(res, self.comparison()));
            }
        }

        res
    }

    fn comparison(&mut self) -> Box<Node> {
        let mut res = self.expression();

        if [
            Token::LessThan,
            Token::LessThanEq,
            Token::GreaterThan,
            Token::GreaterThanEq,
        ]
        .contains(&self.curr())
        {
            if self.curr() == &Token::LessThan {
                self.next();
                res = Box::new(BinaryOpBooleanNode::lt(res, self.expression()));
            } else if self.curr() == &Token::LessThanEq {
                self.next();
                res = Box::new(BinaryOpBooleanNode::lte(res, self.expression()));
            } else if self.curr() == &Token::GreaterThan {
                self.next();
                res = Box::new(BinaryOpBooleanNode::gt(res, self.expression()));
            } else if self.curr() == &Token::GreaterThanEq {
                self.next();
                res = Box::new(BinaryOpBooleanNode::gte(res, self.expression()));
            }
        }

        res
    }

    fn expression(&mut self) -> Box<Node> {
        let mut res = self.factor();

        while self.pos < self.tokens.len() && [Token::Plus, Token::Minus].contains(&self.curr()) {
            if self.curr() == &Token::Plus {
                self.next();
                res = Box::new(BinaryOpNumberNode::plus(res, self.factor()));
            } else if self.curr() == &Token::Minus {
                self.next();
                res = Box::new(BinaryOpNumberNode::minus(res, self.factor()));
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
                res = Box::new(BinaryOpNumberNode::multiply(res, self.power()));
            } else if self.curr() == &Token::Divide {
                self.next();
                res = Box::new(BinaryOpNumberNode::divide(res, self.power()));
            } else if self.curr() == &Token::Modulus {
                self.next();
                res = Box::new(BinaryOpNumberNode::modulus(res, self.power()));
            }
        }

        res
    }
    fn power(&mut self) -> Box<Node> {
        let mut res = self.atom();

        while self.pos < self.tokens.len() && [&Token::Power].contains(&self.curr()) {
            if self.curr() == &Token::Power {
                self.next();
                res = Box::new(BinaryOpNumberNode::power(res, self.atom()));
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
