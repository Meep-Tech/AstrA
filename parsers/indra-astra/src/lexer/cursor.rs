use crate::{lexer::indents::Indents, utils::log};

use super::context::{Context, Language};

pub struct Cursor {
    pub src: Vec<char>,
    pub indents: Indents,
    pub pos: usize,
    ctx: Context,
    state: Vec<State>,
}

pub struct State {
    pub pos: usize,
    pub indents: Indents,
}

impl Cursor {
    pub fn new(source: &str) -> Cursor {
        Cursor::new_with(source, Context::new_empty())
    }

    pub fn new_for(source: &str, lang: Language) -> Cursor {
        Cursor::new_with(source, Context::new_for(lang))
    }

    pub fn new_with(source: &str, ctx: Context) -> Cursor {
        log::add_color("CURSOR", log::Color::BrightGreen);
        log::add_color("TOKEN", log::Color::BrightBlue);
        log::add_color("INDENT", log::Color::BrightWhite);
        log::add_bg("INDENT", log::Color::BrightBlack);
        log::push_unique_key("PARSE");

        log::info!(
            &["CURSOR", ":NEW"],
            &format!("Creating new cursor for input of length {}", source.len()),
        );
        log::info!(&["CURSOR", "INDENT", ":START"], " curr: 0");
        let src: Vec<char> = (source.to_string() + "\0").chars().collect();
        Cursor {
            pos: 0,
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
        if (self.state.last().is_none()) || (self.state.last().unwrap().pos == self.pos) {
            log::info!(&["CURSOR", "SAVE"], &format!("@ {}", self.pos));
        }

        let state = self.state();
        self.state.push(state);

        self.pos
    }

    pub fn restore(&mut self) -> usize {
        let state = self.state.pop().unwrap();
        if self.pos != state.pos {
            log::info!(
                &["CURSOR", "RESTORE"],
                &format!("{} ~> {}", self.pos, state.pos),
            );

            self.pos = state.pos;
            self.indents = state.indents;
        }

        self.pos
    }

    pub fn state(&self) -> State {
        State {
            pos: self.pos,
            indents: self.indents.clone(),
        }
    }

    pub fn read(&mut self) -> char {
        log::info!(
            &["CURSOR", "READ"],
            &format!(
                "{}({}) => {}({}).",
                Cursor::char_to_string(self.curr()),
                self.pos,
                Cursor::char_to_string(self.next()),
                self.pos + 1
            ),
        );

        self._update_indents();
        self.pos += 1;

        if self.is_eof() {
            log::info!(&["CURSOR", ":EOF"], "Reached end of file.");
        }

        self.curr()
    }

    pub fn skip(&mut self) {
        log::info!(
            &["CURSOR", "SKIP"],
            &format!(
                "{}({}) => {}({}).",
                Cursor::char_to_string(self.curr()),
                self.pos,
                Cursor::char_to_string(self.next()),
                self.pos + 1
            ),
        );

        self._update_indents();
        self.pos += 1;

        if self.is_eof() {
            log::info!(&["CURSOR", ":EOF"], "Reached end of file.");
        }
    }

    fn _update_indents(&mut self) {
        match self.curr() {
            '\n' => {
                if self.indents.prev() != self.indents.curr {
                    log::info!(
                        &["CURSOR", "INDENT", "PUSH"],
                        &format!(" prev: {} => {}", self.indents.prev(), self.indents.curr)
                    );
                    self.indents.stack.push(self.indents.curr.to_owned());
                }

                log::info!(&["CURSOR", "INDENT", ":START"], " curr: 0");
                self.indents.curr = 0;
                self.indents.is_reading = true;
            }
            '\t' | ' ' => {
                if self.indents.is_reading {
                    self.indents.curr += 1;
                    log::info!(
                        &["CURSOR", "INDENT", "APPEND"],
                        &format!(
                            " curr: {} => {} VS prev: {} ({})",
                            self.indents.curr - 1,
                            self.indents.curr,
                            self.indents.prev(),
                            if self.indents.curr > self.indents.prev() {
                                "increase"
                            } else if self.indents.curr < self.indents.prev() {
                                "decrease"
                            } else {
                                "same"
                            }
                        )
                    );
                }
            }
            _ => {
                if self.indents.is_reading {
                    self.indents.is_reading = false;
                    log::info!(
                        &["CURSOR", "INDENT", ":END"],
                        &format!(
                            " curr: {} vs prev: {} ({})",
                            self.indents.curr,
                            self.indents.prev(),
                            if self.indents.curr > self.indents.prev() {
                                "increase"
                            } else if self.indents.curr < self.indents.prev() {
                                "decrease"
                            } else {
                                "same"
                            }
                        )
                    );
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
        log::info!(&["CURSOR", "SKIP-WS"], &format!("{}..", self.pos));
        self.skip_while(|c| c.is_whitespace());
        log::info!(&["CURSOR", "SKIP-WS"], &format!("..{}", self.pos));
    }

    pub fn skip_while(&mut self, f: fn(char) -> bool) {
        log::info!(&["CURSOR", "SKIP-WHILE"], &format!("{}..", self.pos));
        while f(self.curr()) {
            self.skip();
        }
        log::info!(&["CURSOR", "SKIP-WHILE"], &format!("..{}", self.pos));
    }

    pub fn skip_until(&mut self, f: fn(char) -> bool) {
        log::info!(&["CURSOR", "SKIP-UNTIL"], &format!("{}..", self.pos));
        while !f(self.curr()) {
            self.skip();
        }
        log::info!(&["CURSOR", "SKIP-UNTIL"], &format!("..{}", self.pos));
    }

    pub fn read_while(&mut self, f: fn(char) -> bool) -> Vec<char> {
        log::info!(&["CURSOR", "READ-WHILE"], &format!("{}..", self.pos));
        let mut result = Vec::new();
        while f(self.curr()) {
            result.push(self.read());
        }
        log::info!(&["CURSOR", "READ-WHILE"], &format!("..{}", self.pos));
        return result;
    }

    pub fn read_until(&mut self, f: fn(char) -> bool) -> Vec<char> {
        log::info!(&["CURSOR", "READ-UNTIL"], &format!("{}..", self.pos));
        let mut result = Vec::new();
        while !f(self.curr()) {
            result.push(self.read());
        }
        log::info!(&["CURSOR", "READ-UNTIL"], &format!("..{}", self.pos));
        return result;
    }

    /// Returns true if the current character is the first character in the input.
    /// This is the character that was read first. \0 if the current character is the first character.
    pub fn is_first(&self) -> bool {
        return self.pos == 0;
    }

    /// Returns the previous character before the current character.
    /// This is the character that was read last. \0 if the current character is the first character.
    pub fn prev(&self) -> char {
        return self.back(1);
    }

    /// Returns the current character being examined.
    /// This is the character that will be read next.
    pub fn curr(&self) -> char {
        return self.at(self.pos);
    }

    /// Returns the next character after the current character.
    /// This is the next character that will be read after the current character.
    pub fn next(&self) -> char {
        return self.ahead(1);
    }

    pub fn curr_str(&self) -> String {
        return self.at(self.pos).to_string();
    }

    pub fn prev_pos(&self) -> usize {
        return match self.pos {
            0 => 0,
            _ => self.pos - 1,
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

    pub fn ahead(&self, offset: usize) -> char {
        return self.at(self.pos + offset);
    }

    pub fn back(&self, offset: usize) -> char {
        return self.at(self.pos - offset);
    }

    pub fn at(&self, pos: usize) -> char {
        return self.src[pos];
    }

    pub fn is_eof(&self) -> bool {
        return self.eof_at(self.pos);
    }

    pub fn eof_at(&self, pos: usize) -> bool {
        return pos == self.src.len() - 2;
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

    pub fn slice(&self, start: usize, end: usize) -> String {
        return self.src[start..end].iter().collect();
    }

    pub fn lang(&self) -> &Language {
        return &self.ctx.lang;
    }
}