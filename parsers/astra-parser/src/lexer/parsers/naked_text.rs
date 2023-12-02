use super::{
    dot_lookup, escape_sequence,
    indent::{self},
    slash_lookup,
};
use crate::{
    lexer::{
        parser::{self},
        results::{builder::Builder, parsed::Parsed},
    },
    Cursor, End, Token,
};

pub const KEY: &str = "naked-text";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        let mut result = Token::new();
        loop {
            if cursor.is_eof() {
                break;
            }
            if let Some(escape) = escape_sequence::Parser::Try_Parse_At(cursor) {
                result = result.child(escape);
            } else {
                match cursor.curr() {
                    '\n' => match indent::increase::Parser::Try_Parse_At(cursor) {
                        Some(token) => {
                            result = result.child(token);
                        }
                        None => {
                            return End::Token();
                        }
                    },
                    '.' => {
                        if cursor.curr().is_whitespace() && !cursor.next().is_whitespace() {
                            match dot_lookup::Parser::Parse_At(cursor) {
                                Parsed::Token(child) => {
                                    result = result.child(child);
                                }
                                Parsed::Error(error) => return End::Error_In_Child(result, error),
                            }
                        }
                    }
                    '/' => {
                        if cursor.curr().is_whitespace() && !cursor.next().is_whitespace() {
                            match slash_lookup::Parser::Parse_At(cursor) {
                                Parsed::Token(child) => {
                                    result = result.child(child);
                                }
                                Parsed::Error(error) => return End::Error_In_Child(result, error),
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
        }

        return result.end();
    }
}
