use crate::{
    parser::{self, results::token_builder::TokenBuilder},
    utils::{ansi::Color, sexp::SExpressable},
};
use serde::{Deserialize, Serialize};

use super::{end::End, span::Span};
use crate::parser::results::node::{Node, _EMPTY_KEYS, _EMPTY_TAGS};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub tags: Option<HashSet<String>>,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Token>,
    pub keys: Option<HashMap<String, usize>>,
}

impl Token {
    #[allow(non_snake_case)]
    pub fn Start(at: usize) -> TokenBuilder {
        return TokenBuilder::new().start(at);
    }

    #[allow(non_snake_case)]
    pub fn New() -> TokenBuilder {
        return TokenBuilder::new();
    }

    #[allow(non_snake_case)]
    pub fn With_Name(name: &str) -> TokenBuilder {
        let mut token = TokenBuilder::new();
        token.set_name(name);

        return token;
    }

    #[allow(non_snake_case)]
    pub fn Of_Type<T: parser::Parser + 'static>() -> TokenBuilder {
        let name = T::Get().name();
        let mut token = TokenBuilder::new();
        token.set_name(name);

        return token;
    }

    #[allow(non_snake_case)]
    pub fn End() -> End {
        return End::Match(Token::New());
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
            start: Some(self.start),
        };
    }
}

impl Node<Token> for Token {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn tags(&self) -> &HashSet<String> {
        let hash_set = self.tags.as_ref();
        hash_set.unwrap_or(&_EMPTY_TAGS)
    }

    fn children(&self) -> Vec<&Token> {
        return self.children.iter().collect();
    }

    fn keys(&self) -> &HashMap<String, usize> {
        let hash_map = self.keys.as_ref();
        hash_map.unwrap_or(&_EMPTY_KEYS)
    }
}

impl SExpressable<Token> for Token {
    fn get_children(&self) -> Vec<&Token> {
        self.children()
    }
    fn get_keys(&self) -> &HashMap<String, usize> {
        self.keys()
    }
    fn get_name(&self) -> &str {
        self.name()
    }
    fn get_tags(&self) -> &HashSet<String> {
        self.tags()
    }

    fn name_color() -> crate::utils::ansi::Color {
        return crate::utils::ansi::Color::Green;
    }

    fn node_to_sexp_str(node: &Token, depth: usize, colors: &mut Option<Color::Loop>) -> String {
        node.to_sexp_str(depth, colors)
    }
}

impl Span for Token {
    fn start(&self) -> usize {
        return self.start;
    }

    fn end(&self) -> usize {
        return self.end;
    }
}
