use super::{Cursor, Term};
use itertools::PeekingNext;

pub(crate) struct _Context {
    pub indents: Vec<usize>,
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
            indents: vec![],
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
    line.extend(_lex_indentation(source, ctx));
    line.extend(_lex_content(source, ctx));

    Some(line)
}

pub fn _lex_indentation(source: &mut Cursor, ctx: &mut _Context) -> Vec<Term> {
    let start = source.peek().unwrap().0;
    source.reset_peek();

    // TODO: this doesn't actually work.
    // I need to make sure to only add indent tokens when there's an increase, otherwise there will be too many indents per dedent.
    let mut count = 0;
    loop {
        if let Some(next) = &source.peek() {
            if next.1 == '\t' || next.1 == ' ' {
                count += 1;
                source.next();
            } else {
                break;
            }
        } else {
            break;
        }
    }

    _update_indent_level(start, count, ctx)
}

pub(crate) fn _update_indent_level(at: usize, level: usize, ctx: &mut _Context) -> Vec<Term> {
    let mut dents = vec![];
    if ctx.indents.len() == 0 {
        if level != 0 {
            ctx.indents.push(0);
            for i in 0..level {
                dents.push(Term::Of_Type(Term::Type::Whitespaces::Indent, at + i));
            }
        }
    } else {
        let last = *ctx.indents.last().unwrap();
        if level > last {
            ctx.indents.push(level);
            for i in 0..(level - last) {
                dents.push(Term::Of_Type(Term::Type::Whitespaces::Indent, at + i));
            }
        } else if level < last {
            ctx.indents.pop();
            for _ in 0..(last - level) {
                dents.push(Term::Of_Type(Term::Type::Whitespaces::Dedent, at - 1));
            }
        }
    }
    dents
}

fn _lex_content(source: &mut Cursor, ctx: &mut _Context) -> Vec<Term> {
    let mut contents = vec![];
    loop {
        source.reset_peek();
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
                            continue;
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
                            let word = _lex_word(source);
                            ctx.prev_was_ws = false;
                            ctx.prev_was_delim = false;
                            ctx.prev_was_op = false;

                            word
                        }
                        // non-numeric word
                        else if c.is_alphabetic() {
                            let key_or_delimited = _lex_key_or_delimited(source);
                            ctx.prev_was_ws = false;
                            ctx.prev_was_delim = false;
                            ctx.prev_was_op = false;

                            key_or_delimited
                        }
                        // symbol
                        else {
                            let symbol = _lex_symbol(source, ctx);
                            ctx.prev_was_ws = false;

                            symbol
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
    let start = source.next().unwrap();
    let mut end = start.0;

    let mut prev: Option<(usize, char)> = None;
    let mut word = Term::Of_Type(Term::Type::Number, start.0);

    macro_rules! read {
        ($($prev:expr)?) => {
            if prev.is_some() {
                if word.ttype == Term::Type::Words::Whole {
                    word = word.ttype(Term::Type::Words::Delimited);
                }

                source.next();
                prev = None;
            }
            end = match source.next() {
                Some((i, _)) => i,
                None => end,
            };
        };
    }

    macro_rules! skip {
        ($prev:expr) => {
            prev = Some($prev);
            continue;
        };
    }

    loop {
        if let Some((i, c)) = &source.peek() {
            // word or num
            if c.is_alphanumeric() {
                // word
                if word.ttype == Term::Type::Number && !c.is_numeric() {
                    word = word.ttype(Term::Type::Words::Whole);
                }

                read!();
            } else {
                // word with symbol or end of num
                match c {
                    // word with symbol
                    '$' | '@' | '_' => {
                        if word.ttype == Term::Type::Number {
                            word = word.ttype(Term::Type::Words::Whole);
                        }

                        read!();
                    }
                    // word with symbol or end of num
                    '-' | '+' | '%' | '^' | '*' | '~' => {
                        // end of num
                        if word.ttype == Term::Type::Number {
                            break;
                        }
                        // delimited word
                        else {
                            match prev {
                                Some(_) => {
                                    // double delimiter char (not allowed in words, end without reading p)
                                    //source.reset_peek();
                                    break;
                                }
                                None => {
                                    skip!((*i, *c));
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
            break;
        }
    }

    word.end(end)
}

fn _lex_key_or_delimited(source: &mut Cursor) -> Term {
    let start = source.next().unwrap();

    let mut prev: Option<(usize, char)> = None;
    let mut end = start.0;
    let mut key_or_delimited = Term::Of_Type(Term::Type::Words::Whole, start.0);

    macro_rules! read {
        () => {
            if prev.is_some() {
                if key_or_delimited.ttype == Term::Type::Words::Whole {
                    key_or_delimited.ttype = Term::Type::Words::Delimited;
                }

                source.next();
                prev = None;
            }

            end = match source.next() {
                Some((i, _)) => i,
                None => end,
            }
        };
    }

    macro_rules! skip {
        ($prev:expr) => {
            prev = Some($prev);
            continue;
        };
    }

    loop {
        if let Some((i, c)) = source.peek() {
            if c.is_alphanumeric() {
                read!();
            } else {
                match c {
                    // key symbols
                    '$' | '@' | '_' => {
                        read!();
                    }
                    // delimited symbols
                    '-' | '+' | '%' | '^' | '~' => match prev {
                        Some((_, _)) => {
                            //source.reset_peek();
                            break;
                        }
                        None => {
                            skip!((*i, *c));
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
            break;
        }
    }

    key_or_delimited.end(end)
}

fn _lex_symbol(source: &mut Cursor, ctx: &mut _Context) -> Term {
    let start: (usize, char) = source.next().unwrap();
    let mut symbol = Term::New(start.0);
    let first = start.1;

    macro_rules! _skip_to_end_of_symbol {
        () => {{
            let mut end = start.0;
            loop {
                if let Some((_, c)) = source.peek() {
                    if c.is_whitespace() || c.is_alphanumeric() {
                        break;
                    } else {
                        end = source.next().unwrap().0;
                    }
                } else {
                    break;
                }
            }

            end
        }};
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
            symbol.end(start.0 + $num)
        }};
        ($type:path; ++) => {{
            symbol = symbol.ttype($type);

            let end = _skip_to_end_of_symbol!();

            symbol.end(end)
        }};
    }

    macro_rules! _as_unknown {
        ($cat:ident; +$num:expr) => {{
            symbol = symbol.ttype(Term::Type::$cat::Unknown);

            for _ in 0..$num {
                source.next();
            }

            ctx.prev_was_op = true;
            symbol.end(start.0 + $num)
        }};
        ($cat:ident) => {{
            symbol = symbol.ttype(Term::Type::$cat::Unknown);

            let end = _skip_to_end_of_symbol!();

            ctx.prev_was_op = true;
            symbol.end(end)
        }};
    }

    macro_rules! _as_reserved {
        (+$num:expr) => {{
            symbol = symbol.ttype(Term::Type::Reserved);
            for _ in 0..$num {
                source.next();
            }

            ctx.prev_was_op = true;
            ctx.prev_was_delim = true;
            symbol.end(start.0 + $num)
        }};
        () => {{
            symbol = symbol.ttype(Term::Type::Reserved);

            let end = _skip_to_end_of_symbol!();

            ctx.prev_was_op = true;
            ctx.prev_was_delim = true;
            symbol.end(end)
        }};
    }

    macro_rules! _as_ambiguous {
        (+$num:expr; $($types:tt)*) => {{
            symbol = symbol.ttype(Term::Type::Ambiguous(vec![$($types)*]));
            for _ in 0..$num {
                source.next();
            }

            symbol.end(start.0 + $num)
        }};
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
        ($cat:ident::$type:ident; ++) => {{
            ctx.prev_was_delim = true;
            ctx.prev_was_op = false;
            _as!(Term::Type::Delimiters::$cat::$type; ++)
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
        ($cat:ident::$type:ident; ++) => {{
            ctx.prev_was_op = true;
            ctx.prev_was_delim = false;
            _as!(Term::Type::Operators::$cat::$type; ++)
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
            if let Some((_, c)) = source.peek() {
                if c == &'#' {
                    if let Some((_, c)) = source.peek() {
                        if c == &'#' {
                            if ctx.is_start_of_line {
                                if let Some((_, c)) = source.peek() {
                                    // ####
                                    if c == &'#' {
                                        return _as_delim!(Lines::Title; ++);
                                    }
                                    // ### section
                                    else if c.is_whitespace() {
                                        return _as_delim!(Lines::Section; +2);
                                    }
                                }
                            }

                            // ### reserved
                            return _as_reserved!(+2);
                        } else if c.is_whitespace() {
                            // ## doc OR section
                            if ctx.is_start_of_line {
                                return _as_ambiguous!(+2;
                                    Term::Type::Delimiters::Lines::Doc,
                                    Term::Type::Delimiters::Lines::Section
                                );
                            }
                            // ## eol doc
                            else {
                                return _as_delim!(Lines::Doc; +1);
                            }
                        }
                    }

                    // ## tag literal
                    return _as_op!(Prefixes::TagLiteral; +1);
                } else if c.is_whitespace() {
                    // # title
                    if ctx.is_start_of_line {
                        return _as_delim!(Lines::Title);
                    }
                    // # trait modifier
                    else if ctx.prev_was_ws || ctx.prev_was_delim || ctx.prev_was_op {
                        return _as_op!(Spaceds::TraitMod);
                    } else {
                        return _as_reserved!();
                    }
                }
            }

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
                            return _as_op!(Prefixes::ArgLiteral; +1);
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
                    return _as_op!(Prefixes::Arg);
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
                return _as_reserved!();
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
