use crate::lexer::indents::{Indents, LineIndent};

pub struct Cursor {
    pub src: Vec<char>,
    pub indents: Indents,
    pub pos: usize,
    state: Vec<State>,
}

pub struct State {
    pub pos: usize,
    pub indents: Indents,
}

impl Cursor {
    pub fn new(source: &str) -> Cursor {
        println!(
            "== Creating new cursor for input of length: {}",
            source.len()
        );
        let src: Vec<char> = (source.to_string() + "\0").chars().collect();
        Cursor {
            pos: 0,
            src,
            indents: Indents {
                stack: Vec::new(),
                is_reading: true,
                curr: LineIndent { levels: Vec::new() },
            },
            state: Vec::new(),
        }
    }

    pub fn save(&mut self) -> usize {
        if (self.state.last().is_none()) || (self.state.last().unwrap().pos == self.pos) {
            println!("SAVE: @{:?}.", self.pos);
        }

        let state = self.state();
        self.state.push(state);

        self.pos
    }

    pub fn restore(&mut self) -> usize {
        let state = self.state.pop().unwrap();
        if self.pos != state.pos {
            println!("RESTORE: {:?} ~> {:?}", self.pos, state.pos);
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
        println!(
            "READ: {}({}) => {}({}).",
            Cursor::char_to_string(self.char()),
            self.pos,
            Cursor::char_to_string(self.next()),
            self.pos + 1
        );

        self._update_indents();
        self.pos += 1;

        self.char()
    }

    pub fn skip(&mut self) {
        println!(
            "SKIP: {}({}) => {}({}).",
            Cursor::char_to_string(self.char()),
            self.pos,
            Cursor::char_to_string(self.next()),
            self.pos + 1
        );

        self._update_indents();
        self.pos += 1;
    }

    fn _update_indents(&mut self) {
        match self.char() {
            '\n' => {
                let curr = &self.indents.curr;
                self.indents.stack.push(curr.to_owned());
                self.indents.curr = LineIndent { levels: Vec::new() };
                self.indents.is_reading = true;
            }
            '\t' | ' ' => {
                if self.indents.is_reading {
                    if self.indents.matches_prev() {
                        if self.indents.prev_levels() == self.indents.curr_levels() {
                            if self.indents.prev().levels.last().unwrap().size
                                == self.indents.curr.levels.last().unwrap().size
                            {
                                self.indents.is_reading = false;
                            }
                        }
                    }
                }
            }
            _ => {
                self.indents.is_reading = false;
            }
        }
    }

    pub fn read_(&mut self, n: usize) -> Vec<char> {
        let mut result = Vec::new();
        for _ in 0..n {
            result.push(self.read());
        }
        return result;
    }

    pub fn try_read(&mut self, c: char) -> bool {
        if self.is(c) {
            self.read();
            return true;
        }
        return false;
    }

    // TODO: return a ws token with is_ignored = true
    pub fn skip_ws(&mut self) {
        println!("SKIPPING-WS: {}..", self.pos);
        self.skip_while(|c| c.is_whitespace());
        println!("SKIPPED-WS: ..{}", self.pos)
    }

    pub fn skip_while(&mut self, f: fn(char) -> bool) {
        println!("SKIPPING-WHILE: {}..", self.pos);
        while f(self.char()) {
            self.skip();
        }
        println!("SKIPPED-WHILE: ..{}", self.pos);
    }

    pub fn skip_until(&mut self, f: fn(char) -> bool) {
        println!("SKIPPING-UNTIL: {}..", self.pos);
        while !f(self.char()) {
            self.skip();
        }
        println!("SKIPPED-UNTIL: ..{}", self.pos);
    }

    pub fn read_while(&mut self, f: fn(char) -> bool) -> Vec<char> {
        println!("READING-WHILE: {}..", self.pos);
        let mut result = Vec::new();
        while f(self.char()) {
            result.push(self.read());
        }
        println!("READ-WHILE: ..{}", self.pos);
        return result;
    }

    pub fn read_until(&mut self, f: fn(char) -> bool) -> Vec<char> {
        let mut result = Vec::new();
        while !f(self.char()) {
            result.push(self.read());
        }
        return result;
    }

    pub fn char(&self) -> char {
        return self.at(self.pos);
    }

    pub fn next(&self) -> char {
        return self.ahead(1);
    }

    pub fn prev(&self) -> char {
        return self.back(1);
    }

    pub fn start(&self) -> usize {
        return self.state.last().unwrap().pos;
    }

    pub fn is(&self, c: char) -> bool {
        return self.char() == c;
    }

    pub fn next_is(&self, c: char) -> bool {
        return self.next() == c;
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

    pub fn eof(&self) -> bool {
        return self.eof_at(self.pos);
    }

    pub fn eof_at(&self, pos: usize) -> bool {
        return pos == self.src.len() - 1;
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
}
