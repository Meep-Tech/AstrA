use itertools::MultiPeek;
use std::{str::CharIndices, usize};

pub(crate) mod _lexer;
// pub(crate) mod _parser;

pub mod context;
pub mod cursor;
pub mod fs;
pub mod indents;
pub mod symbol;
pub mod term;
// pub mod token;
//pub mod error;

pub use context::Context;
pub use indents::Indents;
pub use symbol::Symbol;
pub use term::Term;
// pub use token::Token;
//pub use error::Error;
pub use cursor::Cursor;

pub type Source<'a> = MultiPeek<CharIndices<'a>>;

pub struct Config {
    pub initial_indent: usize,
    pub skip_comments: bool,
    pub skip_whitespace: bool,
}

// pub fn parse(input: &str, config: &Config) -> Token {
//     let _terms = lex(input, config);
//     todo!()
// }

pub fn lex(input: &str) -> Vec<Term> {
    let config = Config {
        initial_indent: 0,
        skip_comments: true,
        skip_whitespace: true,
    };

    lex_with(input, &config)
}

pub fn lex_with(input: &str, _config: &Config) -> Vec<Term> {
    let mut source: Source = itertools::multipeek(input.char_indices());
    let mut ctx = Cursor::New(source);

    while !ctx.is_eof() {
        _lexer::_lex_line(&mut ctx);
    }

    ctx.end()
}
