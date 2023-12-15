use crate::{
    parser::results::{builder::Builder, end::End, r#match::Match},
    utils::log,
};
use std::collections::{HashMap, HashSet};

pub struct TokenBuilder {
    pub name: Option<String>,
    pub tags: Option<HashSet<String>>,
    pub children: Option<Vec<Match>>,
    pub keys: Option<HashMap<String, usize>>,
}

impl TokenBuilder {
    pub fn new() -> TokenBuilder {
        log::vvv!(&["TOKEN", ":NEW"], "");
        TokenBuilder {
            name: None,
            tags: None,
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
        log::info!(&["TOKEN", "-", "NAME"], name);
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
        log::info!(
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

    pub fn child(mut self, child: Match) -> TokenBuilder {
        log::info!(
            &["TOKEN", "-", "CHILD"],
            &format!(
                "{} : {}",
                self.name.as_ref().unwrap_or(&"^".to_string()),
                child.name
            )
        );

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

    pub fn add_child(&mut self, child: Match) -> &mut TokenBuilder {
        log::info!(
            &["TOKEN", "-", "CHILD"],
            &format!(
                "{} : {}",
                self.name.as_ref().unwrap_or(&"^".to_string()),
                child.name
            )
        );

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

    pub fn prop(mut self, key: &str, value: Match) -> TokenBuilder {
        log::info!(
            &["TOKEN", "-", "PROP"],
            &format!(
                "{} : {}",
                self.name.as_ref().unwrap_or(&"^".to_string()),
                key
            )
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

    pub fn set_prop(&mut self, key: &str, value: Match) -> &mut TokenBuilder {
        log::info!(
            &["TOKEN", "-", "PROP"],
            &format!(
                "{} : {}",
                self.name.as_ref().unwrap_or(&"^".to_string()),
                key
            )
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

impl Builder<Match> for TokenBuilder {
    fn build(self, start: usize, end: usize) -> Match {
        log::vvv!(
            &["TOKEN", ":BUILD"],
            &format!(
                "{} : ({}, {})",
                self.name.as_ref().unwrap_or(&"^".to_string()),
                start,
                end
            )
        );

        return Match {
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
