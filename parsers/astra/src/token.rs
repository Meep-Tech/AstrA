use std::{collections::HashMap, sync::LazyLock};

pub(crate) static _EMPTY_KEYS: LazyLock<HashMap<String, usize>> = LazyLock::new(|| HashMap::new());

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Token {
    ttype: Type,
    start: usize,
    end: usize,
    children: Vec<Token>,
    keys: Option<HashMap<String, usize>>,
}

impl Token {
    pub type Type = Type;

    pub(crate) fn new(ttype: Type, start: usize) -> Token {
        Token {
            ttype,
            start,
            end: start,
            children: Vec::new(),
            keys: None,
        }
    }

    pub fn ttype(&self) -> Type {
        self.ttype
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    fn children(&self) -> Vec<&Token> {
        return self.children.iter().collect();
    }

    fn child(&self, index: usize) -> Option<&Token> {
        return match self.children().get(index) {
            Some(child) => Some(child),
            None => None,
        };
    }

    fn keys(&self) -> &HashMap<String, usize> {
        return match &self.keys {
            Some(keys) => keys,
            None => &_EMPTY_KEYS,
        };
    }

    fn key(&self, index: usize) -> Option<&String> {
        return match self.keys().iter().find(|(_, i)| **i == index) {
            Some((key, _)) => Some(key),
            None => None,
        };
    }

    fn index(&self, key: &str) -> Option<usize> {
        return match self.keys().get(key) {
            Some(index) => Some(*index),
            None => None,
        };
    }

    fn props(&self) -> HashMap<String, &Token> {
        return self
            .keys()
            .iter()
            .map(|(key, index)| (key.clone(), self.child(*index).unwrap()))
            .collect();
    }

    fn prop(&self, key: &str) -> Option<&Token> {
        return match self.index(key) {
            Some(index) => self.child(index),
            None => None,
        };
    }

    pub fn text_from<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }
}

pub(crate) trait TokenBuilder {
    fn set_type(&mut self, ttype: Type) -> &mut Self;
    fn set_start(&mut self, start: usize) -> &mut Self;
    fn set_end(&mut self, end: usize) -> &mut Self;
    fn add_child(&mut self, child: Token) -> &mut Self;
    fn set_prop(&mut self, key: &str, value: Token) -> &mut Self;
}

impl TokenBuilder for Token {
    fn set_type(&mut self, ttype: Type) -> &mut Self {
        self.ttype = ttype;
        self
    }

    fn set_start(&mut self, start: usize) -> &mut Self {
        self.start = start;
        self
    }

    fn set_end(&mut self, end: usize) -> &mut Self {
        self.end = end;
        self
    }

    fn add_child(&mut self, child: Token) -> &mut Self {
        self.children.push(child);
        self
    }

    fn set_prop(&mut self, key: &str, value: Token) -> &mut Self {
        if self.keys.is_none() {
            self.keys = Some(HashMap::new());
        }

        let index = self.children.len();
        self.keys.as_mut().unwrap().insert(key.to_string(), index);
        self.children.push(value);
        self
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Type {
    Unknown,
    Source(Source),
    Attribute(Attribute),
    Symbol(Symbol),
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Source {
    Code(Code),
    Command,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Attribute {
    Group, // group of attributes
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Code {
    Axa, // Default Axa context. Defaults to StruX unless `---`s or other assigners and closures are used.
    Prx, // ProX-only Context.
    Stx, // StruX-only Context.
    Blx, // BloX-only Context.
    Arc, // special archetypical file for defining classes and types with names matching the file name. Must begin with an assigner with no key and no indentation followed by aliases for the class/type names. May optionally have valid attributes (even preceding the assinger)
    Mot, // "Mot"/"Mo[t|d]el/Mote" file. For views with data, like notes. Defaults to view unless `---`s are used. Have some other view focused-features as well.
    Cmd, // command stored in a file.
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Symbol {
    Delimiter(Delimiter),
    Operator(Operator),
    Unknown,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Delimiter {
    Bound,
    Start,
    End,
    Line,
    Modifier,
    Lookup,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Operator {
    Assigner,   // assign value to variable
    Caller,     // call value as proc with potential args
    Relational, // compare values
    Flow,       // effect the flow of code
    Logical,    // deal with logic based on values
}
