#[derive(Debug, Clone)]
pub struct Indents {
    pub curr: LineIndent,
    pub stack: Vec<LineIndent>,
    pub is_reading: bool,
}

#[derive(Debug, Clone)]
pub struct LineIndent {
    pub levels: Vec<IndentLevel>,
}

impl LineIndent {
    pub fn level(&self) -> usize {
        self.levels.len()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IndentLevel {
    pub size: usize,
}

impl Indents {
    pub fn prev(&self) -> &LineIndent {
        return &self.stack.last().unwrap();
    }

    pub fn curr_levels(&self) -> usize {
        self.curr.level()
    }

    pub fn prev_levels(&self) -> usize {
        self.prev().level()
    }

    pub fn prev_last_level(&self) -> &IndentLevel {
        self.prev().levels.last().unwrap()
    }

    pub fn current_level(&self) -> &IndentLevel {
        self.curr.levels.last().unwrap()
    }

    pub fn matches_prev(&self) -> bool {
        return self.curr_levels() == self.prev_levels();
    }
}
