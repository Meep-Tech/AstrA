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
}

impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, _cursor: &mut Cursor) -> Option<End> {
        todo!();
    }
}

// boilerplate
pub struct Parser {}
pub static PARSER: Parser = Parser {};
pub fn parse(input: &str) -> Indents {
    match match_result(PARSER.parse(input)) {
        Some(indents) => indents,
        None => Indents::Error(
            Error::new(&("failed-to-parse".to_string() + &KEY).to_string()).build(0, input.len()),
        ),
    }
}

pub fn parse_at(cursor: &mut Cursor) -> Indents {
    match match_result(PARSER.parse_at(cursor)) {
        Some(indents) => indents,
        None => Indents::Error(
            Error::new(&("failed-to-parse".to_string() + &KEY).to_string()).build(0, 0),
        ),
    }
}

pub fn parse_opt(cursor: &mut Cursor) -> Option<Indents> {
    match_result(PARSER.parse_at(cursor))
}

pub fn match_result(result: Option<Parsed>) -> Option<Indents> {
    match result {
        Some(parsed) => match parsed {
            Parsed::Token(token) => match token.name.as_str() {
                current::KEY => Indents::Current(token).into(),
                increase::KEY => Indents::Increase(token).into(),
                decrease::KEY => Indents::Decrease(token).into(),
                _ => Indents::Error(Error::new("unknown-indent-type").build(0, 0)).into(),
            },
            Parsed::Error(error) => Some(Indents::Error(error)),
        },
        None => None,
    }
}
