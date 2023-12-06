pub mod tag {
    use crate::runtime::nodes::procs::reference;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Node<'rt> {
        refr: reference::Node<'rt>,
    }
}

pub mod alias {
    pub struct Node {}
}
