use crate::{lexer::results::builder::Builder, utils::log, End, Token};
use std::collections::{HashMap, HashSet};

pub struct TokenBuilder {
    pub name: Option<String>,
    pub tags: Option<HashSet<String>>,
    pub children: Option<Vec<Token>>,
    pub props: Option<HashMap<String, Token>>,
}

impl TokenBuilder {
    pub fn new() -> TokenBuilder {
        TokenBuilder {
            name: None,
            tags: None,
            children: None,
            props: None,
        }
    }

    pub fn name(mut self, name: &str) -> TokenBuilder {
        log::info(&["TOKEN", "SET-NAME"], name);
        self.name = Some(name.to_string());
        self
    }

    pub(crate) fn assure_name(self, name: &str) -> TokenBuilder {
        log::info(&["TOKEN", "SET-NAME"], name);
        match self.name {
            Some(_) => self,
            None => self.name(name),
        }
    }

    pub fn tag(mut self, tag: &str) -> TokenBuilder {
        log::info(&["TOKEN", "ADD-TAG"], tag);
        match self.tags {
            Some(ref mut tags) => {
                tags.insert(tag.to_string());
            }
            None => {
                self.tags = Some(vec![tag.to_string()].into_iter().collect());
            }
        }
        self
    }

    pub fn child(mut self, child: Token) -> TokenBuilder {
        log::info(&["TOKEN", "ADD-CHILD"], &format!("{:?}", child.name));
        match self.children {
            Some(ref mut children) => {
                children.push(child);
            }
            None => {
                self.children = Some(vec![child]);
            }
        }
        self
    }

    pub fn prop(mut self, key: &str, value: Token) -> TokenBuilder {
        log::info(
            &["TOKEN", "ADD-PROP"],
            &format!("{} = {:?}", key, value.name),
        );

        match self.props {
            Some(ref mut props) => {
                props.insert(key.to_string(), value);
            }
            None => {
                let mut props = HashMap::new();
                props.insert(key.to_string(), value);
                self.props = Some(props);
            }
        }
        self
    }
}

impl Builder<Token> for TokenBuilder {
    fn build(self, start: usize, end: usize) -> crate::Token {
        return Token {
            name: self.name.unwrap(), //_or("".to_string()),
            tags: self.tags,
            children: self.children.unwrap_or(Vec::new()),
            props: self.props,
            start,
            end,
        };
    }

    fn result(self) -> Option<End> {
        return Some(crate::End::Match(self));
    }
}
