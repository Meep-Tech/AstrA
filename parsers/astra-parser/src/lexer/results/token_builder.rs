use crate::{lexer::results::builder::Builder, utils::log, End, Token};
use std::collections::{HashMap, HashSet};

pub struct TokenBuilder {
    pub name: Option<String>,
    pub tags: Option<HashSet<String>>,
    pub children: Option<Vec<Token>>,
    pub keys: Option<HashMap<String, usize>>,
}

impl TokenBuilder {
    pub fn new() -> TokenBuilder {
        TokenBuilder {
            name: None,
            tags: None,
            children: None,
            keys: None,
        }
    }

    pub fn name(mut self, name: &str) -> TokenBuilder {
        log::info!(&["TOKEN", "SET-NAME"], name);
        self.name = Some(name.to_string());
        self
    }

    pub(crate) fn assure_name(self, name: &str) -> TokenBuilder {
        match self.name {
            Some(_) => self,
            None => self.name(name),
        }
    }

    pub fn tag(mut self, tag: &str) -> TokenBuilder {
        log::info!(&["TOKEN", "ADD-TAG"], tag);
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
        log::info!(&["TOKEN", "ADD-CHILD"], &format!("{:?}", child.name));
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
        log::info!(
            &["TOKEN", "ADD-PROP"],
            &format!("{} = {:?}", key, value.name),
        );

        self = self.child(value);
        let index = self.children.as_ref().unwrap().len() - 1;

        match self.keys {
            Some(ref mut props) => {
                props.insert(key.to_string(), index);
            }
            None => {
                let mut props = HashMap::new();
                props.insert(key.to_string(), index);
                self.keys = Some(props);
            }
        }
        self
    }
}

impl Builder<Token> for TokenBuilder {
    fn build(self, start: usize, end: usize) -> crate::Token {
        return Token {
            name: self.name.unwrap(),
            tags: self.tags,
            children: self.children.unwrap_or(Vec::new()),
            keys: self.keys,
            start,
            end,
        };
    }

    fn end(self) -> End {
        return End::Match(self);
    }
}
