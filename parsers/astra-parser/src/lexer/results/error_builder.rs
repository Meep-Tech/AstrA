use crate::{lexer::results::builder::Builder, lexer::results::error::Error, End, Parsed};
use std::collections::HashMap;

pub struct ErrorBuilder {
    pub name: String,
    pub text: Option<String>,
    pub tags: Option<Vec<String>>,
    pub children: Option<Vec<Parsed>>,
    pub props: Option<HashMap<String, Parsed>>,
}

impl ErrorBuilder {
    pub fn new(name: &str) -> ErrorBuilder {
        ErrorBuilder {
            name: name.to_string(),
            text: None,
            tags: None,
            children: None,
            props: None,
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
                types.push(tag.to_string());
                self.tags = Some(types);
            }
            None => {
                self.tags = Some(vec![tag.to_string()]);
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

    pub fn props(mut self, props: HashMap<String, Parsed>) -> ErrorBuilder {
        self.props = Some(props);
        self
    }

    pub fn prop(mut self, key: &str, value: Parsed) -> ErrorBuilder {
        match self.props {
            Some(mut props) => {
                props.insert(key.to_string(), value);
                self.props = Some(props);
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

impl Builder<Error> for ErrorBuilder {
    fn build(self, start: usize, end: usize) -> Error {
        return Error {
            name: self.name,
            text: self.text,
            tags: self.tags.unwrap_or(Vec::new()),
            children: self.children.unwrap_or(Vec::new()),
            props: self.props,
            start,
            end,
        };
    }

    fn result(self) -> Option<End> {
        return Some(End::Fail(self));
    }
}
