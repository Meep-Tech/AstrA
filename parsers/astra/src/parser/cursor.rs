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
        //self.src.reset_peek();
    }

    pub fn peeked_is(&self, cadence: Cadence) {}

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
        self.clone().peek()
    }

    pub(crate) fn peek(&mut self) -> Option<char> {
        if self.next.is_none() {
            self.next = match self.src.peek() {
                Some((i, c)) => Some(*c),
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

    pub fn is_eof(&self) -> bool {
        self.next().is_none()
    }

    pub(crate) fn reset(&mut self) {
        self.src.reset_peek();
        self.peek_buff.clear();
    }

    pub(crate) fn read(&mut self) -> Option<char> {
        self.prev = Some(self.curr);
        self.curr = self.src.next()?.1;

        self.index += 1;
        self.col += 1;

        self.next = None;
        self.peek_buff.clear();

        Some(self.curr)
    }

    pub(crate) fn read_peeked(&mut self) -> usize {
        self.read_peeked_minus(0)
    }

    pub(crate) fn read_peeked_minus(&mut self, minus: usize) -> usize {
        let mut count = 0;
        for _ in 0..(self.peek_buff.len() - minus) {
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
        while self.read_if(f) {
            count += 1;
        }

        count
    }

    pub(crate) fn read_until<F>(&mut self, f: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut count = 0;
        while !self.read_if(f) {
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
        F: FnOnce(char) -> bool,
        U: FnOnce(char) -> bool,
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

    pub(crate) fn start_term_at(&self, ttype: Term::Type, at: usize) -> &mut Term {
        let term = Term::Of_Type(ttype, at);
        self.push_term(term);

        self.terms.last_mut().unwrap()
    }

    pub(crate) fn start_term(&self, ttype: Term::Type) -> &mut Term {
        self.start_term_at(ttype, self.index)
    }

    pub(crate) fn started_term(&self, ttype: Term::Type) -> &mut Term {
        self.start_term_at(ttype, self.index - 1)
    }

    pub(crate) fn end_term(&self) -> &Term {
        &self.terms.last_mut().unwrap().end(self.index)
    }

    pub(crate) fn ended_term(&self) -> &Term {
        &self.terms.last_mut().unwrap().end(self.index - 1)
    }

    pub(crate) fn increase_indent(&mut self, indent: usize) {
        self.indents.push(indent);
    }

    pub(crate) fn decrease_indent(&mut self) {
        self.indents.pop();
    }

    pub(crate) fn end(&mut self) -> Vec<Term> {
        // close all open indents
        _lexer::_lex_indentation(self);

        self.terms
    }
}
