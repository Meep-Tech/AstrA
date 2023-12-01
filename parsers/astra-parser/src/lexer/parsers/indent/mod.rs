pub mod current;
pub mod decrease;
pub mod increase;

use crate::{
    lexer::{
        parser::{self, Parser as _},
        results::{builder::Builder, error::Error, parsed::Parsed},
    },
    Cursor, End, Token,
};

pub const KEY: &str = "indent";

pub enum Indents {
    Increase(Token),
    Decrease(Token),
    Current(Token),
    Error(Error),
    Ignored(Error),
}

impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, _cursor: &mut Cursor) -> End {
        todo!();
    }
}

// boilerplate
pub struct Parser {}

#[allow(non_snake_case)]
pub fn Parse(input: &str) -> Indents {
    Match(Parser::Parse(input))
}

#[allow(non_snake_case)]
pub fn Parse_At(cursor: &mut Cursor) -> Indents {
    Match(Parser::Parse_At(cursor))
}

#[allow(non_snake_case)]
pub fn Try_Parse_At(cursor: &mut Cursor) -> Option<Indents> {
    match Match(Parser::Parse_At(cursor)) {
        Indents::Current(token) => Some(Indents::Current(token)),
        Indents::Increase(token) => Some(Indents::Increase(token)),
        Indents::Decrease(token) => Some(Indents::Decrease(token)),
        _ => None,
    }
}

#[allow(non_snake_case)]
pub fn Try_Parse(input: &str) -> Option<Indents> {
    match Match(Parser::Parse(input)) {
        Indents::Current(token) => Some(Indents::Current(token)),
        Indents::Increase(token) => Some(Indents::Increase(token)),
        Indents::Decrease(token) => Some(Indents::Decrease(token)),
        _ => None,
    }
}

#[allow(non_snake_case)]
pub fn Match(result: Parsed) -> Indents {
    match result {
        Parsed::Token(token) => match token.name.as_str() {
            current::KEY => Indents::Current(token).into(),
            increase::KEY => Indents::Increase(token).into(),
            decrease::KEY => Indents::Decrease(token).into(),
            _ => Indents::Error(Error::new("unknown-indent-type").build(0, 0)).into(),
        },
        Parsed::Error(error) => Indents::Error(error),
    }
}
