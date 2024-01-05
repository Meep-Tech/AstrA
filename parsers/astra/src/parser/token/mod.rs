use std::{collections::HashMap, fmt::Display, sync::Arc};

use self::cats::{
    Aliases, Attribute, Attributes, Category, Comment, Comments, Entries, Entry, Identifier,
    Identifiers, Modifier, Modifiers, Procedural, Procedurals, Structure, Structures, Tags,
};

use super::{Cursor, Scanner};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    None,
    Ambiguous(Vec<Type>),
    Comment(Comment),
    Attribute(Attribute),
    Structure(Structure),
    Procedural(Procedural),
    Entry(Entry),
    Modifier(Modifier),
    Identifier(Identifier),
}

impl Type {
    pub type Comments = Comments;
    pub type Tags = Tags;
    pub type Aliases = Aliases;
    pub type Attributes = Attributes;
    pub type Structures = Structures;
    pub type Procedurals = Procedurals;
    pub type Entries = Entries;
    pub type Modifiers = Modifiers;
    pub type Identifiers = Identifiers;
}

pub(crate) mod cats;

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: Type,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Token>,
    pub errors: Vec<Error>,
    pub keys: HashMap<String, usize>,
}

impl Token {
    pub type Type = Type;
    pub type Category = dyn Category;

    #[allow(non_snake_case)]
    pub(in super::super) fn New(start: usize) -> Token {
        Token {
            ttype: Type::None,
            start,
            end: start,
            children: vec![],
            keys: HashMap::new(),
            errors: vec![],
        }
    }

    #[allow(non_snake_case)]
    pub(in super::super) fn Of_Type(ttype: Type, start: usize) -> Token {
        Token {
            ttype,
            start,
            end: start,
            children: vec![],
            keys: HashMap::new(),
            errors: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn has(&self, key: &str) -> bool {
        self.keys.contains_key(key)
    }

    pub fn is(&self, ttype: Type) -> bool {
        self.ttype == ttype
    }

    pub fn is_in<T>(&self) -> bool
    where
        T: Category + 'static,
    {
        T::New().all().contains(&self.ttype)
    }

    pub fn child(&mut self, index: usize) -> &Token {
        &mut self.children[index]
    }

    pub fn prop(&self, key: &str) -> Option<&Token> {
        if let Some(index) = self.keys.get(key) {
            return Some(&self.children[*index]);
        }

        None
    }

    pub fn props(&self) -> HashMap<String, &Token> {
        let mut props = HashMap::new();
        for (key, index) in &self.keys {
            props.insert(key.to_string(), &self.children[*index]);
        }

        props
    }

    pub fn text_from(&self, source: &str) -> String {
        source[self.start..self.end].to_string()
    }

    pub(in super::super) fn push(&mut self, token: Token) {
        self.children.push(token);
    }

    pub(in super::super) fn set(&mut self, key: &str, token: Token) {
        if let Some(index) = self.keys.get(key) {
            self.children[*index] = token;
            return;
        } else {
            self.keys.insert(key.to_string(), self.children.len());
        }

        self.children.push(token);
    }

    pub(in super::super) fn end(self, cursor: &Cursor) -> Self {
        self.end_at(cursor.index())
    }

    pub(in super::super) fn end_at(mut self, at: usize) -> Self {
        self.end = at;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    etype: String,
    ttype: Type,
    index: usize,
    data: Vec<Vec<String>>,
}

impl<'e> Error {
    pub const INVALID_KEY: &'static str = "invalid_syntax";
    pub const UNEXPECTED_KEY: &'static str = "unexpected_syntax";
    pub const IN_CHILD_KEY: &'static str = "in_child";
    pub const IN_PROP_KEY: &'static str = "in_prop";

    #[allow(non_snake_case)]
    pub(in super::super) fn Unexpected(
        ttype: &Token::Type,
        index: usize,
        found: impl Display,
        expected: &[&str],
    ) -> Error {
        Error {
            etype: Error::UNEXPECTED_KEY.to_string(),
            index,
            ttype: ttype.clone(),
            data: vec![
                expected.iter().map(|e| e.to_string()).collect(),
                vec![found.to_string()],
            ],
        }
    }

    #[allow(non_snake_case)]
    pub(in super::super) fn Invalid(
        ttype: &Token::Type,
        index: usize,
        found: impl Display,
        reason: impl Display,
    ) -> Error {
        Error {
            etype: Error::INVALID_KEY.to_string(),
            index,
            ttype: ttype.clone(),
            data: vec![vec![reason.to_string()], vec![found.to_string()]],
        }
    }

    pub fn key(&self) -> &str {
        &self.etype
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn message(&self) -> String {
        let mut message = String::from(format!(
            "Error in {} @ {}: *{}*; ",
            format!("{:?}", self.ttype),
            self.index,
            self.etype,
        ));

        match self.etype.as_str() {
            Error::INVALID_KEY => {
                message.push_str(&self.data[0][0]);
            }
            Error::UNEXPECTED_KEY => {
                message.push_str(&format!(
                    "Found {}, Expected: {}",
                    self.data[1][0],
                    self.data[0].join(", "),
                ));
            }
            Error::INVALID_KEY => {
                if self.data.len() > 1 {
                    message.push_str(&format!(
                        "Invalid Syntax: {}. {}",
                        self.data[1][0], self.data[0][0],
                    ));
                } else {
                    message.push_str(&format!("Invalid Syntax. {}", self.data[0][0]));
                }
            }
            _ => panic!("unhandled error type"),
        }

        message
    }
}
