use crate::lexer::{parser, results::token_builder::TokenBuilder};

use std::collections::{HashMap, HashSet};

use super::{
    data::{Data, _EMPTY_KEYS, _EMPTY_TAGS},
    end::End,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Token {
    pub name: String,
    pub tags: Option<HashSet<String>>,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Token>,
    pub keys: Option<HashMap<String, usize>>,
}

impl Token {
    pub fn new() -> TokenBuilder {
        return TokenBuilder::new();
    }

    pub fn with_name(name: &str) -> TokenBuilder {
        return TokenBuilder::new().name(name);
    }

    pub fn of_type<T: parser::Parser + 'static>() -> TokenBuilder {
        let name = T::Instance().get_name();
        return TokenBuilder::new().name(name).tag(name);
    }

    pub fn result() -> End {
        return End::Match(Token::new());
    }

    pub fn to_builder(self) -> TokenBuilder {
        return TokenBuilder {
            name: Some(self.name),
            tags: self.tags,
            children: if !self.children.is_empty() {
                Some(self.children)
            } else {
                None
            },
            keys: self.keys,
        };
    }
}

impl Data<Token> for Token {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn tags(&self) -> &HashSet<String> {
        let hash_set = self.tags.as_ref();
        hash_set.unwrap_or(&_EMPTY_TAGS)
    }

    fn start(&self) -> usize {
        return self.start;
    }

    fn end(&self) -> usize {
        return self.end;
    }

    fn children(&self) -> Vec<&Token> {
        return self.children.iter().collect();
    }

    fn keys(&self) -> &HashMap<String, usize> {
        let hash_map = self.keys.as_ref();
        hash_map.unwrap_or(&_EMPTY_KEYS)
    }
}