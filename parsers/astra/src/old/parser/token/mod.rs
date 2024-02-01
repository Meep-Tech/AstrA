use std::collections::HashMap;

use self::cats::{
    Aliases, Attribute, Attributes, Category, Comment, Comments, Entries, Entry, Identifier,
    Identifiers, Modifier, Modifiers, Procedural, Procedurals, Structure, Structures, Tags,
};

use super::{Error, Term};

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
    pub terms: Vec<Term>,
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
            terms: vec![],
            children: vec![],
            errors: vec![],
            keys: HashMap::new(),
        }
    }

    #[allow(non_snake_case)]
    pub(in super::super) fn Of_Type(ttype: Type, start: usize) -> Token {
        Token {
            ttype,
            start,
            end: start,
            terms: vec![],
            children: vec![],
            errors: vec![],
            keys: HashMap::new(),
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

    pub(in super::super) fn end(mut self, at: usize) -> Self {
        self.end = at;
        self
    }
}
