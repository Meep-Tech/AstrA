use crate::{lexer::{results::builder::Builder, parser::{self}}, lexer::results::error::Error, End, Cursor};

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
        let mut is_pure_numeric: bool;
        let mut curr: char = cursor.char();

        if curr.is_numeric() {
            is_pure_numeric = true;
        } else if curr.is_alphabetic() || Parser::is_allowed_symbol(curr) {
            is_pure_numeric = false;
        } else {
            return Error::new("invalid-name-first-char")
                .text("Invalid first character in 'name' token.")
                .end();
        }

        let mut last_lone_char: Option<char> = None;
        loop {
            curr = cursor.read();
            if cursor.eof() {
                return End::Token();
            }

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
                            .text( "Non-repeatable symbol in 'name' token repeated twice.")
                            .end();
                    }
                }

                // skip the last_lone_char = None call.
                last_lone_char = Some(curr);
                continue;
            } else {
                if !is_pure_numeric {
                    return End::Token();
                } else {
                    return 
                        Error::new("invalid-name-numeric")
                        .text("Token type 'name' cannot be purely numeric and must contain some alphanumeric characters and/or non-math symbols.")
                        .end();
                    
                }
            }

            last_lone_char = None;
        }
    }
}