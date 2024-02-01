use std::collections::HashMap;

use crate::parser::{
    context::{Context, Language},
    fs,
    indents::Indents,
};

use super::{term, Error, Status, Token};

#[derive(Debug)]
pub struct Cursor {
    indents: Indents,
    index: usize,
    ctx: Context,

    src: Vec<char>,
    state: Vec<State>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub pos: usize,
    pub indents: Indents,
}

impl Cursor {
    #[allow(non_snake_case)]
    pub fn New(source: &str) -> Cursor {
        Cursor::New_With(source, Context::New_Empty())
    }

    #[allow(non_snake_case)]
    pub fn New_For(source: &str, lang: Language) -> Cursor {
        Cursor::New_With(source, Context::New_For(lang))
    }

    #[allow(non_snake_case)]
    pub fn New_With(source: &str, ctx: Context) -> Cursor {
        let src: Vec<char> = (source.to_string() + "\0").chars().collect();
        Cursor {
            index: 0,
            src,
            ctx,
            indents: Indents {
                stack: Vec::new(),
                is_reading: true,
                curr: 0,
            },
            state: Vec::new(),
        }
    }

    pub fn err(&self) -> Errer {
        Errer { cursor: self }
    }

    pub fn token(&self) -> Tokenizer {
        Tokenizer { cursor: self }
    }
}

pub(crate) trait InternalCursorData {
    fn src(&self) -> &Vec<char>;
    fn ctx(&self) -> &Context;
    fn update_indents(&mut self);
}

pub struct Tokenizer<'temp> {
    cursor: &'temp Cursor,
}

impl<'temp> Tokenizer<'temp> {
    pub fn start(&self) -> Token {
        self.start_at(self.cursor.index())
    }

    pub fn start_at(&self, start: usize) -> Token {
        Token::New(start)
    }

    pub fn start_from_prev(&self) -> Token {
        self.start_at(self.cursor.prev_index())
    }

    pub fn of_type(&self, token_type: Token::Type) -> Token {
        self.at(self.cursor.index(), token_type)
    }

    pub fn at(&self, start: usize, of_type: Token::Type) -> Token {
        Token::Of_Type(of_type, self.cursor.index())
    }

    pub fn at_prev(&self, of_type: Token::Type) -> Token {
        self.at(self.cursor.prev_index(), of_type)
    }

    pub fn end(&self, token: Token) -> Token {
        self.end_at(self.cursor.index(), token)
    }

    pub fn end_at(&self, end: usize, token: Token) -> Token {
        token.end_at(end)
    }

    pub fn end_at_prev(&self, token: Token) -> Token {
        self.end_at(self.cursor.prev_index(), token)
    }
}

impl InternalCursorData for Cursor {
    fn src(&self) -> &Vec<char> {
        &self.src
    }

    fn ctx(&self) -> &Context {
        &self.ctx
    }

    fn update_indents(&mut self) {
        match self.curr() {
            '\n' => {
                if self.indents.prev() != self.indents.curr {
                    self.indents.stack.push(self.indents.curr.to_owned());
                }

                self.indents.curr = 0;
                self.indents.is_reading = true;
            }
            '\t' | ' ' => {
                if self.is_in_indent() {
                    self.indents.curr += 1;
                }
            }
            _ => {
                if self.is_in_indent() {
                    self.indents.is_reading = false;
                }
            }
        }
    }
}

pub trait Scanner: InternalCursorData {
    fn indents(&self) -> &Indents;
    fn index(&self) -> usize;
    fn state(&self) -> State;

    /// Returns true if the current character is the first character in the input.
    /// This is the character that was read first. \0 if the current character is the first character.
    fn is_first(&self) -> bool {
        return self.index() == 0;
    }

    /// Returns true if the current character is the first non-whitespace character in the current line.
    fn is_start_of_line(&self) -> bool {
        return self.is_in_indent() && !self.next_is_ws() && !self.is_eof();
    }

    /// Returns the prev index (before the current character).
    fn prev_index(&self) -> usize {
        return self.index() - 1;
    }

    /// Returns the previous character before the current character.
    /// This is the character that was read last. \0 if the current character is the first character.
    fn prev(&self) -> char {
        return self.back(1);
    }

    /// Returns the current character being examined.
    /// This is the character that will be read next.
    fn curr(&self) -> char {
        return self.at(self.index());
    }

    /// Returns the next character after the current character.
    /// This is the next character that will be read after the current character.
    fn next(&self) -> char {
        return self.ahead(1);
    }

    fn curr_str(&self) -> String {
        return self.at(self.index()).to_string();
    }

    fn next_str(&self) -> String {
        return self.ahead(1).to_string();
    }

    fn prev_str(&self) -> String {
        return self.back(1).to_string();
    }

    fn prev_pos(&self) -> usize {
        return match self.index() {
            0 => 0,
            _ => self.index() - 1,
        };
    }

    fn next_is(&self, c: char) -> bool {
        return self.next() == c;
    }

    fn curr_is(&self, c: char) -> bool {
        return self.curr() == c;
    }

    fn prev_is(&self, c: char) -> bool {
        return self.prev() == c;
    }

    fn curr_is_ws(&self) -> bool {
        self.curr_is(' ') || self.curr_is('\t') || self.curr_is('\n')
    }

    fn next_is_ws(&self) -> bool {
        self.next_is(' ') || self.next_is('\t') || self.next_is('\n')
    }

    fn prev_is_ws(&self) -> bool {
        self.prev_is(' ') || self.prev_is('\t') || self.prev_is('\n')
    }

    fn curr_is_nl(&self) -> bool {
        return self.curr_is('\n');
    }

    fn next_is_nl(&self) -> bool {
        return self.next_is('\n');
    }

    fn prev_is_nl(&self) -> bool {
        return self.prev_is('\n');
    }

    fn next_is_digit(&self) -> bool {
        return self.next().is_digit(10);
    }

    fn prev_is_digit(&self) -> bool {
        return self.prev().is_digit(10);
    }

    fn curr_is_digit(&self) -> bool {
        return self.curr().is_digit(10);
    }

    fn next_is_alpha(&self) -> bool {
        return self.next().is_alphabetic();
    }

    fn prev_is_alpha(&self) -> bool {
        return self.prev().is_alphabetic();
    }

    fn curr_is_alpha(&self) -> bool {
        return self.curr().is_alphabetic();
    }

    fn next_is_alphanumeric(&self) -> bool {
        return self.next().is_alphanumeric();
    }

    fn prev_is_alphanumeric(&self) -> bool {
        return self.prev().is_alphanumeric();
    }

    fn curr_is_alphanumeric(&self) -> bool {
        return self.curr().is_alphanumeric();
    }

    fn next_is_nbsp(&self) -> bool {
        let next = self.next();
        return next == ' ' || next == '\t';
    }

    fn prev_is_nbsp(&self) -> bool {
        let prev = self.prev();
        return prev == ' ' || prev == '\t';
    }

    fn curr_is_nbsp(&self) -> bool {
        let curr = self.curr();
        return curr == ' ' || curr == '\t';
    }

    fn ahead(&self, offset: usize) -> char {
        return self.at(self.index() + offset);
    }

    fn back(&self, offset: usize) -> char {
        return self.at(self.index() - offset);
    }

    fn at(&self, pos: usize) -> char {
        return self.src()[pos];
    }

    fn is_eof(&self) -> bool {
        return self.eof_at(self.index());
    }

    fn eof_at(&self, pos: usize) -> bool {
        return pos == self.src().len() - 2;
    }

    fn slice(&self, start: usize, end: usize) -> String {
        return self.src()[start..end].iter().collect();
    }

    fn lang(&self) -> &Language {
        return &self.ctx().lang;
    }

    fn file_type(&self) -> &fs::Type {
        return match &self.ctx().file {
            Some(file) => &file.kind,
            None => &fs::Type::Unknown,
        };
    }

    /// Returns true if a non-whitespace char has not been read yet for the current line.
    /// This does NOT include the current un-read character.
    fn is_in_indent(&self) -> bool {
        return self.indents().is_reading;
    }
}

impl Scanner for Cursor {
    /// Returns the current index (of the current character).
    fn index(&self) -> usize {
        return self.index;
    }

    /// Returns the indent information.
    fn indents(&self) -> &Indents {
        return &self.indents;
    }

    /// Returns a clone of the current state of the cursor.
    fn state(&self) -> State {
        State {
            pos: self.index(),
            indents: self.indents().clone(),
        }
    }
}
pub trait Reader: Scanner + InternalCursorData {
    /// Reads the current character and returns it.
    fn read(&mut self) -> char;

    /// Skips the current character.
    fn skip(&mut self);

    /// Saves the current state of the cursor and returns the index of the current character.
    fn save(&mut self) -> usize;

    /// Restores the cursor to the previous state and returns the index of the current character.
    fn restore(&mut self) -> usize;

    /// Reads the next n characters and returns them as a vector.
    fn read_chars(&mut self, n: usize) -> Vec<char> {
        let mut result = Vec::new();
        for _ in 0..n {
            result.push(self.read());
        }
        return result;
    }

    /// Reads the next character as specified by the predicate, or returns False.
    fn try_read(&mut self, c: char) -> bool {
        if self.curr_is(c) {
            self.read();
            return true;
        }

        return false;
    }

    // Reads while the predicate is true.
    fn read_while(&mut self, f: fn(char) -> bool) -> Vec<char> {
        let mut result = Vec::new();
        while f(self.curr()) {
            result.push(self.read());
        }

        return result;
    }

    // Reads until the predicate is true.
    fn read_until(&mut self, f: fn(char) -> bool) -> Vec<char> {
        let mut result = Vec::new();
        while !f(self.curr()) {
            result.push(self.read());
        }

        return result;
    }

    // Skips while the predicate is true.
    fn skip_while(&mut self, f: fn(char) -> bool) {
        while f(self.curr()) {
            self.skip();
        }
    }

    // Skips until the predicate is true.
    fn skip_until(&mut self, f: fn(char) -> bool) {
        while !f(self.curr()) {
            self.skip();
        }
    }
    /// Skips over all whitespace characters up to the next non-whitespace character.
    fn skip_ws(&mut self) {
        self.skip_while(|c| c.is_whitespace());
    }
}

impl Reader for Cursor {
    fn read(&mut self) -> char {
        self.update_indents();
        self.index += 1;
        self.curr()
    }

    fn skip(&mut self) {
        self.update_indents();
        self.index += 1;
    }

    fn save(&mut self) -> usize {
        let state = self.state();
        self.state.push(state);

        self.index
    }

    fn restore(&mut self) -> usize {
        let state = self.state.pop().unwrap();
        if self.index() != state.pos {
            self.index = state.pos;
            self.indents = state.indents;
        }

        self.index()
    }
}

pub struct Errer<'temp> {
    cursor: &'temp Cursor,
}

impl<'temp> Errer<'temp> {
    pub fn unexpected_char_in(&mut self, token: &mut Token, expected: &[&str]) -> Status {
        self.unexpected_char_at(self.cursor.index(), token, expected)
    }

    pub fn unexpected_prev_in(&mut self, token: &mut Token, expected: &[&str]) -> Status {
        self.unexpected_char_at(self.cursor.prev_index(), token, expected)
    }

    pub fn unexpected_char_of(
        &mut self,
        token: &mut Token,
        expected: HashMap<String, Option<String>>,
    ) -> Status {
        self.unexpected_char_at_index_of(token, self.cursor.index(), expected)
    }

    pub fn unexpected_prev_of(
        &mut self,
        token: &mut Token,
        expected: HashMap<String, Option<String>>,
    ) -> Status {
        self.unexpected_char_at_index_of(token, self.cursor.prev_index(), expected)
    }

    pub fn unexpected_char_at(
        &mut self,
        at: usize,
        in_token: &mut Token,
        expected: &[&str],
    ) -> Status {
        in_token.errors.push(Error::Unexpected(
            &Error::Type::Token(in_token.ttype.clone()),
            at,
            self.cursor.at(at),
            expected,
        ));

        Status::Err
    }

    pub fn unexpected_char_at_index_of(
        &mut self,
        token: &mut Token,
        at: usize,
        expected: HashMap<String, Option<String>>,
    ) -> Status {
        let expected = expected
            .iter()
            .map(|(k, v)| {
                if let Some(v) = v {
                    format!("{}: ({})", k, v)
                } else {
                    k.to_string()
                }
            })
            .collect::<Vec<String>>();

        token.errors.push(Error::Unexpected(
            &Error::Type::Token(token.ttype.clone()),
            at,
            self.cursor.at(at),
            expected
                .iter()
                .map(|e| e.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        ));

        Status::Err
    }

    pub fn invalid_char_in(&mut self, token: &mut Token, reason: &str) -> Status {
        self.invalid_char_at(self.cursor.index(), token, reason)
    }

    pub fn invalid_prev_in(&mut self, token: &mut Token, reason: &str) -> Status {
        self.invalid_char_at(self.cursor.prev_index(), token, reason)
    }

    pub fn invalid_char_at(&mut self, at: usize, in_token: &mut Token, reason: &str) -> Status {
        in_token.errors.push(Error::Invalid(
            &Error::Type::Token(in_token.ttype.clone()),
            at,
            self.cursor.at(at),
            reason.to_string(),
        ));

        Status::Err
    }
}

pub fn char_to_string(c: char) -> String {
    match c {
        '\0' => "\\0".to_string(),
        '\r' => "\\r".to_string(),
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        ' ' => "' '".to_string(),
        c => c.to_string(),
    }
}
