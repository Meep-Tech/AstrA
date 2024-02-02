use crate::{
    parser::{
        context::{Context, Language},
        fs,
        indents::Indents,
    },
    utils::log,
};

#[cfg(feature = "log")]
use crate::utils::ansi::Color;

pub struct Cursor {
    src: Vec<char>,
    indents: Indents,
    pos: usize,
    ctx: Context,
    state: Vec<State>,
}

pub struct State {
    pub pos: usize,
    pub indents: Indents,
}

impl Cursor {
    #[allow(non_snake_case)]
    pub fn New(source: &str) -> Cursor {
        Cursor::New_With(source, Context::new_empty())
    }

    #[allow(non_snake_case)]
    pub fn New_For(source: &str, lang: Language) -> Cursor {
        Cursor::New_With(source, Context::new_for(lang))
    }

    #[allow(non_snake_case)]
    pub fn New_With(source: &str, ctx: Context) -> Cursor {
        log::color!("CURSOR", Color::BrightGreen);
        log::color!("TOKEN", Color::BrightBlue);
        log::color!("INDENT", Color::BrightWhite);
        log::bg!("INDENT", Color::BrightBlack);
        log::push_unique!("PARSE");

        log::vvv!(
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

    pub fn curr_indent(&self) -> usize {
        return self.indents.curr;
    }

    pub fn indent(&self) -> &Indents {
        return &self.indents;
    }

    pub fn context(&self) -> &Context {
        return &self.ctx;
    }

    pub fn save(&mut self) -> usize {
        if (self.state.last().is_none()) || (self.state.last().unwrap().pos == self.pos) {
            log::vv!(&["CURSOR", "SAVE"], &format!("@ {}", self.pos));
        }

        let state = self.state();
        self.state.push(state);

        self.pos
    }

    pub fn restore(&mut self) -> usize {
        let state = self.state.pop().unwrap();
        if self.pos != state.pos {
            log::vv!(
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
        #[cfg(feature = "v")]
        let next = if self.pos >= self.src.len() - 2 {
            '\0'
        } else {
            self.next()
        };
        log::info!(
            &["CURSOR", "READ"],
            &format!(
                "{}({}) => {}({}).",
                Cursor::char_to_string(self.curr()),
                self.pos,
                Cursor::char_to_string(next),
                self.pos + 1
            ),
        );

        self._update_indents();
        self.pos += 1;

        if self.is_eof() {
            log::info!(&["CURSOR", ":EOF"], "Reached end of file.");
            return '\0';
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
                    log::vv!(
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
        log::vv!(&["CURSOR", "SKIP-WS"], &format!("{}..", self.pos));
        self.skip_while(|c| c.is_whitespace());
        log::vv!(&["CURSOR", "SKIP-WS"], &format!("..{}", self.pos));
    }

    pub fn skip_while(&mut self, f: fn(char) -> bool) {
        log::vvv!(&["CURSOR", "SKIP-WHILE"], &format!("{}..", self.pos));
        while f(self.curr()) {
            self.skip();
        }
        log::vvv!(&["CURSOR", "SKIP-WHILE"], &format!("..{}", self.pos));
    }

    pub fn skip_until(&mut self, f: fn(char) -> bool) {
        log::vvv!(&["CURSOR", "SKIP-UNTIL"], &format!("{}..", self.pos));
        while !f(self.curr()) {
            self.skip();
        }
        log::vvv!(&["CURSOR", "SKIP-UNTIL"], &format!("..{}", self.pos));
    }

    pub fn read_while(&mut self, f: fn(char) -> bool) -> Vec<char> {
        log::vvv!(&["CURSOR", "READ-WHILE"], &format!("{}..", self.pos));
        let mut result = Vec::new();
        while f(self.curr()) {
            result.push(self.read());
        }
        log::vvv!(&["CURSOR", "READ-WHILE"], &format!("..{}", self.pos));
        return result;
    }

    pub fn read_until(&mut self, f: fn(char) -> bool) -> Vec<char> {
        log::vvv!(&["CURSOR", "READ-UNTIL"], &format!("{}..", self.pos));
        let mut result = Vec::new();
        while !f(self.curr()) {
            result.push(self.read());
        }
        log::vvv!(&["CURSOR", "READ-UNTIL"], &format!("..{}", self.pos));
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

    pub fn next_str(&self) -> String {
        return self.ahead(1).to_string();
    }

    pub fn prev_str(&self) -> String {
        return self.back(1).to_string();
    }

    pub fn next_pos(&self) -> usize {
        return self.pos + 1;
    }

    pub fn curr_pos(&self) -> usize {
        return self.pos;
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

    pub fn curr_is_ws(&self) -> bool {
        self.curr_is(' ') || self.curr_is('\t') || self.curr_is('\n')
    }

    pub fn next_is_ws(&self) -> bool {
        self.next_is(' ') || self.next_is('\t') || self.next_is('\n')
    }

    pub fn prev_is_ws(&self) -> bool {
        self.prev_is(' ') || self.prev_is('\t') || self.prev_is('\n')
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
        return pos >= self.src.len() - 1;
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

    pub fn file_type(&self) -> &fs::Type {
        return match &self.ctx.file {
            Some(file) => &file.kind,
            None => &fs::Type::Unknown,
        };
    }
}
