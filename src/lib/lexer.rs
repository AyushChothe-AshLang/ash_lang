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
        Position::from(self.line, self.col)
    }

    fn add_double_char_token(
        &mut self,
        first_token: &mut Token,
        second_token: &mut Token,
        chars: [char; 2],
    ) -> Token {
        let from = self.get_pos();
        let mut tkn = first_token.set_pos(PosRange::new(from.clone(), None));
        self.next();
        if self.pos < self.code.len() && self.curr() == chars[1] {
            let to = self.get_pos();
            tkn = second_token.set_pos(PosRange::new(from, Some(to)));
            self.next();
        }
        tkn
    }
    fn add_triple_char_token(
        &mut self,
        first_token: &mut Token,
        second_token: &mut Token,
        third_token: &mut Token,
        chars: [char; 3],
    ) -> Token {
        let from = self.get_pos();
        let mut tkn = first_token.set_pos(PosRange::new(from.clone(), None));
        self.next();

        if self.pos < self.code.len() && self.curr() == chars[1] {
            let to = self.get_pos();
            tkn = second_token.set_pos(PosRange::new(from.clone(), Some(to)));
            self.next();

            if self.pos < self.code.len() && self.curr() == chars[2] {
                let to = self.get_pos();
                tkn = third_token.set_pos(PosRange::new(from, Some(to)));
                self.next();
            }
        }
        tkn
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
            ' '
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
        } else if id == "true".to_string() || id == "false".to_string() {
            Token::Boolean(
                if id == "true".to_string() {
                    true
                } else {
                    false
                },
                PosRange::new(from, Some(to)),
            )
        } else {
            Token::Identifier(id, PosRange::new(from, Some(to)))
        }
    }
    fn parse_string(&mut self) -> Token {
        let mut id = String::from("");
        let from = self.get_pos();

        // Eat '"'
        self.next();

        let mut escape = false;
        while self.pos < self.code.len() && self.curr() != '"' {
            if self.curr() == '\\' {
                escape = true;
                self.next();
            }

            if escape {
                match self.curr() {
                    'n' => id.push('\n'),
                    't' => id.push('\t'),
                    'r' => id.push('\r'),
                    _ => id.push(self.curr()),
                }
                escape = false
            } else {
                id.push(self.curr());
            }

            self.next();
        }

        // Eat '"'
        self.next();

        let to = self.get_pos();

        Token::String(id, PosRange::new(from, Some(to)))
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
                '"' => tokens.push(self.parse_string()),
                '0'..='9' | '.' => tokens.push(self.parse_number()),
                _ if c.is_ascii_alphabetic() => tokens.push(self.parse_identifier()),
                '+' => {
                    tokens.push(self.add_double_char_token(
                        &mut Token::Plus(PosRange::empty()),
                        &mut Token::PlusEq(PosRange::empty()),
                        ['+', '='],
                    ));
                }
                '-' => {
                    tokens.push(self.add_double_char_token(
                        &mut Token::Minus(PosRange::empty()),
                        &mut Token::MinusEq(PosRange::empty()),
                        ['-', '='],
                    ));
                }
                '*' => {
                    tokens.push(self.add_double_char_token(
                        &mut Token::Multiply(PosRange::empty()),
                        &mut Token::MultiplyEq(PosRange::empty()),
                        ['*', '='],
                    ));
                }
                '/' => {
                    tokens.push(self.add_double_char_token(
                        &mut Token::Divide(PosRange::empty()),
                        &mut Token::DivideEq(PosRange::empty()),
                        ['/', '='],
                    ));
                }
                '%' => {
                    tokens.push(self.add_double_char_token(
                        &mut Token::Modulus(PosRange::empty()),
                        &mut Token::ModulusEq(PosRange::empty()),
                        ['%', '='],
                    ));
                }
                '^' => {
                    if self.lookahed() == '=' {
                        tokens.push(self.add_double_char_token(
                            &mut Token::Power(PosRange::empty()),
                            &mut Token::PowerEq(PosRange::empty()),
                            ['^', '='],
                        ));
                    } else {
                        tokens.push(self.add_triple_char_token(
                            &mut Token::Power(PosRange::empty()),
                            &mut Token::PowerDivide(PosRange::empty()),
                            &mut &mut Token::PowerDivideEq(PosRange::empty()),
                            ['^', '/', '='],
                        ));
                    }
                }
                '~' => {
                    if self.lookahed() == '=' {
                        tokens.push(self.add_double_char_token(
                            &mut Token::Power(PosRange::empty()),
                            &mut Token::PowerEq(PosRange::empty()),
                            ['~', '='],
                        ));
                    } else {
                        tokens.push(self.add_triple_char_token(
                            &mut Token::Tilde(PosRange::empty()),
                            &mut Token::TildeDivide(PosRange::empty()),
                            &mut &mut Token::TildeDivideEq(PosRange::empty()),
                            ['~', '/', '='],
                        ));
                    }
                }
                '=' => {
                    tokens.push(self.add_double_char_token(
                        &mut Token::Equals(PosRange::empty()),
                        &mut Token::DoubleEquals(PosRange::empty()),
                        ['=', '='],
                    ));
                }
                '!' => {
                    tokens.push(self.add_double_char_token(
                        &mut Token::Not(PosRange::empty()),
                        &mut Token::NotEquals(PosRange::empty()),
                        ['!', '='],
                    ));
                }
                '<' => {
                    tokens.push(self.add_double_char_token(
                        &mut Token::LessThan(PosRange::empty()),
                        &mut Token::LessThanEq(PosRange::empty()),
                        ['<', '='],
                    ));
                }
                '>' => {
                    tokens.push(self.add_double_char_token(
                        &mut Token::GreaterThan(PosRange::empty()),
                        &mut Token::GreaterThanEq(PosRange::empty()),
                        ['>', '='],
                    ));
                }
                '&' => {
                    tokens.push(Token::And(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                '|' => {
                    tokens.push(Token::Or(PosRange::new(self.get_pos(), None)));
                    self.next()
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
                ':' => {
                    tokens.push(Token::Colon(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                ';' => {
                    tokens.push(Token::Semicolon(PosRange::new(self.get_pos(), None)));
                    self.next()
                }
                // '\\' => self.next(),
                _ => panic!("Invalid Token [{}:{}]:'{}'", self.line, self.col, c),
            }
        }
        // Add EOF
        tokens.push(Token::EOF(PosRange::new(self.get_pos(), None)));
        // Return tokens
        tokens
    }
}
#[cfg(test)]
mod tests {
    use crate::tokens::{PosRange, Position, Token};

    use super::Lexer;

    #[test]
    fn empty_program() {
        let mut lexer = Lexer::new("".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::EOF(PosRange::new(Position::from(1, 1), None))]
        )
    }

    #[test]
    fn operators() {
        let mut lexer = Lexer::new("+-*/^%".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Plus(PosRange::new(Position::from(1, 1), None)),
                Token::Minus(PosRange::new(Position::from(1, 2), None)),
                Token::Multiply(PosRange::new(Position::from(1, 3), None)),
                Token::Divide(PosRange::new(Position::from(1, 4), None)),
                Token::Power(PosRange::new(Position::from(1, 5), None)),
                Token::Modulus(PosRange::new(Position::from(1, 6), None)),
                Token::EOF(PosRange::new(Position::from(1, 7), None)),
            ]
        )
    }

    #[test]
    fn params() {
        let mut lexer = Lexer::new("()".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::LParan(PosRange::new(Position::from(1, 1), None)),
                Token::RParan(PosRange::new(Position::from(1, 2), None)),
                Token::EOF(PosRange::new(Position::from(1, 3), None)),
            ]
        )
    }

    #[test]
    fn braces() {
        let mut lexer = Lexer::new("{}".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::LBrace(PosRange::new(Position::from(1, 1), None)),
                Token::RBrace(PosRange::new(Position::from(1, 2), None)),
                Token::EOF(PosRange::new(Position::from(1, 3), None)),
            ]
        )
    }

    #[test]
    fn symbols() {
        let mut lexer = Lexer::new("!,:;".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Not(PosRange::new(Position::from(1, 1), None)),
                Token::Comma(PosRange::new(Position::from(1, 2), None)),
                Token::Colon(PosRange::new(Position::from(1, 3), None)),
                Token::Semicolon(PosRange::new(Position::from(1, 4), None)),
                Token::EOF(PosRange::new(Position::from(1, 5), None)),
            ]
        )
    }

    #[test]
    fn assignment() {
        let mut lexer = Lexer::new("= += -= *= /= %= ^= ~/= ^/=".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Equals(PosRange::new(Position::from(1, 1), None)),
                Token::PlusEq(PosRange::new(
                    Position::from(1, 3),
                    Some(Position::from(1, 4))
                )),
                Token::MinusEq(PosRange::new(
                    Position::from(1, 6),
                    Some(Position::from(1, 7))
                )),
                Token::MultiplyEq(PosRange::new(
                    Position::from(1, 9),
                    Some(Position::from(1, 10))
                )),
                Token::DivideEq(PosRange::new(
                    Position::from(1, 12),
                    Some(Position::from(1, 13))
                )),
                Token::ModulusEq(PosRange::new(
                    Position::from(1, 15),
                    Some(Position::from(1, 16))
                )),
                Token::PowerEq(PosRange::new(
                    Position::from(1, 18),
                    Some(Position::from(1, 19))
                )),
                Token::TildeDivideEq(PosRange::new(
                    Position::from(1, 21),
                    Some(Position::from(1, 23))
                )),
                Token::PowerDivideEq(PosRange::new(
                    Position::from(1, 25),
                    Some(Position::from(1, 27))
                )),
                Token::EOF(PosRange::new(Position::from(1, 28), None)),
            ]
        )
    }

    #[test]
    fn comparison() {
        let mut lexer = Lexer::new("!= == < > <= >=".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::NotEquals(PosRange::new(Position::from(1, 1), Position::to(1, 2))),
                Token::DoubleEquals(PosRange::new(Position::from(1, 4), Position::to(1, 5))),
                Token::LessThan(PosRange::new(Position::from(1, 7), None)),
                Token::GreaterThan(PosRange::new(Position::from(1, 9), None)),
                Token::LessThanEq(PosRange::new(Position::from(1, 11), Position::to(1, 12))),
                Token::GreaterThanEq(PosRange::new(Position::from(1, 14), Position::to(1, 15))),
                Token::EOF(PosRange::new(Position::from(1, 16), None)),
            ]
        )
    }

    #[test]
    fn if_elif_else() {
        let mut lexer = Lexer::new("if elif else".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::IfK(PosRange::new(Position::from(1, 1), Position::to(1, 3))),
                Token::ElifK(PosRange::new(Position::from(1, 4), Position::to(1, 8))),
                Token::ElseK(PosRange::new(Position::from(1, 9), Position::to(1, 13))),
                Token::EOF(PosRange::new(Position::from(1, 13), None)),
            ]
        )
    }

    #[test]
    fn while_break_continue() {
        let mut lexer = Lexer::new("while break continue".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::WhileK(PosRange::new(Position::from(1, 1), Position::to(1, 6))),
                Token::BreakK(PosRange::new(Position::from(1, 7), Position::to(1, 12))),
                Token::ContinueK(PosRange::new(Position::from(1, 13), Position::to(1, 21))),
                Token::EOF(PosRange::new(Position::from(1, 21), None)),
            ]
        )
    }

    #[test]
    fn fn_return() {
        let mut lexer = Lexer::new("fn return".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::FnK(PosRange::new(Position::from(1, 1), Position::to(1, 3))),
                Token::ReturnK(PosRange::new(Position::from(1, 4), Position::to(1, 10))),
                Token::EOF(PosRange::new(Position::from(1, 10), None)),
            ]
        )
    }

    #[test]
    fn integer() {
        let mut lexer = Lexer::new("1".to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Int(1, PosRange::new(Position::from(1, 1), Position::to(1, 2))),
                Token::EOF(PosRange::new(Position::from(1, 2), None))
            ]
        )
    }
}
