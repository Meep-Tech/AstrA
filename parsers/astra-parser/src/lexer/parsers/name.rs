use crate::{
    lexer::results::error::Error,
    lexer::{
        parser::{self},
        results::builder::Builder,
    },
    Cursor, End,
};

pub const KEY: &str = "name";

impl Parser {
    fn is_allowed_symbol(c: char) -> bool {
        match c {
            '$' | '@' => true,
            _ => false,
        }
    }

    fn is_allowed_in_middle_without_repeating(c: char) -> bool {
        match c {
            '-' | '+' | '*' | '/' | '%' | '^' | '~' => true,
            _ => false,
        }
    }

    fn is_allowed_in_middle_with_repeating(c: char) -> bool {
        match c {
            '_' => true,
            _ => false,
        }
    }
}

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        let start = cursor.pos;
        let mut is_pure_numeric: bool;
        let mut curr: char = cursor.curr();

        if curr.is_numeric() {
            is_pure_numeric = true;
        } else if curr.is_alphabetic() || Parser::is_allowed_symbol(curr) {
            is_pure_numeric = false;
        } else {
            return End::Missing(
                "first-letter",
                "alphanumeric or allowed symbol: $, @",
                &cursor.curr_str(),
            );
        }

        cursor.read();

        let mut last_lone_char: Option<char> = None;
        loop {
            if cursor.eof_at(cursor.pos + 1) {
                return End::Token();
            }
            curr = cursor.curr();

            if curr.is_alphanumeric()
                || Parser::is_allowed_symbol(curr)
                || Parser::is_allowed_in_middle_with_repeating(curr)
            {
                if is_pure_numeric && !curr.is_numeric() {
                    is_pure_numeric = false;
                }
            } else if Parser::is_allowed_in_middle_without_repeating(curr) {
                if let Some(last) = last_lone_char {
                    if last == curr {
                        return Error::new("invalid-name-repeat-lone-symbol")
                            .text("Non-repeatable symbol in 'name' token repeated twice.")
                            .end();
                    }
                }

                // skip the last_lone_char = None call.
                last_lone_char = Some(curr);
                cursor.read();
                continue;
            } else {
                if !is_pure_numeric {
                    return End::Token();
                } else {
                    return Error::unexpected("pure-numeric-key", &cursor.slice(start, cursor.pos));
                }
            }

            last_lone_char = None;
            cursor.read();
        }
    }
}
