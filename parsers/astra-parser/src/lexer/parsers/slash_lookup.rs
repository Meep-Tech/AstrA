use crate::{
    lexer::{
        parser::{self, Parser as _},
        results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
    },
    Cursor, End,
};

pub static KEY: &'static str = "slash-lookup";

impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> Option<End> {
        if cursor.try_read('/') {
            if let Some(name) = crate::lexer::parsers::name::PARSER.try_parse_at(cursor) {
                return Token::new().child(name).result();
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

// boilerplate
pub struct Parser {}
pub static PARSER: Parser = Parser {};
pub fn parse(input: &str) -> Parsed {
    match PARSER.parse(input) {
        Some(parsed) => parsed,
        None => Parsed::Error(Error::new("failed-to-parse-slash-lookup").build(0, input.len())),
    }
}

pub fn parse_at(cursor: &mut Cursor) -> Parsed {
    PARSER.parse_at(cursor).unwrap()
}

pub fn parse_opt(cursor: &mut Cursor) -> Option<Parsed> {
    PARSER.parse_at(cursor)
}

pub fn try_parse_at(cursor: &mut Cursor) -> Option<Token> {
    PARSER.try_parse_at(cursor)
}
