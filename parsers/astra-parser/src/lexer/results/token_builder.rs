use crate::{lexer::results::builder::Builder, End, Token};
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
        self.name = Some(name.to_string());
        self
    }

    pub(crate) fn assure_name(self, name: &str) -> TokenBuilder {
        match self.name {
            Some(_) => self,
            None => self.name(name),
        }
    }

    pub fn tags(mut self, types: Vec<&str>) -> TokenBuilder {
        self.tags = Some(types.iter().map(|t| t.to_string()).collect());
        self
    }

    pub fn tag(mut self, tag: &str) -> TokenBuilder {
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
        println!("ADD-CHILD: {:?}", child.name);
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
        println!("ADD-PROP: {} = {:?}", key, value.name);
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
