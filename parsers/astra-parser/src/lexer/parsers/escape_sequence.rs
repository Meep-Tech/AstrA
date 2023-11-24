use crate::{
    lexer::{
        parser::{self, Parser as _},
        results::{builder::Builder as _, parsed::Parsed, token::Token},
    },
    Cursor, End,
};

pub static KEY: &'static str = "escape-sequence";

impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> Option<End> {
        if cursor.try_read('\\') {
            cursor.read();
            return End::Token();
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
        None => Parsed::Error(
            crate::lexer::results::error::Error::new("failed-to-parse-escape-sequence")
                .build(0, input.len()),
        ),
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
