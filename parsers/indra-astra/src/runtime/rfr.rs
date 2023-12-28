use super::nodes::{Any, Entry, Key, Node, Primitive, Procedural, Struct, Structure, Value};
use crate::runtime::Runtime;
use slotmap::Key as _;
use std::marker::PhantomData;

slotmap::new_key_type! {
    pub struct RId;
}

pub type Rfr<T> = Reference<T>;

#[derive(Debug, Copy, Eq, Hash, PartialEq)]
pub struct Reference<T: Node> {
    __: PhantomData<T>,
    id: RId,
}

impl<T: Node> Clone for Rfr<T> {
    fn clone(&self) -> Self {
        Self {
            __: PhantomData,
            id: self.id,
        }
    }
}

impl<T: Node> Rfr<T> {
    #[allow(non_snake_case)]
    pub(crate) fn New<'rt>(rt: &'rt mut Runtime, value: T) -> Self {
        let id = rt._add_node(value.as_node());
        Self {
            __: PhantomData,
            id,
        }
    }

    #[allow(non_snake_case)]
    pub fn To<'rt>(rfr: Rfr<T>) -> Rfr<T> {
        rfr.clone()
    }

    #[allow(non_snake_case)]
    pub(crate) fn Empty() -> Self {
        Self {
            __: PhantomData,
            id: RId::null(),
        }
    }

    pub fn get<'rt>(&'rt self, rt: &'rt Runtime) -> &'rt T {
        todo!()
    }

    pub(crate) fn get_mut<'rt>(&'rt self, rt: &'rt Runtime) -> &'rt mut T {
        todo!()
    }

    pub(crate) fn cast<R: Node>(&self) -> Rfr<R> {
        Rfr {
            __: PhantomData,
            id: self.id,
        }
    }
}

impl Rfr<Entry> {
    pub fn get_value<'rt>(&'rt self, rt: &'rt Runtime) -> &Rfr<Value> {
        self.get(rt).get_value()
    }

    pub fn get_key<'rt>(&self, rt: &'rt Runtime) -> Key {
        self.get(rt).get_key()
    }
}

impl Rfr<Value> {
    pub fn as_structure(&self) -> Rfr<Structure> {
        self.cast()
    }
    pub fn as_struct(&self) -> Rfr<Struct> {
        self.cast()
    }
    pub fn as_proc(&self) -> Rfr<Procedural> {
        self.cast()
    }
    pub fn as_prim(&self) -> Rfr<Primitive> {
        self.cast()
    }
    pub fn as_ref(&self) -> Rfr<Any> {
        self.cast()
    }
}

pub trait Source {
    fn add_node<'rt, T>(&mut self, value: T) -> Rfr<T>
    where
        T: Node;
}

impl<'rt> Source for Runtime<'rt> {
    fn add_node<T>(&mut self, value: T) -> Rfr<T>
    where
        T: Node,
    {
        Rfr::New(self, value)
    }
}
