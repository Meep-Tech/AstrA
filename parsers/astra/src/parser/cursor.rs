use std::collections::HashMap;

use crate::parser::{
    context::{Context, Language},
    fs,
    indents::Indents,
};

use super::{token::Error, Status, Token};

pub struct Cursor {
    indents: Indents,
    index: usize,
    ctx: Context,

    src: Vec<char>,
    state: Vec<State>,
}

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

    pub fn save(&mut self) -> usize {
        let state = self.state();
        self.state.push(state);

        self.index
    }

    pub fn restore(&mut self) -> usize {
        let state = self.state.pop().unwrap();
        if self.index != state.pos {
            self.index = state.pos;
            self.indents = state.indents;
        }

        self.index
    }

    pub fn state(&self) -> State {
        State {
            pos: self.index,
            indents: self.indents.clone(),
        }
    }

    pub fn read(&mut self) -> char {
        self._update_indents();
        self.index += 1;
        self.curr()
    }

    pub fn skip(&mut self) {
        self._update_indents();
        self.index += 1;
    }

    fn _update_indents(&mut self) {
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

    pub fn read_chars(&mut self, n: usize) -> Vec<char> {
        let mut result = Vec::new();
        for _ in 0..n {
            result.push(self.read());
        }
        return result;
    }

    pub fn try_read(&mut self, c: char) -> bool {
        if self.curr_is(c) {
            self.read();
            return true;
        }

        return false;
    }

    // TODO: return a ws token with is_ignored = true
    pub fn skip_ws(&mut self) {
        self.skip_while(|c| c.is_whitespace());
    }

    pub fn skip_while(&mut self, f: fn(char) -> bool) {
        while f(self.curr()) {
            self.skip();
        }
    }

    pub fn skip_until(&mut self, f: fn(char) -> bool) {
        while !f(self.curr()) {
            self.skip();
        }
    }

    pub fn read_while(&mut self, f: fn(char) -> bool) -> Vec<char> {
        let mut result = Vec::new();
        while f(self.curr()) {
            result.push(self.read());
        }

        return result;
    }

    pub fn read_until(&mut self, f: fn(char) -> bool) -> Vec<char> {
        let mut result = Vec::new();
        while !f(self.curr()) {
            result.push(self.read());
        }

        return result;
    }

    /// Returns true if the current character is the first character in the input.
    /// This is the character that was read first. \0 if the current character is the first character.
    pub fn is_first(&self) -> bool {
        return self.index == 0;
    }

    /// Returns true if the current character is the first non-whitespace character in the current line.
    pub fn is_start_of_line(&self) -> bool {
        return self.is_in_indent() && !self.next_is_ws() && !self.is_eof();
    }

    /// Returns the current index (of the current character).
    pub fn index(&self) -> usize {
        return self.index;
    }

    /// Returns the prev index (before the current character).
    pub fn prev_index(&self) -> usize {
        return self.index - 1;
    }

    /// Returns the indent information.
    pub fn indents(&self) -> &Indents {
        return &self.indents;
    }

    /// Returns the previous character before the current character.
    /// This is the character that was read last. \0 if the current character is the first character.
    pub fn prev(&self) -> char {
        return self.back(1);
    }

    /// Returns the current character being examined.
    /// This is the character that will be read next.
    pub fn curr(&self) -> char {
        return self.at(self.index);
    }

    /// Returns the next character after the current character.
    /// This is the next character that will be read after the current character.
    pub fn next(&self) -> char {
        return self.ahead(1);
    }

    pub fn curr_str(&self) -> String {
        return self.at(self.index).to_string();
    }

    pub fn next_str(&self) -> String {
        return self.ahead(1).to_string();
    }

    pub fn prev_str(&self) -> String {
        return self.back(1).to_string();
    }

    pub fn prev_pos(&self) -> usize {
        return match self.index {
            0 => 0,
            _ => self.index - 1,
        };
    }

    pub fn next_is(&self, c: char) -> bool {
        return self.next() == c;
    }

    pub fn curr_is(&self, c: char) -> bool {
        return self.curr() == c;
    }

    pub fn prev_is(&self, c: char) -> bool {
        return self.prev() == c;
    }

    pub fn curr_is_ws(&self) -> bool {
        self.curr_is(' ') || self.curr_is('\t') || self.curr_is('\n')
    }

    pub fn next_is_ws(&self) -> bool {
        self.next_is(' ') || self.next_is('\t') || self.next_is('\n')
    }

    pub fn prev_is_ws(&self) -> bool {
        self.prev_is(' ') || self.prev_is('\t') || self.prev_is('\n')
    }

    pub fn curr_is_nl(&self) -> bool {
        return self.curr_is('\n');
    }

    pub fn next_is_nl(&self) -> bool {
        return self.next_is('\n');
    }

    pub fn prev_is_nl(&self) -> bool {
        return self.prev_is('\n');
    }

    pub fn next_is_digit(&self) -> bool {
        return self.next().is_digit(10);
    }

    pub fn prev_is_digit(&self) -> bool {
        return self.prev().is_digit(10);
    }

    pub fn curr_is_digit(&self) -> bool {
        return self.curr().is_digit(10);
    }

    pub fn next_is_alpha(&self) -> bool {
        return self.next().is_alphabetic();
    }

    pub fn prev_is_alpha(&self) -> bool {
        return self.prev().is_alphabetic();
    }

    pub fn curr_is_alpha(&self) -> bool {
        return self.curr().is_alphabetic();
    }

    pub fn next_is_alphanumeric(&self) -> bool {
        return self.next().is_alphanumeric();
    }

    pub fn prev_is_alphanumeric(&self) -> bool {
        return self.prev().is_alphanumeric();
    }

    pub fn curr_is_alphanumeric(&self) -> bool {
        return self.curr().is_alphanumeric();
    }

    pub fn next_is_nbsp(&self) -> bool {
        let next = self.next();
        return next == ' ' || next == '\t';
    }

    pub fn prev_is_nbsp(&self) -> bool {
        let prev = self.prev();
        return prev == ' ' || prev == '\t';
    }

    pub fn curr_is_nbsp(&self) -> bool {
        let curr = self.curr();
        return curr == ' ' || curr == '\t';
    }

    pub fn ahead(&self, offset: usize) -> char {
        return self.at(self.index + offset);
    }

    pub fn back(&self, offset: usize) -> char {
        return self.at(self.index - offset);
    }

    pub fn at(&self, pos: usize) -> char {
        return self.src[pos];
    }

    pub fn is_eof(&self) -> bool {
        return self.eof_at(self.index);
    }

    pub fn eof_at(&self, pos: usize) -> bool {
        return pos == self.src.len() - 2;
    }

    pub fn slice(&self, start: usize, end: usize) -> String {
        return self.src[start..end].iter().collect();
    }

    pub fn lang(&self) -> &Language {
        return &self.ctx.lang;
    }

    pub fn file_type(&self) -> &fs::Type {
        return match &self.ctx.file {
            Some(file) => &file.kind,
            None => &fs::Type::Unknown,
        };
    }

    /// Returns true if a non-whitespace char has not been read yet for the current line.
    /// This does NOT include the current un-read character.
    pub fn is_in_indent(&self) -> bool {
        return self.indents.is_reading;
    }

    // #region Errors

    pub fn unexpected_char_in(&mut self, token: &mut Token, expected: &[&str]) -> Status {
        self.unexpected_char_at(self.index, token, expected)
    }

    pub fn unexpected_prev_in(&mut self, token: &mut Token, expected: &[&str]) -> Status {
        self.unexpected_char_at(self.prev_index(), token, expected)
    }

    pub fn unexpected_char_of(
        &mut self,
        token: &mut Token,
        expected: HashMap<String, Option<String>>,
    ) -> Status {
        self.unexpected_char_at_index_of(token, self.index, expected)
    }

    pub fn unexpected_prev_of(
        &mut self,
        token: &mut Token,
        expected: HashMap<String, Option<String>>,
    ) -> Status {
        self.unexpected_char_at_index_of(token, self.prev_index(), expected)
    }

    pub fn unexpected_char_at(
        &mut self,
        at: usize,
        in_token: &mut Token,
        expected: &[&str],
    ) -> Status {
        in_token.errors.push(Error::Unexpected(
            &in_token.ttype,
            at,
            self.at(at),
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
            &token.ttype,
            at,
            self.at(at),
            expected
                .iter()
                .map(|e| e.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        ));

        Status::Err
    }

    // #endregion
}

pub fn char_to_string(c: char) -> String {
    match c {
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        ' ' => "\\_".to_string(),
        '\0' => "EOF".to_string(),
        c => c.to_string(),
    }
}
