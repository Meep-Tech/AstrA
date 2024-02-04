use crate::{
    parser::results::{builder::Builder, end::End, token::Token},
    utils::log,
};
use std::collections::{HashMap, HashSet};

pub struct TokenBuilder {
    pub name: Option<String>,
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub tags: Option<HashSet<String>>,
    pub children: Option<Vec<Token>>,
    pub keys: Option<HashMap<String, usize>>,
}

impl TokenBuilder {
    pub fn new() -> TokenBuilder {
        log::vvv!(&["TOKEN", ":NEW"], "");
        TokenBuilder {
            name: None,
            tags: None,
            start: None,
            end: None,
            children: None,
            keys: None,
        }
    }

    pub fn name(mut self, name: &str) -> TokenBuilder {
        log::vvv!(&["TOKEN", "-", "NAME"], name);
        self.name = Some(name.to_string());
        self
    }

    pub fn set_name(&mut self, name: &str) -> &mut TokenBuilder {
        log::vvv!(&["TOKEN", "-", "NAME"], name);
        self.name = Some(name.to_string());
        self
    }

    pub(crate) fn assure_name(self, name: &str) -> TokenBuilder {
        match self.name {
            Some(_) => self,
            None => self.name(name),
        }
    }

    pub fn start(mut self, start: usize) -> TokenBuilder {
        log::vvv!(&["TOKEN", "-", "START"], &start.to_string());
        self.start = Some(start);
        self
    }

    pub fn set_start(&mut self, start: usize) -> &mut TokenBuilder {
        log::vvv!(&["TOKEN", "-", "START"], &start.to_string());
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: usize) -> TokenBuilder {
        log::vvv!(&["TOKEN", "-", "END"], &end.to_string());
        self.end = Some(end);
        self
    }

    pub fn set_end(&mut self, end: usize) -> &mut TokenBuilder {
        log::vvv!(&["TOKEN", "-", "END"], &end.to_string());
        self.end = Some(end);
        self
    }

    pub fn tag(mut self, tag: &str) -> TokenBuilder {
        log::vvv!(
            &["TOKEN", "-", "TAG"],
            &format!(
                "{} : {}",
                self.name.as_ref().unwrap_or(&"^".to_string()),
                tag
            )
        );

        match self.tags {
            Some(mut tags) => {
                tags.insert(tag.to_string());
                self.tags = Some(tags);
            }
            None => {
                self.tags = Some(vec![tag.to_string()].into_iter().collect());
            }
        }
        self
    }

    pub fn add_tag(&mut self, tag: &str) -> &mut TokenBuilder {
        log::info!(
            &["TOKEN", "-", "TAG"],
            &format!(
                "{} : {}",
                self.name.as_ref().unwrap_or(&"^".to_string()),
                tag
            )
        );

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

    pub fn prepend_child(&mut self, child: Token) -> &TokenBuilder {
        log::info!(
            &["TOKEN", "-", "PREPEND-CHILD"],
            &format!(" <+ : {}", child.name)
        );

        match self.children {
            Some(ref mut children) => {
                children.insert(0, child);
            }
            None => {
                self.children = Some(vec![child]);
            }
        }

        if let Some(keys) = &mut self.keys {
            for (_, index) in keys.iter_mut() {
                *index += 1;
            }
        }

        self
    }

    pub fn child(mut self, child: Token) -> TokenBuilder {
        log::info!(&["TOKEN", "-", "CHILD"], &format!("++ {}", child.name));

        match self.children {
            Some(mut children) => {
                children.push(child);
                self.children = Some(children);
            }
            None => {
                self.children = Some(vec![child]);
            }
        }
        self
    }

    pub fn add_child(&mut self, child: Token) -> &mut TokenBuilder {
        log::info!(&["TOKEN", "-", "CHILD"], &format!(" ++ {}", child.name));

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
            &["TOKEN", "-", "PROP"],
            &format!("{} : {}", key, value.name)
        );

        self.add_child(value);
        let index = self.children.as_ref().unwrap().len() - 1;

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

    pub fn set_prop(&mut self, key: &str, value: Token) -> &mut TokenBuilder {
        log::info!(
            &["TOKEN", "-", "PROP"],
            &format!("{} : {}", key, value.name)
        );

        self.add_child(value);
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
    fn len(&self) -> usize {
        match &self.children {
            Some(els) => els.len(),
            None => 0,
        }
    }

    fn build(self) -> Token {
        let start = self.start.unwrap_or_else(|| {
            panic!("build called with start not set");
        });
        let end = self.end.unwrap_or_else(|| {
            panic!("build called with end not set");
        });
        self.build_from(start, end)
    }

    fn build_from(self, start: usize, end: usize) -> Token {
        if end < start {
            panic!(
                "TokenBuilder::build called with end < start: {} < {}",
                end, start
            );
        }

        log::vvv!(
            &["TOKEN", ":BUILD"],
            &format!(
                "{} : ({}, {})",
                self.name.as_ref().unwrap_or(&"^".to_string()),
                start,
                end
            )
        );

        return Token {
            name: self.name.unwrap(),
            tags: self.tags,
            children: self.children.unwrap_or(Vec::new()),
            keys: self.keys,
            start,
            end,
        };
    }

    fn build_at(self, start: usize) -> Token {
        let end = self
            .end
            .unwrap_or_else(|| panic!("Builder::build_from called without end being set!"));
        self.build_from(start, end)
    }

    fn build_to(self, end: usize) -> Token {
        let start = self
            .start
            .unwrap_or_else(|| panic!("Builder::build_to called without start being set!"));
        self.build_from(start, end)
    }

    fn build_with_defaults(self, start: usize, end: usize) -> Token {
        let start = self.start.unwrap_or(start);
        let end = self.end.unwrap_or(end);
        if end < start {
            panic!("token builder called with end < start: {} < {}", end, start);
        }

        log::vvv!(
            &["TOKEN", ":BUILD"],
            &format!(
                "{} : ({}, {})",
                self.name.as_ref().unwrap_or(&"^".to_string()),
                start,
                end
            )
        );

        return Token {
            name: self.name.unwrap(),
            tags: self.tags,
            children: self.children.unwrap_or(Vec::new()),
            keys: self.keys,
            start,
            end,
        };
    }

    fn to_end(self) -> End {
        return End::Match(self);
    }
}
