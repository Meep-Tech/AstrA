use super::cell::{Cell, Referable, Reference, Source, Srs};
use std::collections::HashMap;

// pub type Srs<T> = Rc<RefCell<T>>;
// pub type Ref<T> = Rc<RefCell<T>>;

// pub trait Refable<T> {
//     #[allow(non_snake_case)]
//     fn New(val: T) -> Srs<T> {
//         Rc::new(RefCell::new(val))
//     }

//     fn get_ref(&self) -> Ref<T>;
// }

// impl<T> Refable<T> for Srs<T> {
//     fn get_ref(&self) -> Ref<T> {
//         Rc::clone(&self)
//     }
// }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Key;

pub struct Structure {
    traits: Option<HashMap<Key, Srs<Trait>>>,
    own: Option<HashMap<Key, Srs<Entry>>>,
    // This structure as an entry within its parent/super structure.
    source: Option<Reference<Entry>>,
}

impl Structure {
    #[allow(non_snake_case)]
    pub fn New(srs: Reference<Entry>) -> Self {
        Structure {
            traits: None,
            own: None,
            source: Some(srs),
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn Void() -> Srs<Self> {
        Source::New(Structure {
            traits: None,
            own: None,
            source: None,
        })
    }
}

pub enum Value {
    Stx(Structure),
    Pmv(Primitive),
    Ref(Reference<Value>),
}

pub enum Primitive {
    Str(String),
    Int(i64),
    Bln(bool),
    Dec(f64),
    Nil,
}

pub struct Entry {
    // The key of this entry within its parent/super structure.
    key: Key,
    // The value of this entry.
    value: Srs<Value>,
    // The parent/super structure of this entry.
    source: Reference<Structure>,
}

impl Entry {
    #[allow(non_snake_case)]
    pub(crate) fn Root() -> Srs<Entry> {
        let root = Entry {
            key: Key,
            value: Srs::New(Value::Pmv(Primitive::Nil)),
            source: Reference::New(&Structure::Void()),
        };

        let mut root_source = Srs::New(root);
        let root_value = Srs::New(Value::Stx(Structure::New(root_source.get_ref())));

        root_source.get_mut().value = root_value;

        root_source
    }

    #[allow(non_snake_case)]
    pub fn New(key: Key, val: Srs<Value>, srs: Reference<Structure>) -> Self {
        Entry {
            key,
            value: val,
            source: srs,
        }
    }
}

pub struct Trait {
    // The key of this trait within its parent/super structure.
    key: Key,
    // The source entry of this trait.
    value: Reference<Entry>,
    // The parent/super structure that this trait is applied to.
    source: Reference<Structure>,
}

// pub trait Node {
//     /// borrow a hashmap with all of the node's tags.
//     fn tags() -> &'n HashMap<Key, Srs<Tag>>;

//     /// get one of the node's tags by key. (including inherited tags)
//     fn tags_(key: Key) -> Ref<Tag>;

//     /// borrow a hashmap with all of the node's traits. (including inherited traits)
//     fn trts() -> &'n HashMap<Key, Srs<Trait>>;

//     /// get one of the node's traits by key.
//     fn trts_(key: Key) -> Ref<Trait>;

//     /// borrow a hashmap with all of the node's owned entries.
//     fn own() -> &'n HashMap<Key, Srs<Entry>>;

//     /// get one of the node's owned entries by key.
//     fn own_(key: Key) -> Ref<Entry>;

//     /// Gets the containing entry of the current node as an Entry within its parent/super structure.
//     fn var() -> Ref<Entry>;

//     /// Get all of the node's own exposed entries, and then recursively through it's traits for their exposed entries.
//     fn vars() -> &'n Vec<Srs<Entry>>;

//     /// Get one of the node's entries by key;
//     /// Checking through own entries and then recursively through traits
//     fn vars_(key: Key) -> Ref<Entry>;
// }
