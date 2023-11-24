pub struct Cursor {
    pub src: Vec<char>,
    pub pos: usize,
}

pub struct State {
    pub pos: usize,
}

impl Cursor {
    pub fn new(source: &str) -> Cursor {
        println!("Creating new cursor for input of length: {}", source.len());
        let src: Vec<char> = (source.to_string() + "\0").chars().collect();
        Cursor { pos: 0, src }
    }

    pub fn save(&self) -> State {
        println!("SAVE: {:?}.", self.pos);
        State { pos: self.pos }
    }

    pub fn restore(&mut self, state: State) {
        if self.pos != state.pos {
            println!("RESTORE: {:?} -> {:?}.", self.pos, state.pos);
            self.pos = state.pos;
        }
    }

    pub fn read(&mut self) -> char {
        println!(
            "READ: {}({}) => {}({}).",
            match self.char() {
                '\n' => "\\n".to_string(),
                '\t' => "\\t".to_string(),
                ' ' => "| |".to_string(),
                '\0' => "\\0".to_string(),
                c => c.to_string(),
            },
            self.pos,
            match self.next() {
                '\n' => "\\n".to_string(),
                '\t' => "\\t".to_string(),
                ' ' => "| |".to_string(),
                '\0' => "\\0".to_string(),
                c => c.to_string(),
            },
            self.pos + 1
        );
        return {
            self.pos += 1;
            self.char()
        };
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

    pub fn skip_ws(&mut self) {
        println!("SKIPPING-WS: {}..", self.pos);
        self.skip_while(|c| c.is_whitespace());
    }

    pub fn skip_while(&mut self, f: fn(char) -> bool) {
        while f(self.char()) {
            self.read();
        }
    }

    pub fn skip_until(&mut self, f: fn(char) -> bool) {
        while !f(self.char()) {
            self.read();
        }
    }

    pub fn read_while(&mut self, f: fn(char) -> bool) -> Vec<char> {
        let mut result = Vec::new();
        while f(self.char()) {
            result.push(self.read());
        }
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
}
