#[derive(Debug, Clone)]
pub struct Indents {
    pub curr: usize,
    pub stack: Vec<usize>,
    pub is_reading: bool,
}

impl Indents {
    pub type Diff = Diff;

    pub fn curr(&self) -> usize {
        return self.curr;
    }

    pub fn prev(&self) -> usize {
        return self.stack.last().unwrap_or(&0).clone();
    }

    pub fn diff(&self) -> Diff {
        if self.curr > self.prev() {
            return Diff::Increase;
        } else if self.curr < self.prev() {
            return Diff::Decrease;
        }

        Diff::None
    }
}

pub enum Diff {
    None,
    Increase,
    Decrease,
}

impl Diff {
    pub fn is_same(&self) -> bool {
        match self {
            Diff::None => true,
            _ => false,
        }
    }

    pub fn is_more(&self) -> bool {
        match self {
            Diff::Increase => true,
            _ => false,
        }
    }

    pub fn is_less(&self) -> bool {
        match self {
            Diff::Decrease => true,
            _ => false,
        }
    }
}
