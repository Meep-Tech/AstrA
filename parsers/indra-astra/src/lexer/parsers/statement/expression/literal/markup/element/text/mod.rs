use crate::{
    lexer::{
        parser::{self},
        parsers::{
            statement::expression::{
                invocation::identifier::lookup::{dot_lookup, slash_lookup},
                literal::escape::escape_sequence,
            },
            whitespace::indent,
        },
        results::{builder::Builder, parsed::Parsed},
    },
    Cursor, End, Token,
};

pub const KEY: &str = "text";

pub struct Parser {}
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
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
                                Parsed::Pass(child) => {
                                    result = result.child(child);
                                }
                                Parsed::Fail(error) => return End::Unexpected_Child(result, error),
                            }
                        }
                    }
                    '/' => {
                        if cursor.curr().is_whitespace() && !cursor.next().is_whitespace() {
                            match slash_lookup::Parser::Parse_At(cursor) {
                                Parsed::Pass(child) => {
                                    result = result.child(child);
                                }
                                Parsed::Fail(error) => return End::Unexpected_Child(result, error),
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
