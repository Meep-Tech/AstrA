use crate::{lexer::{results::{builder::Builder, parsed::Parsed, token::Token}, parser::{self, Parser as _}}, lexer::results::error::Error, End, Cursor};

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

impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> Option<End> {
        let mut is_pure_numeric: bool;
        let mut curr: char = cursor.char();

        if curr.is_numeric() {
            is_pure_numeric = true;
        } else if curr.is_alphabetic() || Parser::is_allowed_symbol(curr) {
            is_pure_numeric = false;
        } else {
            return Error::new("invalid-name-first-char")
                .text("Invalid first character in 'name' token.")
                .result();
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
                            .result();
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
                        .result();
                    
                }
            }

            last_lone_char = None;
        }
    }
}

// boilerplate
pub struct Parser {}
pub static PARSER: Parser = Parser {};
pub fn parse(input: &str) -> Parsed {
    match PARSER.parse(input) {
        Some(parsed) => parsed,
        None => Parsed::Error(Error::new("failed-to-parse-name").build(0, input.len())),
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
