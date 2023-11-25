use crate::lexer::results::token_builder::TokenBuilder;

use std::collections::{HashMap, HashSet};

use super::end::End;

#[derive(PartialEq, Eq, Debug)]
pub struct Token {
    pub name: String,
    pub tags: Option<HashSet<String>>,
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

    pub fn range(&self) -> std::ops::Range<usize> {
        return self.start..self.end;
    }
}
