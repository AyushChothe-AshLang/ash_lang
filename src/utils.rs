pub mod utils {
    use super::super::tokens::Token;
    use std::collections::HashMap;

    pub fn is_keyword(id: &str) -> Option<Token> {
        let keywords: HashMap<&str, Token> = HashMap::from([
            ("while", Token::WhileK),
            ("if", Token::IfK),
            ("else", Token::ElseK),
            ("elif", Token::ElifK),
            ("fn", Token::FnK),
            ("let", Token::LetK),
            ("break", Token::BreakK),
            ("continue", Token::ContinueK),
            ("return", Token::ReturnK),
        ]);
        if keywords.contains_key(id) {
            Some(keywords.get(id).unwrap().clone())
        } else {
            None
        }
    }
}
