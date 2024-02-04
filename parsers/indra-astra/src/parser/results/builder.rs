use super::end::End;

pub trait Builder<TResult> {
    /// the number of children in the builder.
    fn len(&self) -> usize;
    /// build the result using existing start and end values.
    fn build(self) -> TResult;
    /// build the token with the provided start and end values.
    fn build_from(self, start: usize, to_end: usize) -> TResult;
    /// end the token at the current cursor position.
    fn to_end(self) -> End;
    /// build the token using the provided end and already set start value.
    fn build_to(self, end: usize) -> TResult;
    /// build the token using the provided start and already set end value.
    fn build_at(self, start: usize) -> TResult;
    /// uild using the existing start and end values, or defaults if not set.
    fn build_with_defaults(self, start: usize, end: usize) -> TResult;
}
