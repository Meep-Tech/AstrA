use std::{marker::PhantomData, sync::MutexGuard};

use slotmap::{DefaultKey, Key};

use super::{
    node::{Any, Node},
    Runtime,
};

pub type Id = DefaultKey;
pub type Opt<T> = Option<T>;
pub type Src<T> = Source<T>;
pub type Rfr<T> = Reference<T>;

pub struct Source<T: Node> {
    __: PhantomData<T>,
    id: Id,
}

pub struct Reference<T: Node> {
    __: PhantomData<T>,
    id: Id,
}

pub enum Cell<T: Node> {
    Src(Source<T>),
    Rfr(Reference<T>),
}

impl<T: Node> Cell<T> {
    pub fn id(&self) -> Id {
        match self {
            Self::Src(src) => src.id,
            Self::Rfr(rfr) => rfr.id,
        }
    }

    pub fn get<'rt>(&'rt self, rt: &'rt Runtime) -> MutexGuard<Any> {
        match self {
            Self::Src(src) => src.get(rt),
            Self::Rfr(rfr) => rfr.get(rt),
        }
    }
}

impl<T: Node> Source<T> {
    #[allow(non_snake_case)]
    pub(crate) fn Empty() -> Self {
        Self {
            __: PhantomData,
            id: Id::null(),
        }
    }

    #[allow(non_snake_case)]
    pub fn Of(value: T, rt: &mut Runtime) -> Self {
        let id = rt._add_node(value.as_any());
        Self {
            __: PhantomData,
            id,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn get<'rt>(&'rt self, rt: &'rt Runtime) -> MutexGuard<Any> {
        let node = rt._get_node(self.id());
        node.lock().unwrap()
    }
}

impl<T: Node> Reference<T> {
    #[allow(non_snake_case)]
    pub fn To(source: &Src<T>) -> Self {
        Self {
            __: PhantomData,
            id: source.id,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn get<'rt>(&'rt self, rt: &'rt Runtime) -> MutexGuard<Any> {
        let node = rt._get_node(self.id());
        node.lock().unwrap()
    }
}
