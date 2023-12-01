use crate::{lexer::results::builder::Builder, lexer::results::error::Error, End, Parsed};
use std::collections::{HashMap, HashSet};

pub struct ErrorBuilder {
    pub name: String,
    pub text: Option<String>,
    pub tags: Option<HashSet<String>>,
    pub children: Option<Vec<Parsed>>,
    pub keys: Option<HashMap<String, usize>>,
}

impl ErrorBuilder {
    pub fn new(name: &str) -> ErrorBuilder {
        ErrorBuilder {
            name: name.to_string(),
            text: None,
            tags: None,
            children: None,
            keys: None,
        }
    }

    pub fn text(mut self, text: &str) -> ErrorBuilder {
        self.text = Some(text.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> ErrorBuilder {
        self.name = name.to_string();
        self
    }

    pub fn tags(mut self, types: Vec<&str>) -> ErrorBuilder {
        self.tags = Some(types.iter().map(|t| t.to_string()).collect());
        self
    }

    pub fn tag(mut self, tag: &str) -> ErrorBuilder {
        match self.tags {
            Some(mut types) => {
                types.insert(tag.to_string());
                self.tags = Some(types);
            }
            None => {
                self.tags = Some(vec![tag.to_string()].into_iter().collect());
            }
        }
        self
    }

    pub fn children(mut self, els: Vec<Parsed>) -> ErrorBuilder {
        self.children = Some(els);
        self
    }

    pub fn child(mut self, el: Parsed) -> ErrorBuilder {
        match self.children {
            Some(mut els) => {
                els.push(el);
                self.children = Some(els);
            }
            None => {
                self.children = Some(vec![el]);
            }
        }
        self
    }

    pub fn props(mut self, props: HashMap<String, usize>) -> ErrorBuilder {
        self.keys = Some(props);
        self
    }

    pub fn prop(mut self, key: &str, value: Parsed) -> ErrorBuilder {
        self = self.child(value);
        let index = match &self.children {
            Some(els) => els.len() - 1,
            None => 0,
        };

        match self.keys {
            Some(mut props) => {
                props.insert(key.to_string(), index);
                self.keys = Some(props);
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

impl Builder<Error> for ErrorBuilder {
    fn build(self, start: usize, end: usize) -> Error {
        return Error {
            name: self.name,
            text: self.text,
            tags: self.tags,
            children: self.children.unwrap_or(Vec::new()),
            keys: self.keys,
            start,
            end,
        };
    }

    fn result(self) -> Option<End> {
        return Some(End::Fail(self));
    }
}
