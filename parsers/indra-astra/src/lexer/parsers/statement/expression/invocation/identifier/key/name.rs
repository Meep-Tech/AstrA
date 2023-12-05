use crate::{
    lexer::parser::{self},
    tests::lexer::parsers::test::Testable,
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
            '-' | '+' | '%' | '^' | '~' => true,
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

    fn as_tests(&self) -> Option<&dyn Testable> {
        Some(self)
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
            return End::Mismatch(
                "first-letter",
                "alphanumeric or allowed symbol: $, @",
                &cursor.curr_str(),
            );
        }

        cursor.read();

        let mut last_lone_char: Option<char> = None;
        loop {
            if cursor.is_eof() {
                cursor.read();
                return _check_end(is_pure_numeric, cursor, start);
            }

            curr = cursor.curr();

            if curr.is_alphanumeric()
                || Parser::is_allowed_symbol(curr)
                || Parser::is_allowed_in_middle_with_repeating(curr)
            {
                if is_pure_numeric && !(curr.is_numeric() || curr == '_') {
                    is_pure_numeric = false;
                }
            } else if Parser::is_allowed_in_middle_without_repeating(curr) {
                if let Some(last) = last_lone_char {
                    if last == curr {
                        return End::Unexpected(
                            "repeat-lone-symbol",
                            &cursor.slice(cursor.pos - 1, cursor.pos),
                        );
                    }
                }

                // skip the last_lone_char = None call.
                last_lone_char = Some(curr);
                cursor.read();
                continue;
            } else {
                return _check_end(is_pure_numeric, cursor, start);
            }

            last_lone_char = None;
            cursor.read();
        }
    }
}

fn _check_end(is_pure_numeric: bool, cursor: &mut Cursor, start: usize) -> End {
    if !is_pure_numeric {
        if Parser::is_allowed_in_middle_with_repeating(cursor.prev())
            || Parser::is_allowed_in_middle_without_repeating(cursor.prev())
        {
            return End::Unexpected("last-letter", &cursor.slice(cursor.pos - 1, cursor.pos));
        }
        return End::Token();
    } else {
        return End::Unexpected("pure-numeric-key", &cursor.slice(start, cursor.pos));
    }
}
