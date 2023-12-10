use crate::lexer::results::node::Node;

pub trait Span<TNode>: Node<TNode> {
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn range(&self) -> std::ops::Range<usize> {
        return self.start()..self.end();
    }
}
