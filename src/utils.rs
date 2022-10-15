pub mod utils {
    use crate::tokens::PosRange;

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
    pub fn variant_eq<T>(a: &T, b: &T) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }
}
