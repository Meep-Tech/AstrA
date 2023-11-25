/// A boilerplate template for a parser.
use crate::{
    lexer::{
        parser::{self, Parser as _},
        results::{builder::Builder, error::Error, parsed::Parsed},
    },
    Cursor, End, Token,
};

pub const KEY: &str = "indent-increase";

impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> Option<End> {
        while cursor.char().is_whitespace() {
            match cursor.read() {
                '\n' => {
                    if cursor.next().is_whitespace() {
                        return End::Token();
                    }
                }
                _ => {}
            }
            cursor.next();
        }

        todo!()
    }
}

// boilerplate
pub struct Parser {}
pub static PARSER: Parser = Parser {};
pub fn parse(input: &str) -> Parsed {
    match PARSER.parse(input) {
        Some(parsed) => parsed,
        None => Parsed::Error(
            Error::new(&("failed-to-parse".to_string() + &KEY).to_string()).build(0, input.len()),
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
