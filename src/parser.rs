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

    fn lookahead(&mut self) -> &Token {
        if self.pos + 1 < self.tokens.len() && self.curr() != &Token::EOF {
            self.tokens.iter().nth(self.pos + 1).unwrap()
        } else {
            panic!("Reached EOF")
        }
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
        let mut res = Vec::new();
        while self.curr() != &Token::EOF {
            res.push(self.primary_statements());
        }
        if self.curr() != &Token::EOF {
            panic!("Invalid Syntax");
        }
        Box::new(BlockStatementNode::new(res))
    }

    fn primary_statements(&mut self) -> Box<Node> {
        if let Token::Identifier(_) = self.curr() {
            if self.lookahead() == &Token::Equals {
                return self.assignment();
            } else if self.lookahead() == &Token::LParan {
                let res = self.function_call_statement();
                self.eat(&Token::Semicolon);
                return res;
            }
        }

        if self.curr() == &Token::FnK {
            return self.function_declaration_statement();
        }

        let res = self.equality();
        self.eat(&Token::Semicolon);

        return res;
    }

    fn function_declaration_statement(&mut self) -> Box<Node> {
        self.eat(&Token::FnK);

        //Parses Function name
        let id = self.identifier();

        // Parses Funtion params
        self.eat(&Token::LParan);
        let mut params = Vec::new();

        if let Token::Identifier(_) = self.curr() {
            let param = self.identifier();
            params.push(param);
        }

        while self.pos < self.tokens.len() && self.curr() != &Token::RParan {
            self.eat(&Token::Comma);
            let param = self.identifier();
            params.push(param);
        }
        self.eat(&Token::RParan);

        // Parses Function body
        let body = self.block_statement();

        Box::new(FunctionDeclarationNode::new(id, params, body))
    }

    fn block_statement(&mut self) -> Box<Node> {
        let mut value = Vec::new();
        self.eat(&Token::LBrace);
        while self.curr() != &Token::RBrace {
            value.push(self.primary_statements());
        }
        self.eat(&Token::RBrace);
        return Box::new(BlockStatementNode::new(value));
    }

    fn assignment(&mut self) -> Box<Node> {
        let id = self.identifier();
        self.eat(&Token::Equals);
        let value = self.equality();
        self.eat(&Token::Semicolon);
        Box::new(AssignmentNode::new(id, value))
    }

    fn identifier(&mut self) -> String {
        let res = match self.curr() {
            Token::Identifier(_id) => _id.clone(),
            _ => panic!("Expected Identifier"),
        };
        self.next();
        res
    }

    fn function_call_statement(&mut self) -> Box<Node> {
        let id = self.identifier();
        self.eat(&Token::LParan);

        let mut args = Vec::new();

        if self.curr() != &Token::RParan {
            args.push(self.equality());
        }

        while self.pos < self.tokens.len() && self.curr() != &Token::RParan {
            self.eat(&Token::Comma);
            args.push(self.equality());
        }

        self.eat(&Token::RParan);
        Box::new(FunctionCallNode::new(id, args))
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
            Token::LParan => {
                self.eat(&Token::LParan);
                let res = self.equality();
                self.eat(&Token::RParan);
                res
            }
            Token::Identifier(id) => {
                let res = Box::new(Node::Identifier(IdentifierNode { value: id.clone() }));
                self.next();
                res
            }
            Token::Plus => {
                self.next();
                let res = Box::new(UnaryNumberNode::plus(self.atom()));
                res
            }
            Token::Minus => {
                self.next();
                let res = Box::new(UnaryNumberNode::minus(self.atom()));
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
            _ => panic!("Invalid Syntax! {:?}", self.curr()),
        }
    }
}
