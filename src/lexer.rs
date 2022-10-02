use super::tokens::Token;
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

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.pos < self.code.len() {
            let c = self.curr();
            match c {
                ' ' | '\n' | '\t' | '\r' => self.next(),
                '0'..='9' | '.' => tokens.push(self.parse_number()),
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
                '(' => {
                    tokens.push(Token::LParam);
                    self.next()
                }
                ')' => {
                    tokens.push(Token::RParam);
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
