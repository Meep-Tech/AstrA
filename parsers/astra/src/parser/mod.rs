use itertools::MultiPeek;
use std::str::CharIndices;

pub(crate) mod _lexer;
// pub(crate) mod _parser;

pub mod context;
pub mod error;
pub mod fs;
pub mod indents;
pub mod term;
pub mod token;

pub use context::Context;
pub use error::Error;
pub use indents::Indents;
pub use term::Term;
pub use token::Token;

pub type Cursor<'a> = MultiPeek<CharIndices<'a>>;

pub struct Config {
    pub initial_indent: usize,
    pub skip_comments: bool,
    pub skip_whitespace: bool,
}

pub fn parse(input: &str, config: &Config) -> Token {
    let _terms = lex(input, config);
    // let context = Context::New_Empty();
    // let mut cursor = Cursor::New_With(input, context);

    // let mut source = cursor.token().start();
    // cursor.skip_ws();

    // // start first line.
    // _parse_line_as_new_statement(&mut cursor, &mut source, Indents::Diff::None);

    // source.end(&cursor)
    todo!()
}

pub fn lex(input: &str, _config: &Config) -> Vec<Term> {
    let mut terms = vec![];
    let mut source: Cursor = itertools::multipeek(input.char_indices());
    let mut ctx = _lexer::_Context::New();

    loop {
        if source.peek().is_none() {
            break;
        } else {
            source.reset_peek();
        }

        let line = _lexer::_lex_line(&mut source, &mut ctx);
        terms.extend(line.unwrap());
    }

    terms
}
