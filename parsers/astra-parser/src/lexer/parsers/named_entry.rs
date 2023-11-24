use crate::{
    lexer::results::token::Token,
    lexer::{parser, parsers::name},
    lexer::{parser::Parser as _, results::error::Error},
    lexer::{parsers::naked_text, results::builder::Builder},
    Cursor, End, Parsed,
};

use super::mutable_field_assigner;

pub static KEY: &'static str = "named-entry";

impl parser::Parser for Parser {
    fn rule(&self, cursor: &mut Cursor) -> Option<End> {
        let mut result = Token::new();

        let key = name::parse_at(cursor);
        match key {
            Parsed::Token(key) => {
                result = result.prop("key", key);
                cursor.skip_ws();

                let assigner = mutable_field_assigner::parse_at(cursor);
                match assigner {
                    Parsed::Token(assigner) => {
                        result = result.prop("assigner", assigner);
                        cursor.skip_ws();

                        let value = naked_text::parse_at(cursor);

                        match value {
                            Parsed::Token(value) => {
                                return result.prop("value", value).result();
                            }
                            Parsed::Error(error) => return Error::in_child(result, error),
                        }
                    }
                    Parsed::Error(error) => return Error::in_child(result, error),
                }
            }
            Parsed::Error(error) => return Error::in_child(result, error),
        }
    }

    fn get_name(&self) -> &'static str {
        return KEY;
    }
}

// boilerplate
pub struct Parser {}
pub static PARSER: Parser = Parser {};
pub fn parse(input: &str) -> Parsed {
    match PARSER.parse(input) {
        Some(parsed) => parsed,
        None => Parsed::Error(Error::new("failed-to-parse-named-entry").build(0, input.len())),
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
