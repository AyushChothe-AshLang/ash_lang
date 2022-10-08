pub mod utils {
    use crate::tokens::PosRange;

    use super::super::tokens::Token;
    use std::collections::HashMap;

    pub fn is_keyword(id: &str, pos: PosRange) -> Option<Token> {
        let keywords: HashMap<&str, Token> = HashMap::from([
            ("while", Token::WhileK(pos)),
            ("if", Token::IfK(pos)),
            ("else", Token::ElseK(pos)),
            ("elif", Token::ElifK(pos)),
            ("fn", Token::FnK(pos)),
            ("let", Token::LetK(pos)),
            ("break", Token::BreakK(pos)),
            ("continue", Token::ContinueK(pos)),
            ("return", Token::ReturnK(pos)),
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
