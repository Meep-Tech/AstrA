use itertools::MultiPeek;
use std::str::CharIndices;

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

pub fn parse(input: &str, _config: &Config) -> Token {
    todo!()
    // let context = Context::New_Empty();
    // let mut cursor = Cursor::New_With(input, context);

    // let mut source = cursor.token().start();
    // cursor.skip_ws();

    // // start first line.
    // _parse_line_as_new_statement(&mut cursor, &mut source, Indents::Diff::None);

    // source.end(&cursor)
}

pub fn lex(input: &str, _config: &Config) -> Vec<Term> {
    let mut terms = vec![];
    let mut source: Cursor = itertools::multipeek(input.char_indices());
    loop {
        if source.peek().is_none() {
            break;
        } else {
            source.reset_peek();
        }

        let line = _lexer::_lex_line(&mut source);
    }

    terms
}

pub(crate) mod _lexer {
    use super::{Cursor, Term};
    use itertools::PeekingNext;

    pub fn _lex_line(source: &mut Cursor) -> Option<Vec<super::Term>> {
        let mut line = vec![];

        line.extend(_lex_indents(source));
        line.extend(_lex_contents(source));

        Some(line)
    }

    fn _lex_indents(source: &mut Cursor) -> Vec<Term> {
        let mut indents = vec![];
        loop {
            if let Some(next) = &source.peek() {
                if next.1 == '\t' || next.1 == ' ' {
                    indents.push(Term::Of_Type(
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

        indents
    }

    fn _lex_contents(source: &mut Cursor) -> Vec<Term> {
        let mut contents = vec![];
        let mut prev = '\n';

        loop {
            match &source.peek() {
                // eof
                None => {
                    break;
                }
                // any
                Some((i, c)) => {
                    match c {
                        // ws
                        ' ' | '\t' => {}
                        // eol
                        '\n' => {
                            source.next();
                            source.peeking_next(|next| next.1 == '\r');
                            break;
                        }
                        // eol
                        '\r' => {
                            if let Some(_) = source.peeking_next(|next| next.1 == '\n') {
                                break;
                            }
                        } // potentially numeric word
                        _ => {
                            contents.push(if c.is_numeric() {
                                _lex_word(source)
                            }
                            // non-numeric word
                            else if c.is_alphabetic() {
                                _lex_key_or_delimited(source)
                            }
                            // symbol
                            else {
                                _lex_symbol(source, prev.is_whitespace())
                            });
                        }
                    }

                    prev = *c;
                }
            }
        }

        contents
    }

    fn _lex_word(source: &mut Cursor) -> Term {
        source.next();

        let mut p: Option<(usize, char)> = None;
        let word = Term::Of_Type(Term::Type::Words::Number, source.count());

        macro_rules! read {
            ($($prev:expr)?) => {
                if p.is_some() {
                    if word.ttype == Term::Type::Words::Key {
                        word.ttype = Term::Type::Words::Delimited;
                    }

                    source.next();
                    p = None;
                }
                source.next();
            };
        }

        macro_rules! read_p {
            ($($prev:expr)?) => {
                if word.ttype != Term::Type::Words::Key {
                    word.ttype = Term::Type::Words::Delimited;
                }

                source.next();
                p = source.next();
            };
        }

        macro_rules! skip {
            ($prev:expr) => {
                p = Some($prev);
                continue;
            };
        }

        loop {
            if let Some((i, n)) = &source.peek() {
                // word or num
                if n.is_alphanumeric() {
                    // word
                    if word.ttype == Term::Type::Words::Number && !n.is_numeric() {
                        word.ttype = Term::Type::Words::Key;
                    }

                    read!();
                } else {
                    // word with symbol or end of num
                    match n {
                        // word with symbol
                        '$' | '@' | '_' => {
                            if word.ttype == Term::Type::Words::Number {
                                word.ttype = Term::Type::Words::Key;
                            }

                            read!();
                        }
                        // word with symbol or end of num
                        '-' | '+' | '%' | '^' | '~' => {
                            // end of num
                            if word.ttype == Term::Type::Words::Number {
                                break;
                            }
                            // delimited word
                            else {
                                match p {
                                    Some((_, prev)) => {
                                        // double delimiter char (not allowed in words, end without reading p)
                                        if prev == *n {
                                            source.reset_peek();
                                            break;
                                        } else {
                                            read_p!();
                                        }
                                    }
                                    None => {
                                        skip!((*i, *n));
                                    }
                                }
                            }
                        }
                        // non-word symbol
                        _ => {
                            break;
                        }
                    };
                };
            }
            // eof
            else {
                return word;
            }
        }

        word.end(source.count())
    }

    fn _lex_key_or_delimited(source: &mut Cursor) -> Term {
        source.next();

        let mut p: Option<(usize, char)> = None;
        let key_or_delimited = Term::Of_Type(Term::Type::Words::Key, source.count());

        macro_rules! read {
            () => {
                if p.is_some() {
                    if key_or_delimited.ttype == Term::Type::Words::Key {
                        key_or_delimited.ttype = Term::Type::Words::Delimited;
                    }

                    source.next();
                    p = None;
                }
                source.next();
            };
        }

        macro_rules! read_p {
            () => {
                if key_or_delimited.ttype == Term::Type::Words::Key {
                    key_or_delimited.ttype = Term::Type::Words::Delimited;
                }

                p = source.next();
                source.next();
            };
        }

        macro_rules! skip {
            ($prev:expr) => {
                p = Some($prev);
                continue;
            };
        }

        loop {
            if let Some((i, n)) = &source.peek() {
                if n.is_alphanumeric() {
                    read!();
                } else {
                    match n {
                        // key symbols
                        '$' | '@' | '_' => {
                            read!();
                        }
                        // delimited symbols
                        '-' | '+' | '%' | '^' | '~' => match p {
                            Some((_, prev)) => {
                                if prev == *n {
                                    source.reset_peek();
                                    break;
                                } else {
                                    read_p!();
                                }
                            }
                            None => {
                                skip!((*i, *n));
                            }
                        },
                        // non-word symbol
                        _ => {
                            break;
                        }
                    };
                };
            }
            // eof
            else {
                return key_or_delimited;
            }
        }

        key_or_delimited.end(source.count())
    }

    fn _lex_symbol(source: &mut Cursor, prev_was_ws: bool) -> Term {
        let first = source.next().unwrap().1;

        macro_rules! _skip_to_end_of_symbol {
            () => {
                loop {
                    if let Some((i, c)) = source.peek() {
                        if c.is_whitespace() || c.is_alphanumeric() {
                            break;
                        } else {
                            source.next();
                        }
                    } else {
                        break;
                    }
                }
            };
        }

        macro_rules! _lex_unknown {
            ($cat:ident $(, - $offset:expr)?) => {{
                let offset: usize = 0 $(- $offset)?;
                let mut unknown = Term::Of_Type(Term::Type::$cat::Unknown, source.count() - 1);

                _skip_to_end_of_symbol!();

                unknown.end(source.count())
            }
        }}

        macro_rules! _lex_reserved {
            ($(- $offset:expr)?) => {{
                let offset: usize = 0 $(- $offset)?;
                let mut unknown = Term::Of_Type(Term::Type::Reserved, source.count() - 1);

                _skip_to_end_of_symbol!();

                unknown.end(source.count())
            }
        }}

        match first {
            '{' => {
                return Term::Of_Type(Term::Type::Delimiters::MapStart, source.count());
            }
            '}' => {
                return Term::Of_Type(Term::Type::Delimiters::MapEnd, source.count());
            }
            '[' => {
                return Term::Of_Type(Term::Type::Delimiters::ArrayStart, source.count());
            }
            ']' => {
                return Term::Of_Type(Term::Type::Delimiters::ArrayEnd, source.count());
            }
            '(' => {
                return Term::Of_Type(Term::Type::Delimiters::GroupStart, source.count());
            }
            ')' => {
                return Term::Of_Type(Term::Type::Delimiters::GroupEnd, source.count());
            }
            ',' => {
                return Term::Of_Type(Term::Type::Delimiters::EntrySeperator, source.count());
            }
            ';' => {
                // ;; pipe
                if let Some((_, c)) = source.peek()
                    && c == &';'
                {
                    let mut symbol =
                        Term::Of_Type(Term::Type::Operators::Betweens::Pipe, source.count());
                    source.next();
                    return symbol.end(source.count());
                }
                // ; expression terminator
                else {
                    return Term::Of_Type(
                        Term::Type::Delimiters::ExpressionTerminator,
                        source.count(),
                    );
                }
            }
            '<' => {
                if prev_was_ws {
                    return Term::Of_Type(Term::Type::Delimiters::GenericStart, source.count());
                } else {
                    return _lex_unknown!(Operators, -1);
                }
            }
            '>' => {
                if let Some((_, c)) = source.peek() {
                    if c.is_whitespace() {
                        return Term::Of_Type(Term::Type::Delimiters::GenericEnd, source.count());
                    } else if c == &'>' {
                        if let Some((_, c)) = source.peek() {
                            if c.is_whitespace() {
                            } else if c == &'>' {
                                _lex_reserved!(-2);
                            }
                        }
                    } else if prev_was_ws {
                        return Term::Of_Type(
                            Term::Type::Operators::Prefixes::Input,
                            source.count(),
                        );
                    }
                }

                return Term::Of_Type(Term::Type::Delimiters::GenericEnd, source.count());
            }
            _ => {
                return _lex_unknown!(Operators);
            }
        }
    }
}
