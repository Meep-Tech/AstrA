use super::{Cursor, Term};
use itertools::PeekingNext;

pub(crate) struct _Context {
    pub generic_depth: usize,
    pub prev_was_delim: bool,
    pub prev_was_ws: bool,
    pub prev_was_op: bool,
    pub is_start_of_line: bool,
}

impl _Context {
    #[allow(non_snake_case)]
    pub fn New() -> Self {
        Self {
            generic_depth: 0,
            is_start_of_line: true,
            prev_was_ws: true,
            prev_was_delim: false,
            prev_was_op: false,
        }
    }
}

pub(crate) fn _lex_line(source: &mut Cursor, ctx: &mut _Context) -> Option<Vec<super::Term>> {
    let mut line = vec![];

    ctx.is_start_of_line = true;
    line.extend(_lex_indents(source));
    line.extend(_lex_contents(source, ctx));

    Some(line)
}

fn _lex_indents(source: &mut Cursor) -> Vec<Term> {
    source.reset_peek();

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

fn _lex_contents(source: &mut Cursor, ctx: &mut _Context) -> Vec<Term> {
    source.reset_peek();

    let mut contents = vec![];
    loop {
        let n = source.peek();
        match n {
            // eof
            None => {
                break;
            }
            // any
            Some((_, ref c)) => {
                let c = *c;
                match c {
                    // ws
                    ' ' | '\t' => {
                        while let Some((_, _)) =
                            source.peeking_next(|next| next.1 == ' ' || next.1 == '\t')
                        {
                            source.next();
                        }

                        ctx.prev_was_ws = true;
                        ctx.prev_was_delim = false;
                        ctx.prev_was_op = false;
                        continue;
                    }
                    // eol
                    '\n' => {
                        source.next();
                        source.peeking_next(|next| next.1 == '\r');
                        break;
                    }
                    // eol OR ignored
                    '\r' => {
                        source.next();

                        // eol when with newline
                        if let Some(_) = source.peeking_next(|next| next.1 == '\n') {
                            break;
                        }
                        // else ignored
                        else {
                            continue;
                        }
                    } // potentially numeric word
                    _ => {
                        contents.push(if c.is_numeric() {
                            ctx.prev_was_ws = false;
                            ctx.prev_was_delim = false;
                            ctx.prev_was_op = false;
                            _lex_word(source)
                        }
                        // non-numeric word
                        else if c.is_alphabetic() {
                            ctx.prev_was_ws = false;
                            ctx.prev_was_delim = false;
                            ctx.prev_was_op = false;
                            _lex_key_or_delimited(source)
                        }
                        // symbol
                        else {
                            ctx.prev_was_ws = false;
                            _lex_symbol(source, ctx)
                        });

                        ctx.is_start_of_line = false;
                    }
                }
            }
        }
    }

    ctx.prev_was_ws = true;
    ctx.prev_was_delim = false;
    ctx.prev_was_op = false;
    contents
}

fn _lex_word(source: &mut Cursor) -> Term {
    source.next();

    let mut p: Option<(usize, char)> = None;
    let mut word = Term::Of_Type(Term::Type::Words::Number, source.count());

    macro_rules! read {
        ($($prev:expr)?) => {
            if p.is_some() {
                if word.ttype == Term::Type::Words::Key {
                    word = word.ttype(Term::Type::Words::Delimited);
                }

                source.next();
                p = None;
            }
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
            // word or num
            if n.is_alphanumeric() {
                // word
                if word.ttype == Term::Type::Words::Number && !n.is_numeric() {
                    word = word.ttype(Term::Type::Words::Key);
                }

                read!();
            } else {
                // word with symbol or end of num
                match n {
                    // word with symbol
                    '$' | '@' | '_' => {
                        if word.ttype == Term::Type::Words::Number {
                            word = word.ttype(Term::Type::Words::Key);
                        }

                        read!();
                    }
                    // word with symbol or end of num
                    '-' | '+' | '%' | '^' | '*' | '~' => {
                        // end of num
                        if word.ttype == Term::Type::Words::Number {
                            break;
                        }
                        // delimited word
                        else {
                            match p {
                                Some((_, _)) => {
                                    // double delimiter char (not allowed in words, end without reading p)
                                    source.reset_peek();
                                    break;
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
    let mut key_or_delimited = Term::Of_Type(Term::Type::Words::Key, source.count());

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
                        Some((_, _)) => {
                            source.reset_peek();
                            break;
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

fn _lex_symbol(source: &mut Cursor, ctx: &mut _Context) -> Term {
    let next: (usize, char) = source.next().unwrap();
    let mut symbol = Term::New(next.0);
    let first = next.1;

    macro_rules! _skip_to_end_of_symbol {
        () => {
            loop {
                if let Some((_, c)) = source.peek() {
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

    macro_rules! _as {
        ($type:path) => {{
            symbol.ttype($type)
        }};
        ($type:path; +$num:expr) => {{
            symbol = symbol.ttype($type);
            for _ in 0..$num {
                source.next();
            }
            symbol.end(source.count())
        }};
    }

    macro_rules! _as_unknown {
        ($cat:ident; +$num:expr) => {{
            symbol = symbol.ttype(Term::Type::$cat::Unknown);

            for _ in 0..$num {
                source.next();
            }

            symbol.end(source.count())
        }};
        ($cat:ident) => {{
            symbol = symbol.ttype(Term::Type::$cat::Unknown);

            _skip_to_end_of_symbol!();

            symbol.end(source.count())
        }};
    }

    macro_rules! _as_reserved {
        () => {{
            symbol.ttype(Term::Type::Reserved)
        }};
        (+$num:expr) => {{
            symbol = symbol.ttype(Term::Type::Reserved);
            for _ in 0..$num {
                source.next();
            }

            symbol.end(source.count())
        }};
    }

    macro_rules! _as_ambiguous {
        ($($types:tt)*) => {{
            symbol.ttype(Term::Type::Ambiguous(vec![$($types)*]))
        }};
    }

    macro_rules! _as_delim {
        ($cat:ident::$type:ident) => {{
            ctx.prev_was_delim = true;
            ctx.prev_was_op = false;
            _as!(Term::Type::Delimiters::$cat::$type)
        }};
        ($cat:ident::$type:ident; +$num:expr) => {{
            ctx.prev_was_delim = true;
            ctx.prev_was_op = false;
            _as!(Term::Type::Delimiters::$cat::$type; +$num)
        }};
    }

    macro_rules! _as_op {
        ($cat:ident::$type:ident) => {{
            ctx.prev_was_op = true;
            ctx.prev_was_delim = false;
            _as!(Term::Type::Operators::$cat::$type)
        }};
        ($cat:ident::$type:ident; +$num:expr) => {{
            ctx.prev_was_op = true;
            ctx.prev_was_delim = false;
            _as!(Term::Type::Operators::$cat::$type; +$num)
        }};
    }

    match first {
        '{' => {
            return _as_delim!(Starts::Map);
        }
        '}' => {
            return _as_delim!(Ends::Map);
        }
        '[' => {
            return _as_delim!(Starts::Array);
        }
        ']' => {
            return _as_delim!(Ends::Array);
        }
        '(' => {
            return _as_delim!(Starts::Group);
        }
        ')' => {
            return _as_delim!(Ends::Group);
        }
        ',' => {
            return _as_delim!(Separators::Entry);
        }
        '#' => {
            return _as_op!(Prefixes::Tag);
        }
        '.' => {
            if let Some((_, c)) = source.peek() {
                if c == &'#' {
                    return _as_op!(Lookups::Tag; +1);
                } else if c == &'.' {
                    if let Some((_, c)) = source.peek() {
                        if c == &'.' {
                            if ctx.prev_was_delim || ctx.prev_was_ws || ctx.prev_was_op {
                                return _as_op!(Prefixes::Spread; +2);
                            } else {
                                return _as_op!(Chaineds::Range; +2);
                            }
                        } else {
                            return _as_op!(Lookups::Parent; +1);
                        }
                    }
                } else if c == &'?' {
                    return _as_op!(Lookups::Query; +1);
                }
            }
            return _as_op!(Lookups::Dot);
        }
        '/' => {
            if let Some((_, c)) = source.peek() {
                // / division
                if c.is_whitespace() {
                    todo!("math /");
                } else if c == &'/' {
                    if let Some((_, c)) = source.peek() {
                        // /// line comment
                        if c == &'/' {
                            return _as_delim!(Lines::Comment; +2);
                        }
                        // // line comment
                        else if ctx.prev_was_ws
                            || ctx.prev_was_delim
                            || ctx.prev_was_op
                            || c.is_whitespace()
                        {
                            return _as_delim!(Lines::Comment; +1);
                        }
                        // // division (Chained)
                        else {
                            todo!("math //");
                        }
                    }
                    // // line comment
                    else {
                        return _as_delim!(Lines::Comment; +1);
                    }
                }
                // /* block comment
                else if c == &'*' {
                    return _as_delim!(Starts::Comment; +1);
                }
                // / lookup
                else {
                    return _as_op!(Lookups::Slash);
                }
            }
            // / lookup
            else {
                return _as_op!(Lookups::Slash);
            }
        }
        ';' => {
            // ;; pipe operator
            if let Some((_, c)) = source.peek()
                && c == &';'
            {
                return _as!(Term::Type::Operators::Betweens::Pipe; +1);
            }
            // ; expression terminator
            else {
                return _as_delim!(Separators::Expression);
            }
        }
        '<' => {
            if let Some((_, c)) = source.peek() {
                if c == &'<' {
                    if let Some((_, c)) = source.peek() {
                        // <<< reserved
                        if c == &'<' {
                            return _as_reserved!(+2);
                        }
                    }
                    // << reserved
                    return _as_reserved!(+1);
                }
                // < generic start
                else {
                    ctx.generic_depth += 1;
                    return _as_delim!(Ends::Generic);
                }
            } else {
                return _as_unknown!(Operators);
            }
        }
        '>' => {
            macro_rules! _as_generic_end {
                () => {{
                    ctx.generic_depth -= 1;
                    _as_delim!(Ends::Generic)
                }};
            }

            if let Some((_, c)) = source.peek() {
                // > generic end
                if c.is_whitespace() {
                    return _as_generic_end!();
                } else if c == &'>' {
                    if let Some((_, c)) = source.peek() {
                        // >>> reserved
                        if c == &'>' {
                            return _as_reserved!(+2);
                        }
                    }

                    // >> proc assigner
                    return _as!(Term::Type::Operators::Betweens::ProcAssigner; +1);
                } else if ctx.prev_was_ws {
                    // > generic end OR input prefix
                    if ctx.generic_depth > 0 {
                        ctx.generic_depth -= 1;
                        ctx.prev_was_delim = true;
                        ctx.prev_was_op = true;
                        return _as_ambiguous!(
                            Term::Type::Delimiters::Ends::Generic,
                            Term::Type::Operators::Prefixes::Input
                        );
                    } else {
                        return _as!(Term::Type::Operators::Prefixes::Input);
                    }
                }
            }

            return _as_generic_end!();
        }
        ':' => {
            if let Some((_, c)) = source.peek() {
                if c == &':' {
                    if let Some((_, c)) = source.peek() {
                        // ::: final field assigner
                        if c == &':' {
                            return _as_op!(Suffixes::FinalFieldAssigner; +2);
                        }

                        // :: const field assigner
                        if c.is_whitespace() || c.is_delimiter() {
                            return _as_op!(Suffixes::ConstFieldAssigner; +1);
                        }
                        // :: single arg literal prefix
                        else if ctx.prev_was_delim || ctx.prev_was_ws {
                            return _as_op!(Prefixes::SingleArgLiteral; +1);
                        }
                        // reserved call of some kind
                        else {
                            return _as_reserved!(+1);
                        }
                    }

                    // :: const field assigner
                    return _as_op!(Suffixes::ConstFieldAssigner; +1);
                }

                // : mutable field assigner
                if c.is_whitespace() || c.is_delimiter() {
                    return _as_op!(Suffixes::MutableFieldAssigner);
                }
                // : single arg prefix
                else if ctx.prev_was_delim || ctx.prev_was_ws {
                    return _as_op!(Prefixes::SingleArg);
                }
                // : chained call
                else {
                    return _as_op!(Chaineds::Caller);
                }
            }

            // : mutable field assigner
            return _as_op!(Suffixes::MutableFieldAssigner);
        }
        '|' => {
            if let Some((_, c)) = source.peek() {
                // || double or
                if c == &'|' {
                    return _as_op!(Chaineds::Or; +1);
                }
                // | single or
                else if c.is_whitespace() {
                    return _as_op!(Spaceds::Or);
                }
                // | alias prefix
                else {
                    return _as_op!(Prefixes::Alias);
                }
            } else if ctx.prev_was_delim || ctx.prev_was_op {
                // | alias prefix
                return _as_op!(Prefixes::Alias);
            } else if ctx.prev_was_ws {
                // | single or
                return _as_op!(Spaceds::Or);
            } else {
                return _as_unknown!(Suffixes);
            }
        }
        _ => {
            return _as_unknown!(Operators);
        }
    }
}

trait MightBeDelimiter {
    fn is_delimiter(&self) -> bool;
}

impl MightBeDelimiter for char {
    fn is_delimiter(&self) -> bool {
        match self {
            '{' | '}' | '[' | ']' | '(' | ')' | ',' | ';' => true,
            _ => false,
        }
    }
}
