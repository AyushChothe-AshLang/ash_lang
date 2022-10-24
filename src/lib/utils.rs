
pub mod utils {

    use crate::{nodes::Assignment, tokens::PosRange};

    use super::super::tokens::Token;
    use std::collections::HashMap;

    pub fn is_keyword(id: &str, pos: PosRange) -> Option<Token> {
        let keywords: HashMap<&str, Token> = HashMap::from([
            ("while", Token::WhileK(pos.clone())),
            ("if", Token::IfK(pos.clone())),
            ("else", Token::ElseK(pos.clone())),
            ("elif", Token::ElifK(pos.clone())),
            ("fn", Token::FnK(pos.clone())),
            ("cfn", Token::CFnK(pos.clone())),
            ("let", Token::LetK(pos.clone())),
            ("break", Token::BreakK(pos.clone())),
            ("continue", Token::ContinueK(pos.clone())),
            ("return", Token::ReturnK(pos.clone())),
        ]);
        if keywords.contains_key(id) {
            Some(keywords.get(id).unwrap().clone())
        } else {
            None
        }
    }

    pub fn get_assignment() -> Vec<Token> {
        vec![
            Token::Equals(PosRange::empty()),
            Token::PlusEq(PosRange::empty()),
            Token::MinusEq(PosRange::empty()),
            Token::MultiplyEq(PosRange::empty()),
            Token::DivideEq(PosRange::empty()),
            Token::PowerEq(PosRange::empty()),
            Token::ModulusEq(PosRange::empty()),
            Token::TildeDivideEq(PosRange::empty()),
            Token::PowerDivideEq(PosRange::empty()),
        ]
    }

    pub fn get_assignment_from_token(tkn: &Token) -> Assignment {
        match tkn {
            Token::Equals(_) => Assignment::Equals,
            Token::PlusEq(_) => Assignment::PlusEq,
            Token::MinusEq(_) => Assignment::MinusEq,
            Token::MultiplyEq(_) => Assignment::MultiplyEq,
            Token::DivideEq(_) => Assignment::DivideEq,
            Token::PowerEq(_) => Assignment::PowerEq,
            Token::ModulusEq(_) => Assignment::ModulusEq,
            Token::TildeDivideEq(_) => Assignment::TildeDivideEq,
            Token::PowerDivideEq(_) => Assignment::PowerDivideEq,
            _ => panic!("Invalid Assignment"),
        }
    }
    pub fn variant_eq<T>(a: &T, b: &T) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }
}
