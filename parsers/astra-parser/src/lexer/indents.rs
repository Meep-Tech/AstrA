#[derive(Debug, Clone)]
pub struct Indents {
    pub curr: usize,
    pub stack: Vec<usize>,
    pub is_reading: bool,
}

#[derive(Debug, Clone)]
pub struct LineIndent {
    pub size: usize,
}

impl Indents {
    pub fn prev(&self) -> usize {
        return self.stack.last().unwrap_or(&0).clone();
    }
}
