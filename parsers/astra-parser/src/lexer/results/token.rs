use crate::lexer::results::token_builder::TokenBuilder;

use std::collections::HashMap;

use super::end::End;

#[derive(Debug)]
pub struct Token {
    pub name: String,
    pub tags: Vec<String>,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Token>,
    pub props: Option<HashMap<String, Token>>,
}

impl Token {
    pub fn new() -> TokenBuilder {
        return TokenBuilder::new();
    }

    pub fn result() -> Option<End> {
        return Some(End::Match(Token::new()));
    }
}
