use std::collections::{HashMap, HashSet};

use super::{
    fs::Type,
    term::{self, Cadence},
    Cursor, Term,
};

// pub(crate) struct _Context {
//     pub indents: Vec<usize>,
//     pub generic_depth: usize,
//     pub prev_was_delim: bool,
//     pub prev_was_ws: bool,
//     pub prev_was_op: bool,
//     pub is_start_of_line: bool,
// }

// impl _Context {
//     #[allow(non_snake_case)]
//     pub fn New() -> Self {
//         Self {
//             indents: vec![],
//             generic_depth: 0,
//             is_start_of_line: true,
//             prev_was_ws: true,
//             prev_was_delim: false,
//             prev_was_op: false,
//         }
//     }
// }

pub(crate) fn _lex_line(ctx: &mut Cursor) {
    ctx.start_line();
    _lex_indentation(ctx);
    _lex_content(ctx);
}

pub(crate) fn _lex_indentation(ctx: &mut Cursor) {
    let mut prev_level = *ctx.indents().last().unwrap_or(&0);

    let mut new_level = 0;
    while ctx.read_if_in(&[' ', '\t']) {
        new_level += 1;
    }

    match ctx.next() {
        Some('\n') => {}
        _ => {
            if new_level > prev_level {
                ctx.increase_indent(new_level);
                let start_of_increase = ctx.index() - new_level;
                for i in 0..(new_level - prev_level) {
                    ctx.push_as_term_at(Term::Type::Whitespaces::Indent, start_of_increase + i);
                }
            } else {
                while ctx.indents().last().unwrap_or(&0) > &new_level {
                    ctx.decrease_indent();
                    for _ in 0..(prev_level - new_level) {
                        ctx.push_prev_as_term(Term::Type::Whitespaces::Dedent);
                    }
                }
            }
        }
    }
}

fn _lex_content(ctx: &mut Cursor) {
    loop {
        match ctx.next() {
            // eof
            None => break,
            // any
            Some(c) => {
                match c {
                    // ws
                    ' ' | '\t' => {
                        ctx.read_spacing();
                        continue;
                    }
                    // eol
                    '\n' => {
                        ctx.read();
                        ctx.read_if_is('\r');
                        break;
                    }
                    // eol OR ignored
                    '\r' => {
                        ctx.read();

                        // eol when with newline
                        if ctx.read_if_is('\n') {
                            break;
                        }
                        // else ignored
                        else {
                            continue;
                        }
                    } // potentially numeric word
                    _ => {
                        if c.is_numeric() {
                            _lex_number_or_word(ctx);
                        }
                        // non-numeric word
                        else if c.is_alphabetic() {
                            _lex_word(ctx);
                        }
                        // symbol
                        else {
                            _old_lex_symbol(ctx);
                        }

                        continue;
                    }
                }
            }
        }
    }
}

fn _lex_number_or_word(ctx: &mut Cursor) {
    let mut ttype = Term::Type::Number;
    let mut tailing_underscore_count = 0;
    let mut prev_was_delim = false;

    while let Some(c) = ctx.peek() {
        if c.is_numeric() {
            tailing_underscore_count = 0;
            continue;
        } else if c.is_alphabetic() {
            if ttype == Term::Type::Number {
                ttype = Term::Type::Words::Whole;
            }

            prev_was_delim = false;
            tailing_underscore_count = 0;
            continue;
        } else {
            match c {
                '$' | '@' | '_' => {
                    if ttype == Term::Type::Number {
                        ttype = Term::Type::Words::Whole;
                    }

                    if c == '_' {
                        tailing_underscore_count += 1;
                    } else {
                        tailing_underscore_count = 0;
                    }

                    prev_was_delim = false;
                    continue;
                }
                '-' | '+' | '%' | '^' | '*' | '~' => {
                    if ttype == Term::Type::Number {
                        match ctx.sneak_peek() {
                            Some(n) if n.is_numeric() => {
                                break;
                            }
                            Some(n) if n.is_alphabetic() => {
                                ttype = Term::Type::Words::Delimited;
                            }
                            _ => {
                                break;
                            }
                        }
                    } else if (ctx.prev_peek().unwrap()) == c {
                        ctx.push_as_term(ttype);
                        ctx.read_peeked_minus(2);
                        ctx.end_term();
                        return;
                    }

                    if ttype != Term::Type::Words::Delimited {
                        ttype = Term::Type::Words::Delimited;
                    }

                    prev_was_delim = true;
                    tailing_underscore_count = 0;
                    continue;
                }
                _ => {
                    ctx.push_as_term(ttype);
                    break;
                }
            }
        }
    }

    if tailing_underscore_count > 0 {
        ctx.read_peeked_minus(tailing_underscore_count);
    } else if prev_was_delim {
        ctx.read_peeked_minus(2);
    } else {
        ctx.read_to_prev_peek();
    }

    ctx.end_term();
}

fn _lex_word(ctx: &mut Cursor) {
    let mut ttype = Term::Type::Words::Whole;
    let mut tailing_underscore_count = 0;
    let mut prev_was_delim = false;
    while let Some(c) = ctx.peek() {
        if !c.is_alphanumeric() {
            match c {
                // key symbols
                '$' | '@' => {
                    prev_was_delim = false;
                    tailing_underscore_count = 0;
                    continue;
                }
                '_' => {
                    prev_was_delim = false;
                    tailing_underscore_count += 1;
                    continue;
                }
                // delimited symbols
                '-' | '+' | '%' | '^' | '*' | '~' => {
                    if prev_was_delim && ctx.prev_peek().unwrap() == c {
                        ctx.push_as_term(ttype);
                        ctx.read_peeked_minus(2);
                        ctx.end_term();
                        return;
                    }

                    ttype = Term::Type::Words::Delimited;

                    prev_was_delim = true;
                    tailing_underscore_count = 0;
                    continue;
                }
                // non-word symbol
                _ => {
                    break;
                }
            };
        }
    }

    if tailing_underscore_count > 0 {
        ctx.read_peeked_minus(tailing_underscore_count);
    } else if prev_was_delim {
        ctx.read_peeked_minus(2);
    } else {
        ctx.read_to_prev_peek();
    }

    ctx.end_term();
}

enum Sym {
    Type(Symbol),
    Switch(SymbolSwitch),
    Map(SymbolMap),
}

struct Symbol {
    chars: Vec<char>,
    cadences: HashSet<Cadence>,
    ttype: term::Symbol,
}

struct SymbolSwitch {
    __: HashMap<Cadence, Sym>,
}

struct SymbolMap {
    __: HashMap<char, Sym>,
}

fn _lex_symbol(ctx: &mut Cursor) {
    let mut symbols: &SymbolMap = _get_all_symbols();

    while let Some(c) = ctx.peek() {
        let sym = symbols.__.get(&c);
        match sym {
            None => {
                // read as unknown
            }
            Some(Sym::Type(sym)) => {
                // try to read as type; or unknown
            }
            Some(Sym::Switch(sym)) => {
                // check cadences and try to read as any matching types; or unknown
            }
            Some(Sym::Map(sym)) => {
                // narrow down options, and continue
                symbols = sym;
                continue;
            }
        }
    }
}

fn _get_all_symbols() -> &'static SymbolMap {
    todo!()
}

// fn _old_lex_symbol(ctx: &mut Cursor) {
//     let start: (usize, char) = ctx.next().unwrap();
//     let mut symbol = Term::New(start.0);
//     let first = start.1;

//     macro_rules! _skip_to_end_of_symbol {
//         () => {{
//             let mut end = start.0;
//             loop {
//                 if let Some((_, c)) = source.peek() {
//                     if c.is_whitespace() || c.is_alphanumeric() {
//                         break;
//                     } else {
//                         end = source.next().unwrap().0;
//                     }
//                 } else {
//                     break;
//                 }
//             }

//             end
//         }};
//     }

//     macro_rules! _as {
//         ($type:path) => {{
//             symbol.ttype($type)
//         }};
//         ($type:path; +$num:expr) => {{
//             symbol = symbol.ttype($type);
//             for _ in 0..$num {
//                 source.next();
//             }
//             symbol.end(start.0 + $num)
//         }};
//         ($type:path; ++) => {{
//             symbol = symbol.ttype($type);

//             let end = _skip_to_end_of_symbol!();

//             symbol.end(end)
//         }};
//     }

//     macro_rules! _as_unknown {
//         ($cat:ident; +$num:expr) => {{
//             symbol = symbol.ttype(Term::Type::$cat::Unknown);

//             for _ in 0..$num {
//                 source.next();
//             }

//             ctx.prev_was_op = true;
//             symbol.end(start.0 + $num)
//         }};
//         ($cat:ident) => {{
//             symbol = symbol.ttype(Term::Type::$cat::Unknown);

//             let end = _skip_to_end_of_symbol!();

//             ctx.prev_was_op = true;
//             symbol.end(end)
//         }};
//     }

//     macro_rules! _as_reserved {
//         (+$num:expr) => {{
//             symbol = symbol.ttype(Term::Type::Reserved);
//             for _ in 0..$num {
//                 source.next();
//             }

//             ctx.prev_was_op = true;
//             ctx.prev_was_delim = true;
//             symbol.end(start.0 + $num)
//         }};
//         () => {{
//             symbol = symbol.ttype(Term::Type::Reserved);

//             let end = _skip_to_end_of_symbol!();

//             ctx.prev_was_op = true;
//             ctx.prev_was_delim = true;
//             symbol.end(end)
//         }};
//     }

//     macro_rules! _as_ambiguous {
//         (+$num:expr; $($types:tt)*) => {{
//             symbol = symbol.ttype(Term::Type::Ambiguous(vec![$($types)*]));
//             for _ in 0..$num {
//                 source.next();
//             }

//             symbol.end(start.0 + $num)
//         }};
//         ($($types:tt)*) => {{
//             symbol.ttype(Term::Type::Ambiguous(vec![$($types)*]))
//         }};
//     }

//     macro_rules! _as_delim {
//         ($cat:ident::$type:ident) => {{
//             ctx.prev_was_delim = true;
//             ctx.prev_was_op = false;
//             _as!(Term::Type::Delimiters::$cat::$type)
//         }};
//         ($cat:ident::$type:ident; +$num:expr) => {{
//             ctx.prev_was_delim = true;
//             ctx.prev_was_op = false;
//             _as!(Term::Type::Delimiters::$cat::$type; +$num)
//         }};
//         ($cat:ident::$type:ident; ++) => {{
//             ctx.prev_was_delim = true;
//             ctx.prev_was_op = false;
//             _as!(Term::Type::Delimiters::$cat::$type; ++)
//         }};
//     }

//     macro_rules! _as_op {
//         ($cat:ident::$type:ident) => {{
//             ctx.prev_was_op = true;
//             ctx.prev_was_delim = false;
//             _as!(Term::Type::Operators::$cat::$type)
//         }};
//         ($cat:ident::$type:ident; +$num:expr) => {{
//             ctx.prev_was_op = true;
//             ctx.prev_was_delim = false;
//             _as!(Term::Type::Operators::$cat::$type; +$num)
//         }};
//         ($cat:ident::$type:ident; ++) => {{
//             ctx.prev_was_op = true;
//             ctx.prev_was_delim = false;
//             _as!(Term::Type::Operators::$cat::$type; ++)
//         }};
//     }

//     match first {
//         '{' => {
//             return _as_delim!(Starts::Map);
//         }
//         '}' => {
//             return _as_delim!(Ends::Map);
//         }
//         '[' => {
//             return _as_delim!(Starts::Array);
//         }
//         ']' => {
//             return _as_delim!(Ends::Array);
//         }
//         '(' => {
//             return _as_delim!(Starts::Group);
//         }
//         ')' => {
//             return _as_delim!(Ends::Group);
//         }
//         ',' => {
//             return _as_delim!(Separators::Entry);
//         }
//         '#' => {
//             if let Some((_, c)) = ctx.peek() {
//                 if c == &'#' {
//                     if let Some((_, c)) = ctx.peek() {
//                         if c == &'#' {
//                             if ctx.is_start_of_line {
//                                 if let Some((_, c)) = ctx.peek() {
//                                     // ####
//                                     if c == &'#' {
//                                         return _as_delim!(Lines::Title; ++);
//                                     }
//                                     // ### section
//                                     else if c.is_whitespace() {
//                                         return _as_delim!(Lines::Section; +2);
//                                     }
//                                 }
//                             }

//                             // ### reserved
//                             return _as_reserved!(+2);
//                         } else if c.is_whitespace() {
//                             // ## doc OR section
//                             if ctx.is_start_of_line {
//                                 return _as_ambiguous!(+2;
//                                     Term::Type::Delimiters::Lines::Doc,
//                                     Term::Type::Delimiters::Lines::Section
//                                 );
//                             }
//                             // ## eol doc
//                             else {
//                                 return _as_delim!(Lines::Doc; +1);
//                             }
//                         }
//                     }

//                     // ## tag literal
//                     return _as_op!(Prefixes::TagLiteral; +1);
//                 } else if c.is_whitespace() {
//                     // # title
//                     if ctx.is_start_of_line {
//                         return _as_delim!(Lines::Title);
//                     }
//                     // # trait modifier
//                     else if ctx.prev_was_ws || ctx.prev_was_delim || ctx.prev_was_op {
//                         return _as_op!(Spaceds::TraitMod);
//                     } else {
//                         return _as_reserved!();
//                     }
//                 }
//             }

//             return _as_op!(Prefixes::Tag);
//         }
//         '.' => {
//             if let Some((_, c)) = ctx.peek() {
//                 if c == &'#' {
//                     return _as_op!(Lookups::Tag; +1);
//                 } else if c == &'.' {
//                     if let Some((_, c)) = ctx.peek() {
//                         if c == &'.' {
//                             if ctx.prev_was_delim || ctx.prev_was_ws || ctx.prev_was_op {
//                                 return _as_op!(Prefixes::Spread; +2);
//                             } else {
//                                 return _as_op!(Chaineds::Range; +2);
//                             }
//                         } else {
//                             return _as_op!(Lookups::Parent; +1);
//                         }
//                     }
//                 } else if c == &'?' {
//                     return _as_op!(Lookups::Query; +1);
//                 }
//             }
//             return _as_op!(Lookups::Dot);
//         }
//         '/' => {
//             if let Some((_, c)) = ctx.peek() {
//                 // / division
//                 if c.is_whitespace() {
//                     todo!("math /");
//                 } else if c == &'/' {
//                     if let Some((_, c)) = ctx.peek() {
//                         // /// line comment
//                         if c == &'/' {
//                             return _as_delim!(Lines::Comment; +2);
//                         }
//                         // // line comment
//                         else if ctx.prev_was_ws
//                             || ctx.prev_was_delim
//                             || ctx.prev_was_op
//                             || c.is_whitespace()
//                         {
//                             return _as_delim!(Lines::Comment; +1);
//                         }
//                         // // division (Chained)
//                         else {
//                             todo!("math //");
//                         }
//                     }
//                     // // line comment
//                     else {
//                         return _as_delim!(Lines::Comment; +1);
//                     }
//                 }
//                 // /* block comment
//                 else if c == &'*' {
//                     return _as_delim!(Starts::Comment; +1);
//                 }
//                 // / lookup
//                 else {
//                     return _as_op!(Lookups::Slash);
//                 }
//             }
//             // / lookup
//             else {
//                 return _as_op!(Lookups::Slash);
//             }
//         }
//         ';' => {
//             // ;; pipe operator
//             if let Some((_, c)) = ctx.peek()
//                 && c == &';'
//             {
//                 return _as!(Term::Type::Operators::Betweens::Pipe; +1);
//             }
//             // ; expression terminator
//             else {
//                 return _as_delim!(Separators::Expression);
//             }
//         }
//         '<' => {
//             if let Some((_, c)) = ctx.peek() {
//                 if c == &'<' {
//                     if let Some((_, c)) = ctx.peek() {
//                         // <<< reserved
//                         if c == &'<' {
//                             return _as_reserved!(+2);
//                         }
//                     }
//                     // << reserved
//                     return _as_reserved!(+1);
//                 }
//                 // < generic start
//                 else {
//                     ctx.generic_depth += 1;
//                     return _as_delim!(Ends::Generic);
//                 }
//             } else {
//                 return _as_unknown!(Operators);
//             }
//         }
//         '>' => {
//             macro_rules! _as_generic_end {
//                 () => {{
//                     ctx.generic_depth -= 1;
//                     _as_delim!(Ends::Generic)
//                 }};
//             }

//             if let Some((_, c)) = ctx.peek() {
//                 // > generic end
//                 if c.is_whitespace() {
//                     return _as_generic_end!();
//                 } else if c == &'>' {
//                     if let Some((_, c)) = ctx.peek() {
//                         // >>> reserved
//                         if c == &'>' {
//                             return _as_reserved!(+2);
//                         }
//                     }

//                     // >> proc assigner
//                     return _as!(Term::Type::Operators::Betweens::ProcAssigner; +1);
//                 } else if ctx.prev_was_ws {
//                     // > generic end OR input prefix
//                     if ctx.generic_depth > 0 {
//                         ctx.generic_depth -= 1;
//                         ctx.prev_was_delim = true;
//                         ctx.prev_was_op = true;
//                         return _as_ambiguous!(
//                             Term::Type::Delimiters::Ends::Generic,
//                             Term::Type::Operators::Prefixes::Input
//                         );
//                     } else {
//                         return _as!(Term::Type::Operators::Prefixes::Input);
//                     }
//                 }
//             }

//             return _as_generic_end!();
//         }
//         ':' => {
//             if let Some((_, c)) = ctx.peek() {
//                 if c == &':' {
//                     if let Some((_, c)) = ctx.peek() {
//                         // ::: final field assigner
//                         if c == &':' {
//                             return _as_op!(Suffixes::FinalFieldAssigner; +2);
//                         }

//                         // :: const field assigner
//                         if c.is_whitespace() || c.is_delimiter() {
//                             return _as_op!(Suffixes::ConstFieldAssigner; +1);
//                         }
//                         // :: single arg literal prefix
//                         else if ctx.prev_was_delim || ctx.prev_was_ws {
//                             return _as_op!(Prefixes::ArgLiteral; +1);
//                         }
//                         // reserved call of some kind
//                         else {
//                             return _as_reserved!(+1);
//                         }
//                     }

//                     // :: const field assigner
//                     return _as_op!(Suffixes::ConstFieldAssigner; +1);
//                 }

//                 // : mutable field assigner
//                 if c.is_whitespace() || c.is_delimiter() {
//                     return _as_op!(Suffixes::MutableFieldAssigner);
//                 }
//                 // : single arg prefix
//                 else if ctx.prev_was_delim || ctx.prev_was_ws {
//                     return _as_op!(Prefixes::Arg);
//                 }
//                 // : chained call
//                 else {
//                     return _as_op!(Chaineds::Caller);
//                 }
//             }

//             // : mutable field assigner
//             return _as_op!(Suffixes::MutableFieldAssigner);
//         }
//         '|' => {
//             if let Some((_, c)) = ctx.peek() {
//                 // || double or
//                 if c == &'|' {
//                     return _as_op!(Chaineds::Or; +1);
//                 }
//                 // | single or
//                 else if c.is_whitespace() {
//                     return _as_op!(Spaceds::Or);
//                 }
//                 // | alias prefix
//                 else {
//                     return _as_op!(Prefixes::Alias);
//                 }
//             } else if ctx.prev_was_delim || ctx.prev_was_op {
//                 // | alias prefix
//                 return _as_op!(Prefixes::Alias);
//             } else if ctx.prev_was_ws {
//                 // | single or
//                 return _as_op!(Spaceds::Or);
//             } else {
//                 return _as_reserved!();
//             }
//         }
//         _ => {
//             return _as_unknown!(Operators);
//         }
//     }
// }

// trait MightBeDelimiter {
//     fn is_delimiter(&self) -> bool;
// }

// impl MightBeDelimiter for char {
//     fn is_delimiter(&self) -> bool {
//         match self {
//             '{' | '}' | '[' | ']' | '(' | ')' | ',' | ';' => true,
//             _ => false,
//         }
//     }
// }
