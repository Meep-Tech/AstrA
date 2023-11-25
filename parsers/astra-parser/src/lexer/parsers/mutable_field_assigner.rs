use crate::{
    lexer::{
        parser::{self, Parser as _},
        results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
    },
    Cursor, End,
};

pub const KEY: &str = "mutable-field-assigner";

impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> Option<End> {
        if cursor.try_read(':') {
            if cursor.char().is_whitespace() {
                return End::Token();
            } else {
                Error::new("missing-expected-tailing-whitespace-in-mutable-field-assigner")
                    .text("Missing expected whitespace after 'mutable-field-assigner' token.")
                    .result()
            }
        } else {
            Error::new("unexpected-in-mutable-field-assigner")
                .text("Unexpected character in 'mutable-field-assigner' token.")
                .result()
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
            Error::new("failed-to-parse-mutable-field-assigner").build(0, input.len()),
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
