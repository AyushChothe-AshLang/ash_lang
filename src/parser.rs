use crate::tokens::PosRange;

use super::nodes::*;
use super::tokens::Token;
use super::utils::utils::variant_eq;

// Parser
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn panic_invalid_syntax(&self, message: &str) -> String {
        format!(
            "Invalid Syntax {}: '{}' {message}",
            self.curr().get_pos(),
            self.curr().get_name()
        )
    }

    fn curr(&self) -> &Token {
        self.tokens.iter().nth(self.pos).unwrap()
    }

    fn contains_tkn(&self, vec: Vec<&Token>, curr: &Token) -> bool {
        for t in vec.iter() {
            if variant_eq(*t, curr) {
                return true;
            }
        }
        return false;
    }

    fn lookahead(&self) -> &Token {
        if self.pos + 1 < self.tokens.len()
            && !variant_eq(&Token::EOF(PosRange::empty()), self.curr())
        {
            self.tokens.iter().nth(self.pos + 1).unwrap()
        } else {
            panic!("Reached EOF")
        }
    }
    fn next(&mut self) -> () {
        if self.pos < self.tokens.len() && !variant_eq(&Token::EOF(PosRange::empty()), self.curr())
        {
            self.pos += 1;
        } else {
            panic!("Reached EOF")
        }
    }

    fn eat(&mut self, token_type: &Token) -> () {
        let tkn = self.curr();
        if variant_eq(tkn, token_type) {
            self.next();
        } else {
            panic!(
                "Invalid Syntax {}: Expected {} found {}",
                self.curr().get_pos(),
                token_type,
                tkn
            );
        }
    }

    pub fn parse(&mut self) -> Box<Node> {
        let mut res = Vec::new();
        while !variant_eq(self.curr(), &Token::EOF(PosRange::empty())) {
            res.push(self.primary_statements());
        }
        if !variant_eq(self.curr(), &Token::EOF(PosRange::empty())) {
            panic!("Invalid Syntax {}", self.curr().get_pos());
        }
        // Invoke Main
        res.push(Box::new(FunctionCallNode::new(
            String::from("main"),
            Vec::new(),
        )));

        Box::new(BlockStatementNode::new(res))
    }

    fn primary_statements(&mut self) -> Box<Node> {
        match self.curr() {
            Token::Identifier(_, _) => {
                // Assignment or FunctionCall
                match self.lookahead() {
                    Token::Equals(_) => {
                        return self.assignment();
                    }
                    Token::LParan(_) => {
                        let res = self.function_call_statement();
                        self.eat(&Token::Semicolon(PosRange::empty()));
                        return res;
                    }
                    _ => {
                        let res = self.logical_and_or();
                        self.eat(&Token::Semicolon(PosRange::empty()));
                        return res;
                    }
                }
            }
            Token::FnK(_) => {
                // Function Declaration
                return self.function_declaration_statement();
            }
            Token::LetK(_) => {
                // Variable Declaration
                return self.multi_declaration_node();
            }
            // TODO: Remove This
            Token::LBrace(_) => {
                return self.block_statement();
            }
            _ => panic!(
                "{}",
                self.panic_invalid_syntax(
                    "Only (Variable and Function) Declaration are allowed in Global Scope",
                )
            ),
        }
    }

    // Parses a Function Declaration Statement
    fn function_declaration_statement(&mut self) -> Box<Node> {
        self.eat(&Token::FnK(PosRange::empty()));

        // Parses Function name
        let id = self.identifier();

        // Parses Funtion params
        self.eat(&Token::LParan(PosRange::empty()));
        let mut params = Vec::new();

        if let Token::Identifier(_, _) = self.curr() {
            let param = self.identifier();
            params.push(param);
        }

        while self.pos < self.tokens.len()
            && !variant_eq(self.curr(), &Token::RParan(PosRange::empty()))
        {
            self.eat(&Token::Comma(PosRange::empty()));
            let param = self.identifier();
            params.push(param);
        }
        self.eat(&Token::RParan(PosRange::empty()));

        // Parses Function body
        let body = self.block_statement();

        Box::new(FunctionDeclarationNode::new(id, params, body))
    }

    fn block_statement(&mut self) -> Box<Node> {
        let mut value = Vec::new();
        self.eat(&Token::LBrace(PosRange::empty()));
        while !variant_eq(self.curr(), &Token::RBrace(PosRange::empty())) {
            value.push(self.primary_statements());
        }
        self.eat(&Token::RBrace(PosRange::empty()));
        return Box::new(BlockStatementNode::new(value));
    }

    fn multi_declaration_node(&mut self) -> Box<Node> {
        self.eat(&Token::LetK(PosRange::empty()));

        let mut declarations = Vec::new();

        let id = self.identifier();
        self.eat(&Token::Equals(PosRange::empty()));
        let value = self.logical_and_or();

        declarations.push(Box::new(DeclarationNode::new(id, value)));

        while self.pos < self.tokens.len()
            && variant_eq(self.curr(), &Token::Comma(PosRange::empty()))
        {
            self.eat(&Token::Comma(PosRange::empty()));
            let id = self.identifier();
            self.eat(&Token::Equals(PosRange::empty()));
            let value = self.logical_and_or();
            declarations.push(Box::new(DeclarationNode::new(id, value)));
        }

        self.eat(&Token::Semicolon(PosRange::empty()));
        Box::new(MultiDeclarationNode::new(declarations))
    }

    fn assignment(&mut self) -> Box<Node> {
        let id = self.identifier();
        self.eat(&Token::Equals(PosRange::empty()));
        let value = self.logical_and_or();
        self.eat(&Token::Semicolon(PosRange::empty()));
        Box::new(AssignmentNode::new(id, value))
    }

    fn identifier(&mut self) -> String {
        let res = match self.curr() {
            Token::Identifier(_id, _) => _id.clone(),
            _ => panic!("Expected Identifier"),
        };
        self.next();
        res
    }

    fn function_call_statement(&mut self) -> Box<Node> {
        let id = self.identifier();
        self.eat(&Token::LParan(PosRange::empty()));

        let mut args = Vec::new();

        if !variant_eq(self.curr(), &Token::RParan(PosRange::empty())) {
            args.push(self.logical_and_or());
        }

        while self.pos < self.tokens.len()
            && !variant_eq(self.curr(), &Token::RParan(PosRange::empty()))
        {
            self.eat(&Token::Comma(PosRange::empty()));
            args.push(self.logical_and_or());
        }

        self.eat(&Token::RParan(PosRange::empty()));
        Box::new(FunctionCallNode::new(id, args))
    }

    /// Parses Logical & |

    fn logical_and_or(&mut self) -> Box<Node> {
        let mut res = self.equality();

        if self.contains_tkn(
            vec![
                &Token::And(PosRange::empty()),
                &Token::Or(PosRange::empty()),
            ],
            self.curr(),
        ) {
            if variant_eq(self.curr(), &Token::And(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpBooleanNode::and(res, self.equality()));
            } else if variant_eq(self.curr(), &Token::Or(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpBooleanNode::or(res, self.equality()));
            }
        }

        res
    }

    fn equality(&mut self) -> Box<Node> {
        let mut res = self.comparison();

        if self.contains_tkn(
            vec![
                &Token::DoubleEquals(PosRange::empty()),
                &Token::NotEquals(PosRange::empty()),
            ],
            self.curr(),
        ) {
            if variant_eq(self.curr(), &Token::DoubleEquals(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpBooleanNode::deq(res, self.comparison()));
            } else if variant_eq(self.curr(), &Token::NotEquals(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpBooleanNode::neq(res, self.comparison()));
            }
        }

        res
    }

    fn comparison(&mut self) -> Box<Node> {
        let mut res = self.expression();

        if self.contains_tkn(
            vec![
                &Token::LessThan(PosRange::empty()),
                &Token::LessThanEq(PosRange::empty()),
                &Token::GreaterThan(PosRange::empty()),
                &Token::GreaterThanEq(PosRange::empty()),
            ],
            self.curr(),
        ) {
            if variant_eq(self.curr(), &Token::LessThan(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpBooleanNode::lt(res, self.expression()));
            } else if variant_eq(self.curr(), &Token::LessThanEq(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpBooleanNode::lte(res, self.expression()));
            } else if variant_eq(self.curr(), &Token::GreaterThan(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpBooleanNode::gt(res, self.expression()));
            } else if variant_eq(self.curr(), &Token::GreaterThanEq(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpBooleanNode::gte(res, self.expression()));
            }
        }

        res
    }

    fn expression(&mut self) -> Box<Node> {
        let mut res = self.factor();

        while self.pos < self.tokens.len()
            && self.contains_tkn(
                vec![
                    &Token::Plus(PosRange::empty()),
                    &Token::Minus(PosRange::empty()),
                ],
                self.curr(),
            )
        {
            if variant_eq(self.curr(), &Token::Plus(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpNumberNode::plus(res, self.factor()));
            } else if variant_eq(self.curr(), &Token::Minus(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpNumberNode::minus(res, self.factor()));
            }
        }

        res
    }
    fn factor(&mut self) -> Box<Node> {
        let mut res = self.power();

        while self.pos < self.tokens.len()
            && self.contains_tkn(
                vec![
                    &Token::Multiply(PosRange::empty()),
                    &Token::Divide(PosRange::empty()),
                    &Token::Modulus(PosRange::empty()),
                ],
                self.curr(),
            )
        {
            if variant_eq(self.curr(), &Token::Multiply(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpNumberNode::multiply(res, self.power()));
            } else if variant_eq(self.curr(), &Token::Divide(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpNumberNode::divide(res, self.power()));
            } else if variant_eq(self.curr(), &Token::Modulus(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpNumberNode::modulus(res, self.power()));
            }
        }

        res
    }
    fn power(&mut self) -> Box<Node> {
        let mut res = self.atom();

        while self.pos < self.tokens.len()
            && self.contains_tkn(vec![&Token::Power(PosRange::empty())], self.curr())
        {
            if variant_eq(self.curr(), &Token::Power(PosRange::empty())) {
                self.next();
                res = Box::new(BinaryOpNumberNode::power(res, self.atom()));
            }
        }

        res
    }
    fn atom(&mut self) -> Box<Node> {
        match self.curr() {
            Token::LParan(_) => {
                self.eat(&Token::LParan(PosRange::empty()));
                let res = self.logical_and_or();
                self.eat(&Token::RParan(PosRange::empty()));
                res
            }
            Token::Identifier(id, _) => {
                let res;
                if variant_eq(self.lookahead(), &Token::LParan(PosRange::empty())) {
                    res = self.function_call_statement();
                } else {
                    res = Box::new(Node::Identifier(IdentifierNode { value: id.clone() }));
                    self.next();
                }
                res
            }
            Token::Plus(_) => {
                self.next();
                let res = Box::new(UnaryNumberNode::plus(self.atom()));
                res
            }
            Token::Minus(_) => {
                self.next();
                let res = Box::new(UnaryNumberNode::minus(self.atom()));
                res
            }
            Token::Int(num, _) => {
                let res = Box::new(Node::Int(IntNode {
                    value: num.to_owned(),
                }));
                self.next();
                res
            }
            Token::Double(num, _) => {
                let res = Box::new(Node::Double(DoubleNode {
                    value: num.to_owned(),
                }));
                self.next();
                res
            }
            Token::Boolean(_bool, _) => {
                let res = Box::new(Node::Boolean(BooleanNode {
                    value: _bool.to_owned(),
                }));
                self.next();
                res
            }
            _ => panic!(
                "Invalid Syntax {}: {}",
                self.curr().get_pos(),
                self.curr().get_name()
            ),
        }
    }
}
