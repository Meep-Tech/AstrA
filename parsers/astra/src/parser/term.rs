use super::Symbol;

pub type Index = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Indent {
    Increase,
    Decrease,
}

pub struct Term {
    pub ttype: Type,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Word { is_delimited: bool },
    Number,
    Symbol(Symbol),
    Indent(Indent),
    Ambiguous(Vec<Type>),
}

impl Term {
    pub type Type = Type;

    #[allow(non_snake_case)]
    pub fn Of_Type(ttype: Type, start: usize) -> Term {
        Term {
            ttype,
            start,
            end: start,
        }
    }

    pub fn end(mut self, at: usize) -> Self {
        self.end = at;
        self
    }

    pub fn is_ws(&self) -> bool {
        match &self.ttype {
            Type::Indent(_) => true,
            _ => false,
        }
    }

    pub fn is_ambiguous(&self) -> bool {
        match &self.ttype {
            Type::Ambiguous(_) => true,
            _ => false,
        }
    }
}
