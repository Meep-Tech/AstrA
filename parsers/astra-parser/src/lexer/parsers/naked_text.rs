use super::{dot_lookup, escape_sequence, slash_lookup};
use crate::{
    lexer::{
        parser::{self, Parser as _},
        results::{builder::Builder, error::Error, parsed::Parsed},
    },
    Cursor, End, Token,
};

pub static KEY: &'static str = "naked-text";

impl parser::Parser for Parser {
    fn rule(&self, cursor: &mut Cursor) -> Option<End> {
        let mut result = Token::new();
        loop {
            if let Some(escape) = escape_sequence::PARSER.try_parse_at(cursor) {
                result = result.child(escape);
            } else {
                match cursor.char() {
                    '.' => {
                        if cursor.prev().is_whitespace() && !cursor.next().is_whitespace() {
                            match dot_lookup::PARSER.parse_at(cursor)? {
                                Parsed::Token(child) => {
                                    result = result.child(child);
                                }
                                Parsed::Error(error) => return Error::in_child(result, error),
                            }
                        }
                    }
                    '/' => {
                        if cursor.prev().is_whitespace() && !cursor.next().is_whitespace() {
                            match slash_lookup::PARSER.parse_at(cursor)? {
                                Parsed::Token(child) => {
                                    result = result.child(child);
                                }
                                Parsed::Error(error) => return Error::in_child(result, error),
                            }
                        }
                    }
                    '{' => {
                        todo!();
                    }
                    '#' => {
                        todo!();
                    }
                    '|' => {
                        todo!();
                    }
                    _ => {
                        cursor.read();
                    }
                }
            }

            if cursor.eof() {
                break;
            }
        }

        return result.result();
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
        None => Parsed::Error(Error::new("failed-to-parse-naked-text").build(0, input.len())),
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
