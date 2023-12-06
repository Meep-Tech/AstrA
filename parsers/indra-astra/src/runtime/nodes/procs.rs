pub mod reference {
    use std::sync::Arc;

    use crate::runtime::nodes::Entry;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Node<'rt> {
        pub target: Arc<Entry<'rt>>,
    }
}

pub mod prototype {
    pub struct Node {}
}

pub mod archetype {
    pub struct Node {}
}

pub mod function {
    pub struct Node {}
}
