use self::cats::{Category, Delimiter, Delimiters, Operator, Operators, Whitespace, Whitespaces};

use super::{Cursor, Scanner};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    None,
    Ambiguous(Vec<Type>),
    Word,
    Number,
    Operator(Operator),
    Delimiter(Delimiter),
    Whitespace(Whitespace),
}

impl Type {
    pub type Operators = Operators;
    pub type Delimiters = Delimiters;
    pub type Whitespaces = Whitespaces;
}

pub(crate) mod cats;

#[derive(Debug, Clone)]
pub struct Term {
    pub ttype: Type,
    pub start: usize,
    pub end: usize,
}

impl Term {
    pub type Type = Type;
    pub type Category = dyn Category;

    #[allow(non_snake_case)]
    pub(in super::super) fn New(start: usize) -> Term {
        Term {
            ttype: Type::None,
            start,
            end: start,
        }
    }

    #[allow(non_snake_case)]
    pub(in super::super) fn Of_Type(ttype: Type, start: usize) -> Term {
        Term {
            ttype,
            start,
            end: start,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
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

    pub fn text_from(&self, source: &str) -> String {
        source[self.start..self.end].to_string()
    }

    pub(in super::super) fn typing(&mut self, ttype: Type) -> &mut Self {
        self.ttype = ttype;
        self
    }

    pub(in super::super) fn end(mut self, at: usize) -> Self {
        self.end = at;
        self
    }
}
