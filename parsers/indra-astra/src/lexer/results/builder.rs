use crate::End;

pub trait Builder<TResult> {
    fn build(self, start: usize, end: usize) -> TResult;
    fn end(self) -> End;
}
