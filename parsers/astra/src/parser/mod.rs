pub(crate) mod _parser;
pub(crate) use _parser::*;

pub mod context;
pub mod cursor;
pub mod fs;
pub mod indents;
pub mod token;

pub use context::Context;
pub use cursor::{Cursor, State};
pub use indents::Indents;
pub use token::Token;

pub struct Config {
    pub skip_comments: bool,
    pub skip_whitespace: bool,
}

pub fn parse(input: &str, config: &Config) -> Token {
    let context = Context::New_Empty();
    let mut cursor = Cursor::New_With(input, context);

    let mut source = Token::New(0);
    cursor.skip_ws();

    // start first line.
    _parse_line_as_new_statement(&mut cursor, &mut source, Indents::Diff::None);

    source.end(cursor.index())
}
