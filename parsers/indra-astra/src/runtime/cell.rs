//srs as a field should be used for internal parenting for source of truth stuff related to refs

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};

use super::node::{Entry, Key};

pub type Opt<T> = Option<T>;
pub type Rfr<T> = Reference<T>;
pub type Srs<T> = Source<T>;

pub trait Scope<'rt> {
    fn sup(&self) -> Reference<dyn Scope>;
    fn sub(&self, key: Key) -> Opt<Reference<Entry>>;
}

#[derive(Debug)]
pub enum Cell<T> {
    Srs(Source<T>),
    Ref(Reference<T>),
}

#[derive(Debug, Clone)]
pub struct Source<T: ?Sized> {
    raw: Rc<RefCell<T>>,
}

#[derive(Debug, Clone)]
pub struct Reference<T: ?Sized> {
    raw: Weak<RefCell<T>>,
}

impl<T> Source<T> {
    #[allow(non_snake_case)]
    pub fn New(raw: T) -> Self {
        Self {
            raw: Rc::new(RefCell::new(raw)),
        }
    }
}

impl<T> Reference<T> {
    #[allow(non_snake_case)]
    pub fn New(source: &Source<T>) -> Self {
        Self {
            raw: Rc::downgrade(&source.raw),
        }
    }
}

pub trait Referable<T> {
    fn get(&self) -> Ref<T>;
    fn get_mut(&mut self) -> RefMut<T>;
    fn get_ref(&self) -> Reference<T>;
    fn get_cell(&self) -> Cell<T>;
}

impl<T> Referable<T> for Cell<T> {
    fn get(&self) -> Ref<T> {
        match self {
            Cell::Srs(s) => s.get(),
            Cell::Ref(r) => r.get(),
        }
    }

    fn get_mut(&mut self) -> RefMut<T> {
        match self {
            Cell::Srs(s) => s.get_mut(),
            Cell::Ref(r) => r.get_mut(),
        }
    }

    fn get_ref(&self) -> Reference<T> {
        match self {
            Cell::Srs(s) => s.get_ref(),
            Cell::Ref(r) => r.get_ref(),
        }
    }

    fn get_cell(&self) -> Cell<T> {
        match self {
            Cell::Srs(s) => s.get_cell(),
            Cell::Ref(r) => r.get_cell(),
        }
    }
}

impl<T> Referable<T> for Source<T> {
    fn get(&self) -> Ref<T> {
        self.raw.as_ref().borrow()
    }

    fn get_mut(&mut self) -> RefMut<T> {
        self.raw.borrow_mut()
    }

    fn get_ref(&self) -> Reference<T> {
        Reference::New(self)
    }

    fn get_cell(&self) -> Cell<T> {
        Cell::Ref(Reference::New(self))
    }
}

impl<T> Referable<T> for Reference<T> {
    fn get(&self) -> Ref<T> {
        todo!()
    }

    fn get_mut(&mut self) -> RefMut<T> {
        todo!()
    }

    fn get_ref(&self) -> Reference<T> {
        todo!()
    }

    fn get_cell(&self) -> Cell<T> {
        todo!()
    }
}
