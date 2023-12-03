pub mod current;
pub mod decrease;
pub mod increase;

use crate::{
    lexer::{
        parser::{self, Parser as _},
        results::{builder::Builder, error::Error, parsed::Parsed},
    },
    Cursor, End, Token,
};

pub const KEY: &str = "indent";

pub enum Indents {
    Increase(Token),
    Decrease(Token),
    Current(Token),
    Error(Error),
    None,
}

impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        cursor.skip_ws();
        if !cursor.indents.is_reading {
            return End::None;
        }
        if cursor.indents.curr > cursor.indents.prev() {
            Token::new().name(increase::KEY).end()
        } else if cursor.indents.curr < cursor.indents.prev() {
            Token::new().name(decrease::KEY).end()
        } else {
            Token::new().name(current::KEY).end()
        }
    }
}

// boilerplate
pub struct Parser {}

#[allow(non_snake_case)]
pub fn Parse(input: &str) -> Indents {
    Match(Parser::Parse(input))
}

#[allow(non_snake_case)]
pub fn Parse_At(cursor: &mut Cursor) -> Indents {
    Match(Parser::Parse_At(cursor))
}

#[allow(non_snake_case)]
pub fn Parse_Opt(input: &str) -> Indents {
    Match(Parser::Parse_Opt(input))
}

#[allow(non_snake_case)]
pub fn Parse_Opt_At(cursor: &mut Cursor) -> Indents {
    Match(Parser::Parse_Opt_At(cursor))
}

#[allow(non_snake_case)]
pub fn Parse_Opt_Or_Skip(input: &str) -> Indents {
    Match(Parser::Instance().parse_opt_or_skip(input))
}

#[allow(non_snake_case)]
pub fn Parse_Opt_Or_Skip_At(cursor: &mut Cursor) -> Indents {
    Match(Parser::Instance().parse_opt_or_skip_at(cursor))
}

#[allow(non_snake_case)]
pub fn Try_Parse_At(cursor: &mut Cursor) -> Option<Indents> {
    match Match(Parser::Parse_At(cursor)) {
        Indents::Current(token) => Some(Indents::Current(token)),
        Indents::Increase(token) => Some(Indents::Increase(token)),
        Indents::Decrease(token) => Some(Indents::Decrease(token)),
        _ => None,
    }
}

#[allow(non_snake_case)]
pub fn Try_Parse(input: &str) -> Option<Indents> {
    match Match(Parser::Parse(input)) {
        Indents::Current(token) => Some(Indents::Current(token)),
        Indents::Increase(token) => Some(Indents::Increase(token)),
        Indents::Decrease(token) => Some(Indents::Decrease(token)),
        _ => None,
    }
}

#[allow(non_snake_case)]
pub fn Match(result: Parsed) -> Indents {
    match result {
        Parsed::Pass(token) => match token.name.as_str() {
            current::KEY => Indents::Current(token).into(),
            increase::KEY => Indents::Increase(token).into(),
            decrease::KEY => Indents::Decrease(token).into(),
            _ => Indents::Error(Error::new("unknown-indent-type").build(0, 0).unwrap()),
        },
        Parsed::Fail(error) => match error {
            Some(error) => Indents::Error(error),
            None => Indents::None,
        },
    }
}
