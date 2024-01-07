pub(crate) mod _parser;
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub(crate) use _parser::*;

pub mod context;
pub mod cursor;
pub mod error;
pub mod fs;
pub mod indents;
pub mod term;
pub mod token;

pub use context::Context;
pub use cursor::{Cursor, Reader, Scanner, State};
pub use error::Error;
pub use indents::Indents;
pub use term::Term;
pub use token::Token;

pub struct Config {
    pub initial_indent: usize,
    pub skip_comments: bool,
    pub skip_whitespace: bool,
}

pub fn parse(input: &str, _config: &Config) -> Token {
    let context = Context::New_Empty();
    let mut cursor = Cursor::New_With(input, context);

    let mut source = cursor.token().start();
    cursor.skip_ws();

    // start first line.
    _parse_line_as_new_statement(&mut cursor, &mut source, Indents::Diff::None);

    source.end(&cursor)
}

pub fn lex(input: &str, _config: &Config) -> Vec<Term> {
    let mut terms = vec![];
    let mut source: Peekable<Enumerate<Chars>> = input.chars().enumerate().peekable();
    loop {
        let line = _lexer::_lex_line(&mut source);

        if let Some(mut line) = line {
            terms.append(&mut line);
        } else {
            break;
        }
    }

    terms
}

pub(crate) mod _lexer {
    use std::{
        iter::{Enumerate, Peekable},
        str::Chars,
    };

    use super::Term;

    pub fn _lex_line(source: &mut Peekable<Enumerate<Chars>>) -> Option<Vec<super::Term>> {
        let mut terms = vec![];

        _lex_indents(source, &mut terms);

        if !_lex_contents(source, &mut terms) {
            return None;
        }

        Some(terms)
    }

    fn _lex_contents(source: &mut Peekable<Enumerate<Chars>>, terms: &mut Vec<Term>) -> bool {
        loop {
            match &source.next() {
                None => {
                    return false;
                }
                Some((i, c)) => match c {
                    '\n' => {
                        break;
                    }
                    ' ' | '\t' => {
                        continue;
                    }
                    _ => terms.push(if c.is_numeric() {
                        _lex_alphanumeric(source)
                    } else if c.is_alphabetic() {
                        _lex_word(source)
                    } else {
                        _lex_symbol(source)
                    }),
                },
            }
        }

        true
    }

    fn _lex_alphanumeric(source: &mut Peekable<Enumerate<Chars<'_>>>) -> Term {
        let term = Term::Of_Type(Term::Type::Number, source.count());
        let mut curr = source.peek().unwrap_or(&(0, '\0'));

        loop {
            if let Some(next) = &source.peek() {
                if next.1.is_alphanumeric() {
                    if term.ttype == Term::Type::Number && !next.1.is_numeric() {
                        term.ttype = Term::Type::Word;
                    }

                    curr = next;
                    source.next();
                } else {
                    todo!();
                    // check allowed symbols here
                    break;
                }
            } else {
                return term;
            }
        }

        term.end(source.count())
    }

    fn _lex_indents(source: &mut Peekable<Enumerate<Chars<'_>>>, terms: &mut Vec<Term>) {
        loop {
            if let Some(next) = &source.peek() {
                if next.1 == '\t' || next.1 == ' ' {
                    terms.push(Term::Of_Type(
                        Term::Type::Whitespaces::Indent,
                        source.next().unwrap().0,
                    ));
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}
