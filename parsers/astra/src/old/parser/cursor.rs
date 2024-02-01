use super::{Source, _lexer, symbol::Cadence, Term};

pub struct Cursor<'a> {
    src: Source<'a>,
    index: usize,
    line: usize,
    col: usize,
    curr: char,
    next: Option<char>,
    prev: Option<char>,
    terms: Vec<Term>,
    indents: Vec<usize>,
    indent_increased: bool,
    indent_decreased: bool,
    is_start_of_line: bool,
    prev_was_break: bool,
    peek_buff: Vec<char>,
    potential_generic_depth: usize,
}

impl<'a> Cursor<'a> {
    #[allow(non_snake_case)]
    pub(crate) fn New(mut src: Source<'a>) -> Self {
        let first = src.next().unwrap();
        Cursor {
            src,
            index: 0,
            line: 0,
            col: 0,
            curr: first.1,
            next: None,
            prev: None,
            terms: vec![],
            indent_decreased: false,
            indent_increased: false,
            indents: vec![],
            is_start_of_line: true,
            peek_buff: vec![],
            prev_was_break: true,
            potential_generic_depth: 0,
        }
    }

    pub(crate) fn start_line(&mut self) {
        self.line += 1;
        self.col = 0;
        self.is_start_of_line = true;
        self.src.reset_peek();
        //self.src.reset_peek();
    }

    pub fn peeked_has(&mut self, cadence: Cadence) -> bool {
        match cadence {
            Cadence::Alone => {
                !self.prev().unwrap().is_alphanumeric()
                    && !self.sneak_peek().unwrap().is_alphanumeric()
            }
            Cadence::Intro => self.is_start_of_line && self.sneak_peek().unwrap().is_whitespace(),
            Cadence::Prefix => {
                let prev = self.prev().unwrap();
                let next = self.sneak_peek().unwrap();

                (prev.is_whitespace() || prev.is_opening_delimiter())
                    && !next.is_whitespace()
                    && !next.is_closing_delimiter()
            }
            Cadence::Suffix => {
                let prev = self.prev().unwrap();
                let next = self.sneak_peek().unwrap();

                !prev.is_whitespace()
                    && !prev.is_opening_delimiter()
                    && (next.is_whitespace() || next.is_closing_delimiter())
            }
            Cadence::Infix => {
                let prev = self.prev().unwrap();
                let next = self.sneak_peek().unwrap();
                println!("prev: {:?}, next: {:?}", prev, next);

                !prev.is_whitespace()
                    && !next.is_whitespace()
                    && !next.is_delimiter()
                    && !prev.is_delimiter()
            }
            Cadence::Spaced => {
                let prev = self.prev().unwrap();
                let next = self.sneak_peek().unwrap();

                prev.is_whitespace() && next.is_whitespace()
            }
            Cadence::Sub => {
                if !self.is_start_of_line || !self.indent_increased {
                    return false;
                }

                let next = self.sneak_peek().unwrap();
                !next.is_whitespace() && !next.is_closing_delimiter()
            }
            Cadence::Between => true,
            Cadence::Open => {
                let prev = self.prev().unwrap();

                prev.is_whitespace() || prev.is_opening_delimiter()
            }
            Cadence::Capture => {
                let prev = self.prev().unwrap();

                !prev.is_whitespace() && !prev.is_opening_delimiter()
            }
            Cadence::Close => true,
        }
    }

    pub(crate) fn peeked(&self) -> String {
        self.peek_buff.iter().collect()
    }

    pub fn curr(&self) -> char {
        self.curr
    }

    pub fn indents(&self) -> &Vec<usize> {
        &self.indents
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn prev(&self) -> Option<char> {
        self.prev
    }

    pub fn next(&mut self) -> Option<char> {
        if let None = self.next {
            self.next = match self.src.peek() {
                Some((_, c)) => Some(*c),
                None => None,
            };
            self.src.reset_peek();
        }

        self.next
    }

    pub(crate) fn sneak_peek(&self) -> Option<char> {
        let mut clone = self.src.clone();
        clone.peek().map(|(_, c)| *c)
    }

    // TODO: use the old cursor method with the new cadences and symbols lexer datas.
    pub(crate) fn peek(&mut self) -> Option<char> {
        if self.next.is_none() {
            self.next = match self.src.peek() {
                Some((_, c)) => Some(*c),
                None => None,
            };

            self.peek_buff.push(self.next.unwrap());
            self.next
        } else {
            let next_peek = match self.src.peek() {
                Some((_, c)) => Some(*c),
                None => None,
            };

            if next_peek.is_some() {
                self.peek_buff.push(next_peek.unwrap());
            }

            next_peek
        }
    }

    pub(crate) fn prev_peek(&self) -> Option<char> {
        if self.peek_buff.len() > 0 {
            Some(self.peek_buff[self.peek_buff.len() - 1])
        } else {
            None
        }
    }

    pub fn is_eof(&mut self) -> bool {
        self.next().is_none()
    }

    pub(crate) fn reset(&mut self) {
        self.src.reset_peek();
        self.peek_buff.clear();
    }

    pub(crate) fn read(&mut self) -> Option<char> {
        self.prev = Some(self.curr);
        self.curr = self.src.next()?.1;
        println!(
            "read: prev: {:?} => curr: {:?} | Term: {:?}",
            self.prev,
            self.curr,
            if self.curr.is_whitespace() {
                None
            } else {
                match self.curr_term() {
                    Some(term) => Some(term.ttype.clone()),
                    None => None,
                }
            }
        );

        self.index += 1;
        self.col += 1;

        self.next = None;
        self.peek_buff.clear();

        Some(self.curr)
    }

    pub(crate) fn curr_term(&self) -> Option<&Term> {
        self.terms.last()
    }

    pub(crate) fn read_peeked(&mut self) -> usize {
        self.read_peeked_minus(0)
    }

    pub(crate) fn read_(&mut self, count: usize) -> String {
        let mut out = String::new();
        for _ in 0..count {
            out.push(self.read().unwrap());
        }

        out
    }

    pub(crate) fn read_peeked_minus(&mut self, minus: usize) -> usize {
        let mut count = 0;
        for _ in 0..(self.peek_buff.len() + 1 - minus) {
            count += 1;
            self.read();
        }

        count
    }

    pub(crate) fn read_to_prev_peek(&mut self) -> usize {
        self.read_peeked_minus(1)
    }

    pub(crate) fn read_if<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        if let Some(c) = self.next() {
            if f(c) {
                self.read();
                return true;
            }
        }

        false
    }

    pub(crate) fn read_if_is(&mut self, c: char) -> bool {
        self.read_if(|next| next == c)
    }

    pub(crate) fn read_if_in(&mut self, chars: &[char]) -> bool {
        self.read_if(|next| chars.contains(&next))
    }

    pub(crate) fn read_while<F>(&mut self, f: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut count = 0;
        while self.read_if(&f) {
            count += 1;
        }

        count
    }

    pub(crate) fn read_until<F>(&mut self, f: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut count = 0;
        while !self.read_if(&f) {
            count += 1;
        }

        count
    }

    pub(crate) fn read_until_is(&mut self, c: char) -> usize {
        self.read_until(|next| next == c)
    }

    pub(crate) fn read_spacing(&mut self) -> usize {
        let count = self.read_while(|next| next.is_whitespace());
        if count > 0 {
            self.prev_was_break = true;
        }

        count
    }

    pub(crate) fn find_before<F, U>(&mut self, before: U, find: F) -> bool
    where
        F: FnOnce(char) -> bool + Copy,
        U: FnOnce(char) -> bool + Copy,
    {
        let mut found = false;
        while let Some(c) = self.peek() {
            if before(c) {
                break;
            } else if find(c) {
                found = true;
                break;
            }
        }

        self.src.reset_peek();
        found
    }

    pub(crate) fn push_term(&mut self, term: Term) {
        if !term.is_ws() {
            self.prev_was_break = false;
            self.is_start_of_line = false;
        }

        self.terms.push(term);
    }

    pub(crate) fn push_as_term(&mut self, ttype: Term::Type) {
        self.push_as_term_at(ttype, self.index)
    }

    pub(crate) fn push_as_term_at(&mut self, ttype: Term::Type, at: usize) {
        self.push_term(Term::Of_Type(ttype, at));
    }

    pub(crate) fn push_prev_as_term(&mut self, ttype: Term::Type) {
        self.push_as_term_at(ttype, self.index - 1)
    }

    pub(crate) fn start_term_at(&mut self, ttype: Term::Type, at: usize) -> &mut Term {
        let term = Term::Of_Type(ttype, at);
        self.push_term(term);

        self.terms.last_mut().unwrap()
    }

    pub(crate) fn start_term(&mut self, ttype: Term::Type) -> &mut Term {
        println!("Start of Term: {:?} @ {}", ttype, self.index);
        self.start_term_at(ttype, self.index)
    }

    pub(crate) fn end_term_at(&mut self, at: usize) -> &Term {
        let mut term = self.terms.pop().unwrap();
        term = term.end(at);
        self.terms.push(term);

        self.terms.last().unwrap()
    }

    pub(crate) fn end_term(&mut self) -> &Term {
        println!(
            "End of Term: {:?} @ {}",
            self.curr_term().unwrap().ttype,
            self.index - 1
        );
        self.end_term_at(self.index - 1)
    }

    pub(crate) fn increase_indent(&mut self, indent: usize) {
        self.indents.push(indent);
        self.indent_increased = true;
        self.indent_decreased = false;
    }

    pub(crate) fn decrease_indent(&mut self) {
        self.indents.pop();
        self.indent_decreased = true;
        self.indent_increased = false;
    }

    pub(crate) fn maintain_indent(&mut self) {
        self.indent_decreased = false;
        self.indent_increased = false;
    }

    pub(crate) fn end(&mut self) -> &Vec<Term> {
        // close all open indents
        _lexer::_lex_indentation(self);

        &self.terms
    }
}

pub trait PotentialDelimiterChar {
    fn is_delimiter(&self) -> bool;
    fn is_closing_delimiter(&self) -> bool;
    fn is_opening_delimiter(&self) -> bool;
    fn is_string_delimiter(&self) -> bool;
    fn is_separator(&self) -> bool;
}

impl PotentialDelimiterChar for char {
    fn is_delimiter(&self) -> bool {
        self.is_closing_delimiter() || self.is_opening_delimiter() || self.is_string_delimiter()
    }

    fn is_closing_delimiter(&self) -> bool {
        matches!(self, ')' | ']' | '}' | '>')
    }

    fn is_opening_delimiter(&self) -> bool {
        matches!(self, '(' | '[' | '{' | '<')
    }

    fn is_string_delimiter(&self) -> bool {
        matches!(self, '"' | '\'' | '`')
    }

    fn is_separator(&self) -> bool {
        matches!(self, ';' | ',')
    }
}
