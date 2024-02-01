use super::end::End;

pub trait Builder<TResult> {
    /// build the token with the provided start and end values.
    fn build(self, start: usize, end: usize) -> TResult;
    /// end the token at the current cursor position.
    fn end(self) -> End;
    /// build the token using the provided end and already set start value.
    fn build_to(self, end: usize) -> TResult;
}
