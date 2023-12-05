use crate::{
    lexer::results::builder::Builder, lexer::results::error::Error, utils::log, End, Parsed,
};
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
        log::info!(&["ERROR", ":NEW"], name);
        ErrorBuilder {
            name: name.to_string(),
            text: None,
            tags: None,
            children: None,
            keys: None,
        }
    }

    pub fn text(mut self, text: &str) -> ErrorBuilder {
        log::info!(
            &["ERROR", "-", "TEXT"],
            &format!("{} : {}", self.name, text)
        );

        self.text = Some(text.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> ErrorBuilder {
        log::info!(
            &["ERROR", "-", "NAME"],
            &format!("{} : {}", self.name, name)
        );

        self.name = name.to_string();
        self
    }

    pub fn assure_name(self, name: &str) -> ErrorBuilder {
        if self.name.contains("{}") {
            let current_name = self.name.clone();
            return self.name(&current_name.replace("{}", name));
        } else {
            return self;
        }
    }

    pub fn tag(mut self, tag: &str) -> ErrorBuilder {
        log::info!(&["ERROR", "-", "TAG"], &format!("{} : {}", self.name, tag));
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

    pub fn child(mut self, child: Parsed) -> ErrorBuilder {
        log::info!(
            &["ERROR", "-", "CHILD"],
            &format!(
                "{} : {:?}",
                self.name,
                match &child {
                    Parsed::Pass(token) => token.name.clone(),
                    Parsed::Fail(error) => match error {
                        Some(err) => err.name.clone(),
                        None => "-none-".to_string(),
                    },
                }
            )
        );

        match self.children {
            Some(mut els) => {
                els.push(child);
                self.children = Some(els);
            }
            None => {
                self.children = Some(vec![child]);
            }
        }
        self
    }

    pub fn prop(mut self, key: &str, value: Parsed) -> ErrorBuilder {
        log::info!(
            &["ERROR", "-", "PROP"],
            &format!(
                "{} : {} = {:?}",
                self.name,
                key,
                match &value {
                    Parsed::Pass(token) => token.name.clone(),
                    Parsed::Fail(error) => match error {
                        Some(err) => err.name.clone(),
                        None => "-none-".to_string(),
                    },
                }
            )
        );

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

impl Builder<Option<Error>> for ErrorBuilder {
    fn build(self, start: usize, end: usize) -> Option<Error> {
        log::info!(
            &["ERROR", ":BUILD"],
            &format!("{} : ({}, {})", self.name, start, end)
        );

        return Some(Error {
            name: self.name,
            text: self.text,
            tags: self.tags,
            children: self.children.unwrap_or(Vec::new()),
            keys: self.keys,
            start,
            end,
        });
    }

    fn end(self) -> End {
        return End::Fail(self);
    }
}
