use crate::token::{Code, Source, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    StruX,
    ProX,
    BloX,
}

impl Language {
    fn for_source(root: Source) -> Language {
        match root {
            Source::Code(code) => match code {
                Code::Axa => Language::StruX,
                Code::Stx => Language::StruX,
                Code::Prx => Language::ProX,
                Code::Blx => Language::BloX,
                Code::Arc => Language::StruX,
                Code::Mot => Language::BloX,
                Code::Cmd => Language::ProX,
            },
            Source::Command => Language::ProX,
        }
    }
}

pub struct Cursor {
    src: Vec<char>,
    row: usize,
    col: usize,
    ins: Indents,
    tks: Tokens,
    lng: Language,
    cfg: Config,
    pos: usize,
    buf: usize,
}

pub struct Config {
    source_type: Source,
}

impl Config {
    pub fn source_type(&self) -> Source {
        self.source_type
    }
}

impl Cursor {
    pub(crate) fn new(src: &str, root: Source) -> Cursor {
        Cursor {
            pos: 0,
            row: 0,
            col: 0,
            buf: 0,
            src: src.chars().collect(),
            ins: Indents::new(),
            tks: Tokens::new(root),
            lng: Language::for_source(root),
            cfg: Config { source_type: root },
        }
    }

    pub fn context(&self) -> Language {
        self.lng.clone()
    }

    pub fn config(&self) -> &Config {
        &self.cfg
    }

    pub fn indent(&self) -> &Indents {
        &self.ins
    }

    pub fn index(&self) -> usize {
        if self.pos == 0 {
            0
        } else {
            self.pos - 1
        }
    }

    /// the char that was read before the current char(if any)
    pub fn prev(&self) -> char {
        if self.pos != 0 {
            self.char_at(self.pos - 1)
        } else {
            '\0'
        }
    }

    /// the char that will be read next
    pub fn next(&self) -> char {
        if self.pos == self.src.len() {
            '\0'
        } else {
            self.char_at(self.pos)
        }
    }

    /// the char after the next char
    pub fn next_next(&self) -> char {
        if self.pos + 1 >= self.src.len() {
            '\0'
        } else {
            self.char_at(self.pos + 1)
        }
    }

    /// used to continuously peek ahead at the next char and the ones after using a buffer
    pub fn peek(&mut self) -> char {
        let next = self.char_at(self.pos + self.buf);
        self.buf += 1;

        next
    }

    /// Get the last peeked at char (defaults to the next char if no peeking has been done by you)
    pub fn prev_peek(&mut self) -> char {
        if self.buf == 0 {
            self.next()
        } else {
            self.char_at(self.pos + self.buf - 1)
        }
    }

    // reset the peek buffer
    pub fn reset_peek(&mut self) {
        self.buf = 0;
    }

    /// get the peek buffer as a string
    pub fn peeked(&self) -> String {
        let mut buf = String::new();
        for i in 0..self.buf {
            buf.push(self.char_at(self.pos + i));
        }

        buf
    }

    /// Read the next char; increment the cursor's actual position, updating indents, and resetting the peek buffer.
    pub fn read(&mut self) {
        if !self.is_eof() {
            self._update_position();
            self.reset_peek();
        } else {
            log::warn!("Attempted to read past EOF");
        }
    }

    /// read n chars
    pub fn read_(&mut self, n: usize) {
        for _ in 0..n {
            self.read();
        }
    }

    /// read all peeked chars, leaving the next char as the next unpeeked char.
    pub fn read_peeked(&mut self) {
        self.read_(self.buf);
    }

    /// read to the last peeked char, but leave it as the next char.
    pub fn read_to_peek(&mut self) {
        self.read_(self.buf - 1);
    }

    /// get a char at a specific index
    pub fn char_at(&self, index: usize) -> char {
        self.src[index]
    }

    /// get the current position of the cursor
    pub fn is_eof(&self) -> bool {
        self.pos == self.src.len()
    }

    fn _update_position(&mut self) {
        self.col += 1;

        match self.next() {
            '\n' => {
                if self.ins.is_reading {
                    self._close_indent();
                }
                self._start_indent();
                if self.next_next() == '\r' {
                    self.pos += 1;
                }
                self.col = 0;
                self.row += 1;
            }
            '\r' => {
                if self.next_next() == '\n' {
                    self.pos += 1;
                } else {
                    if self.ins.is_reading {
                        self._close_indent();
                    }
                    self._start_indent();
                }
                self.col = 0;
                self.row += 1;
            }
            ' ' | '\t' => {
                if self.ins.is_reading {
                    self.ins.curr += 1;
                }
            }
            _ => {
                if self.ins.is_reading {
                    self._close_indent();
                }
            }
        }

        self.pos += 1;
    }

    fn _start_indent(&mut self) {
        self.ins.is_reading = true;
        self.ins.curr = 0;
    }

    fn _close_indent(&mut self) {
        self.ins.is_reading = false;
        if self.ins.increased() {
            self.ins.stack.push(self.ins.curr);
        } else {
            while self.ins.decreased() {
                self.ins.stack.pop();
            }
        }
    }
}

pub struct Tokens {
    root: Token,
    stack: Vec<Token>,
}

impl Tokens {
    pub fn new(root: Source) -> Tokens {
        Tokens {
            root: Token::new(Token::Type::Source(root), 0),
            stack: vec![],
        }
    }
}

pub struct Indents {
    is_reading: bool,
    curr: usize,
    stack: Vec<usize>,
}

impl Indents {
    pub fn new() -> Indents {
        Indents {
            is_reading: false,
            curr: 0,
            stack: vec![0],
        }
    }

    pub fn increased(&self) -> bool {
        self.curr > *self.stack.last().unwrap()
    }

    pub fn decreased(&self) -> bool {
        self.curr < *self.stack.last().unwrap()
    }

    pub fn same(&self) -> bool {
        self.curr == *self.stack.last().unwrap()
    }

    pub fn is_reading(&self) -> bool {
        self.is_reading
    }
}
