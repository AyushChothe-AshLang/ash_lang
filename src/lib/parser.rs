use crate::tokens::PosRange;
use crate::utils::utils::{get_assignment, get_assignment_from_token};

use super::nodes::*;
use super::tokens::Token;
use super::utils::utils::variant_eq;

type ParseResult = Result<Node, String>;

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

    fn contains_tkn(&self, vec: Vec<Token>, curr: &Token) -> bool {
        for t in vec.iter() {
            if variant_eq(t, curr) {
                return true;
            }
        }
        return false;
    }

    fn lookahead(&self) -> Result<&Token, String> {
        if self.pos + 1 < self.tokens.len()
            && !variant_eq(&Token::EOF(PosRange::empty()), self.curr())
        {
            Ok(self.tokens.iter().nth(self.pos + 1).unwrap())
        } else {
            Err(format!("Reached EOF").to_string())
        }
    }
    fn next(&mut self) -> Result<(), String> {
        if self.pos < self.tokens.len() && !variant_eq(&Token::EOF(PosRange::empty()), self.curr())
        {
            self.pos += 1;
            Ok(())
        } else {
            Err(format!("Reached EOF").to_string())
        }
    }

    fn eat(&mut self, token_type: &Token) -> Result<(), String> {
        let tkn = self.curr();
        if variant_eq(tkn, token_type) {
            self.next()?;
            Ok(())
        } else {
            Err(format!(
                "Invalid Syntax {}: Expected {} found {}",
                self.curr().get_pos(),
                token_type,
                tkn
            )
            .to_string())
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut res = Vec::new();
        while !variant_eq(self.curr(), &Token::EOF(PosRange::empty())) {
            res.push(self.primary_statements()?);
        }
        if !variant_eq(self.curr(), &Token::EOF(PosRange::empty())) {
            return Err(format!("Invalid Syntax {}", self.curr().get_pos()).to_string());
        }
        // Invoke Main
        res.push(FunctionCallNode::new(String::from("main"), Vec::new()));

        Ok(BlockStatementNode::new(res))
    }

    fn primary_statements(&mut self) -> ParseResult {
        match self.curr() {
            Token::Identifier(_, _) => {
                // Assignment or FunctionCall
                match self.lookahead()? {
                    _ if self.contains_tkn(get_assignment(), self.lookahead()?) => {
                        return self.assignment();
                    }
                    Token::LParan(_) => {
                        let res = self.function_call_statement();
                        self.eat(&Token::Semicolon(PosRange::empty()))?;
                        return res;
                    }
                    _ => {
                        let res = self.logical_and_or()?;
                        self.eat(&Token::Semicolon(PosRange::empty()))?;
                        return Ok(res);
                    }
                }
            }
            Token::FnK(_) => {
                // Function Declaration
                return self.function_declaration_statement(false);
            }
            Token::CFnK(_) => {
                // Function Declaration
                return self.function_declaration_statement(true);
            }
            Token::LetK(_) => {
                // Variable Declaration
                return self.multi_declaration_node();
            }
            _ => Err(self.panic_invalid_syntax(
                "Only (Variable and Function) Declaration are allowed in Global Scope",
            )),
        }
    }

    fn control_flow_statement(&mut self) -> ParseResult {
        match self.curr() {
            Token::Identifier(_, _) => {
                // Assignment or FunctionCall
                match self.lookahead()? {
                    _ if self.contains_tkn(get_assignment(), self.lookahead()?) => {
                        return self.assignment();
                    }
                    Token::LParan(_) => {
                        let res = self.function_call_statement();
                        self.eat(&Token::Semicolon(PosRange::empty()))?;
                        return res;
                    }
                    _ => {
                        let res = self.logical_and_or();
                        self.eat(&Token::Semicolon(PosRange::empty()))?;
                        return res;
                    }
                }
            }
            Token::FnK(_) => {
                // Function Declaration
                return self.function_declaration_statement(false);
            }
            Token::CFnK(_) => {
                // Function Declaration
                return self.function_declaration_statement(true);
            }
            Token::LetK(_) => {
                // Variable Declaration
                return self.multi_declaration_node();
            }
            Token::LBrace(_) => {
                // Block Statement
                return self.block_statement();
            }
            Token::IfK(_) => {
                // If Statement
                return self.if_statement();
            }
            Token::WhileK(_) => {
                // While Loop
                return self.while_loop_statement();
            }
            Token::ReturnK(_) => {
                // Return Statement
                return self.return_statement();
            }
            _ => {
                let res = self.logical_and_or();
                self.eat(&Token::Semicolon(PosRange::empty()))?;
                res
            }
        }
    }

    fn return_statement(&mut self) -> ParseResult {
        self.eat(&Token::ReturnK(PosRange::empty()))?;
        let res = self.logical_and_or()?;
        self.eat(&Token::Semicolon(PosRange::empty()))?;
        Ok(ReturnNode::new(Box::new(res)))
    }

    fn if_statement(&mut self) -> ParseResult {
        // Parse if condition
        self.eat(&Token::IfK(PosRange::empty()))?;
        self.eat(&Token::LParan(PosRange::empty()))?;
        let condition = Box::new(self.logical_and_or()?);
        self.eat(&Token::RParan(PosRange::empty()))?;
        let true_block = Box::new(self.block_statement()?);

        // Parse elif statements
        let mut elif_blocks: Vec<Node> = Vec::new();
        while self.pos < self.tokens.len()
            && variant_eq(self.curr(), &Token::ElifK(PosRange::empty()))
        {
            self.eat(&&Token::ElifK(PosRange::empty()))?;
            self.eat(&&Token::LParan(PosRange::empty()))?;

            let condition = Box::new(self.logical_and_or()?);
            self.eat(&Token::RParan(PosRange::empty()))?;
            let true_block = Box::new(self.block_statement()?);
            elif_blocks.push(ElifStatementNode::new(condition, true_block));
        }

        // Parse else block
        let mut else_block = None;
        if variant_eq(self.curr(), &Token::ElseK(PosRange::empty())) {
            self.eat(&&Token::ElseK(PosRange::empty()))?;
            else_block = Some(Box::new(self.block_statement()?));
        }
        Ok(IfStatementNode::new(
            condition,
            true_block,
            elif_blocks,
            else_block,
        ))
    }

    // While Loop
    fn while_loop_statement(&mut self) -> ParseResult {
        self.eat(&Token::WhileK(PosRange::empty()))?;

        self.eat(&Token::LParan(PosRange::empty()))?;
        let condition = self.logical_and_or()?;
        self.eat(&Token::RParan(PosRange::empty()))?;

        let body = self.block_statement()?;

        Ok(WhileLoopNode::new(Box::new(condition), Box::new(body)))
    }

    // Parses a Function Declaration Statement
    fn function_declaration_statement(&mut self, is_cached: bool) -> ParseResult {
        if is_cached {
            self.eat(&Token::CFnK(PosRange::empty()))?;
        } else {
            self.eat(&Token::FnK(PosRange::empty()))?;
        }

        // Parses Function name
        let id = self.identifier()?;

        // Parses Function params
        self.eat(&Token::LParan(PosRange::empty()))?;
        let mut params = Vec::new();

        if let Token::Identifier(_, _) = self.curr() {
            let param = self.identifier()?;
            params.push(param);
        }

        while self.pos < self.tokens.len()
            && !variant_eq(self.curr(), &Token::RParan(PosRange::empty()))
        {
            self.eat(&Token::Comma(PosRange::empty()))?;
            let param = self.identifier()?;
            params.push(param);
        }
        self.eat(&Token::RParan(PosRange::empty()))?;

        // Parses Function body
        let body = Box::new(self.block_statement()?);

        if is_cached {
            Ok(FunctionDeclarationNode::new_cfn(id, params, body))
        } else {
            Ok(FunctionDeclarationNode::new_fn(id, params, body))
        }
    }

    fn block_statement(&mut self) -> ParseResult {
        let mut value = Vec::new();
        self.eat(&Token::LBrace(PosRange::empty()))?;
        while !variant_eq(self.curr(), &Token::RBrace(PosRange::empty())) {
            value.push(self.control_flow_statement()?);
        }
        self.eat(&Token::RBrace(PosRange::empty()))?;
        Ok(BlockStatementNode::new(value))
    }

    fn multi_declaration_node(&mut self) -> ParseResult {
        self.eat(&Token::LetK(PosRange::empty()))?;

        let mut declarations = Vec::new();

        let id = self.identifier()?;
        self.eat(&Token::Equals(PosRange::empty()))?;
        let value = Box::new(self.logical_and_or()?);

        declarations.push(DeclarationNode::new(id, value));

        while self.pos < self.tokens.len()
            && variant_eq(self.curr(), &Token::Comma(PosRange::empty()))
        {
            self.eat(&Token::Comma(PosRange::empty()))?;
            let id = self.identifier()?;
            self.eat(&Token::Equals(PosRange::empty()))?;
            let value = Box::new(self.logical_and_or()?);
            declarations.push(DeclarationNode::new(id, value));
        }

        self.eat(&Token::Semicolon(PosRange::empty()))?;
        Ok(MultiDeclarationNode::new(declarations))
    }

    fn assignment(&mut self) -> ParseResult {
        let id = self.identifier()?;
        let assign_type;
        if self.contains_tkn(get_assignment(), self.curr()) {
            assign_type = get_assignment_from_token(self.curr());
            self.next()?;
        } else {
            return Err(self.panic_invalid_syntax("Invalid Assignment"));
        }

        let value = Box::new(self.logical_and_or()?);
        self.eat(&Token::Semicolon(PosRange::empty()))?;
        Ok(AssignmentNode::new(id, value, assign_type))
    }

    fn identifier(&mut self) -> Result<String, String> {
        let res = match self.curr() {
            Token::Identifier(_id, _) => _id.clone(),
            _ => return Err(format!("Expected Identifier").to_string()),
        };
        self.next()?;
        Ok(res)
    }

    fn function_call_statement(&mut self) -> ParseResult {
        let id = self.identifier()?;
        self.eat(&Token::LParan(PosRange::empty()))?;

        let mut args = Vec::new();

        if !variant_eq(self.curr(), &Token::RParan(PosRange::empty())) {
            args.push(self.logical_and_or()?);
        }

        while self.pos < self.tokens.len()
            && !variant_eq(self.curr(), &Token::RParan(PosRange::empty()))
        {
            self.eat(&Token::Comma(PosRange::empty()))?;
            args.push(self.logical_and_or()?);
        }

        self.eat(&Token::RParan(PosRange::empty()))?;
        Ok(FunctionCallNode::new(id, args))
    }

    fn list_literal(&mut self) -> ParseResult {
        let mut elements = Vec::new();

        self.eat(&Token::LSquare(PosRange::empty()))?;

        if !variant_eq(self.curr(), &Token::RSquare(PosRange::empty())) {
            let elem = self.logical_and_or()?;
            elements.push(elem);
            if variant_eq(self.curr(), &Token::Comma(PosRange::empty())) {
                while self.pos < self.tokens.len()
                    && !variant_eq(self.curr(), &Token::RSquare(PosRange::empty()))
                {
                    self.eat(&&Token::Comma(PosRange::empty()))?;
                    let elem = self.logical_and_or()?;
                    elements.push(elem);
                }
            }
        }

        self.eat(&Token::RSquare(PosRange::empty()))?;

        Ok(Node::List(ListNode { elements }))
    }

    // Parses Logical & |
    fn logical_and_or(&mut self) -> ParseResult {
        let mut res = self.equality()?;

        if self.contains_tkn(
            vec![Token::And(PosRange::empty()), Token::Or(PosRange::empty())],
            self.curr(),
        ) {
            if variant_eq(self.curr(), &Token::And(PosRange::empty())) {
                self.next()?;
                res = BinaryOpBooleanNode::and(Box::new(res), Box::new(self.equality()?));
            } else if variant_eq(self.curr(), &Token::Or(PosRange::empty())) {
                self.next()?;
                res = BinaryOpBooleanNode::or(Box::new(res), Box::new(self.equality()?));
            }
        }

        Ok(res)
    }

    fn equality(&mut self) -> ParseResult {
        let mut res = self.comparison()?;

        if self.contains_tkn(
            vec![
                Token::DoubleEquals(PosRange::empty()),
                Token::NotEquals(PosRange::empty()),
            ],
            self.curr(),
        ) {
            if variant_eq(self.curr(), &Token::DoubleEquals(PosRange::empty())) {
                self.next()?;
                res = BinaryOpBooleanNode::deq(Box::new(res), Box::new(self.comparison()?));
            } else if variant_eq(self.curr(), &Token::NotEquals(PosRange::empty())) {
                self.next()?;
                res = BinaryOpBooleanNode::neq(Box::new(res), Box::new(self.comparison()?));
            }
        }

        Ok(res)
    }

    fn comparison(&mut self) -> ParseResult {
        let mut res = self.expression()?;

        if self.contains_tkn(
            vec![
                Token::LessThan(PosRange::empty()),
                Token::LessThanEq(PosRange::empty()),
                Token::GreaterThan(PosRange::empty()),
                Token::GreaterThanEq(PosRange::empty()),
            ],
            self.curr(),
        ) {
            if variant_eq(self.curr(), &Token::LessThan(PosRange::empty())) {
                self.next()?;
                res = BinaryOpBooleanNode::lt(Box::new(res), Box::new(self.expression()?));
            } else if variant_eq(self.curr(), &Token::LessThanEq(PosRange::empty())) {
                self.next()?;
                res = BinaryOpBooleanNode::lte(Box::new(res), Box::new(self.expression()?));
            } else if variant_eq(self.curr(), &Token::GreaterThan(PosRange::empty())) {
                self.next()?;
                res = BinaryOpBooleanNode::gt(Box::new(res), Box::new(self.expression()?));
            } else if variant_eq(self.curr(), &Token::GreaterThanEq(PosRange::empty())) {
                self.next()?;
                res = BinaryOpBooleanNode::gte(Box::new(res), Box::new(self.expression()?));
            }
        }

        Ok(res)
    }

    fn expression(&mut self) -> ParseResult {
        let mut res = self.factor()?;

        while self.pos < self.tokens.len()
            && self.contains_tkn(
                vec![
                    Token::Plus(PosRange::empty()),
                    Token::Minus(PosRange::empty()),
                ],
                self.curr(),
            )
        {
            if variant_eq(self.curr(), &Token::Plus(PosRange::empty())) {
                self.next()?;
                res = BinaryOpNumberNode::plus(Box::new(res), Box::new(self.factor()?));
            } else if variant_eq(self.curr(), &Token::Minus(PosRange::empty())) {
                self.next()?;
                res = BinaryOpNumberNode::minus(Box::new(res), Box::new(self.factor()?));
            }
        }

        Ok(res)
    }
    fn factor(&mut self) -> ParseResult {
        let mut res = self.power()?;

        while self.pos < self.tokens.len()
            && self.contains_tkn(
                vec![
                    Token::Multiply(PosRange::empty()),
                    Token::Divide(PosRange::empty()),
                    Token::TildeDivide(PosRange::empty()),
                    Token::PowerDivide(PosRange::empty()),
                    Token::Modulus(PosRange::empty()),
                ],
                self.curr(),
            )
        {
            if variant_eq(self.curr(), &Token::Multiply(PosRange::empty())) {
                self.next()?;
                res = BinaryOpNumberNode::multiply(Box::new(res), Box::new(self.power()?));
            } else if variant_eq(self.curr(), &Token::Divide(PosRange::empty())) {
                self.next()?;
                res = BinaryOpNumberNode::divide(Box::new(res), Box::new(self.power()?));
            } else if variant_eq(self.curr(), &Token::TildeDivide(PosRange::empty())) {
                self.next()?;
                res = BinaryOpNumberNode::tilde_divide(Box::new(res), Box::new(self.power()?));
            } else if variant_eq(self.curr(), &Token::PowerDivide(PosRange::empty())) {
                self.next()?;
                res = BinaryOpNumberNode::power_divide(Box::new(res), Box::new(self.power()?));
            } else if variant_eq(self.curr(), &Token::Modulus(PosRange::empty())) {
                self.next()?;
                res = BinaryOpNumberNode::modulus(Box::new(res), Box::new(self.power()?));
            }
        }

        Ok(res)
    }
    fn power(&mut self) -> ParseResult {
        let mut res = self.atom()?;

        while self.pos < self.tokens.len()
            && self.contains_tkn(vec![Token::Power(PosRange::empty())], self.curr())
        {
            if variant_eq(self.curr(), &Token::Power(PosRange::empty())) {
                self.next()?;
                res = BinaryOpNumberNode::power(Box::new(res), Box::new(self.atom()?));
            }
        }

        Ok(res)
    }
    fn atom(&mut self) -> ParseResult {
        match self.curr() {
            Token::LParan(_) => {
                self.eat(&Token::LParan(PosRange::empty()))?;
                let res = self.logical_and_or()?;
                self.eat(&Token::RParan(PosRange::empty()))?;
                Ok(res)
            }
            Token::LSquare(_) => self.list_literal(),
            Token::Identifier(id, _) => {
                let res;
                if variant_eq(self.lookahead()?, &Token::LParan(PosRange::empty())) {
                    res = self.function_call_statement()?;
                } else {
                    res = Node::Identifier(IdentifierNode { value: id.clone() });
                    self.next()?;
                }
                Ok(res)
            }
            Token::Plus(_) => {
                self.next()?;
                let res = UnaryNumberNode::plus(Box::new(self.atom()?));
                Ok(res)
            }
            Token::Minus(_) => {
                self.next()?;
                let res = UnaryNumberNode::minus(Box::new(self.atom()?));
                Ok(res)
            }
            Token::Int(num, _) => {
                let res = Node::Int(IntNode {
                    value: num.to_owned(),
                });
                self.next()?;
                Ok(res)
            }
            Token::Double(num, _) => {
                let res = Node::Double(DoubleNode {
                    value: num.to_owned(),
                });
                self.next()?;
                Ok(res)
            }
            Token::Boolean(_bool, _) => {
                let res = Node::Boolean(BooleanNode {
                    value: _bool.to_owned(),
                });
                self.next()?;
                Ok(res)
            }
            Token::String(_str, _) => {
                let res = Node::String(StringNode {
                    value: _str.to_owned(),
                });
                self.next()?;
                Ok(res)
            }

            _ => Err(format!(
                "Invalid Syntax {}: {}",
                self.curr().get_pos(),
                self.curr().get_name()
            )
            .to_string()),
        }
    }
}
