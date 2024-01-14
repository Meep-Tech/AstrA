use itertools::MultiPeek;
use std::str::CharIndices;

pub(crate) mod _lexer;
// pub(crate) mod _parser;

pub mod context;
//pub mod error;
pub mod fs;
pub mod indents;
pub mod term;
// pub mod token;

pub use context::Context;
//pub use error::Error;
pub use indents::Indents;
pub use term::Term;
// pub use token::Token;

pub type Cursor<'a> = MultiPeek<CharIndices<'a>>;

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
    let mut terms = vec![];
    let mut source: Cursor = itertools::multipeek(input.char_indices());
    let mut ctx = _lexer::_Context::New();

    loop {
        if source.peek().is_none() {
            let dedents = _lexer::_update_indent_level(input.len(), 0, &mut ctx);
            terms.extend(dedents);

            break;
        } else {
            source.reset_peek();
        }

        let line = _lexer::_lex_line(&mut source, &mut ctx);
        terms.extend(line.unwrap());
    }

    terms
}
