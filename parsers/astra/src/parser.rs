use self::cursor::Cursor;
use crate::token::{Source, Token};

pub mod cursor;
pub mod try_as;

pub fn parse(src: &str, ctx: Source) -> Token {
    let cursor = Cursor::new(src, ctx);
    match try_as::source(cursor) {
        try_as::Result::Match(token) => token,
        try_as::Result::None => panic!("Failed to parse source"),
    }
}
