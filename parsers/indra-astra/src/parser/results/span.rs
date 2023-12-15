pub trait Span {
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn range(&self) -> std::ops::Range<usize> {
        return self.start()..self.end();
    }
}
