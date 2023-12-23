use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

pub type Srs<T> = Rc<RefCell<T>>;
pub type Ref<T> = Rc<RefCell<T>>;

pub trait Refable<T> {
    #[allow(non_snake_case)]
    fn New(val: T) -> Srs<T> {
        Rc::new(RefCell::new(val))
    }

    fn get_ref(&self) -> Ref<T>;
}

impl<T> Refable<T> for Srs<T> {
    fn get_ref(&self) -> Ref<T> {
        Rc::clone(&self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Key;

pub struct Structure {
    trts: Option<HashMap<Key, Srs<Trait>>>,
    own: Option<HashMap<Key, Srs<Entry>>>,
    // This structure as an entry within its parent/super structure.
    var: Option<Ref<Entry>>,
}

impl Structure {
    #[allow(non_snake_case)]
    pub fn New(var: Ref<Entry>) -> Self {
        Structure {
            trts: None,
            own: None,
            var: Some(var),
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn Void() -> Self {
        Structure {
            trts: None,
            own: None,
            var: None,
        }
    }
}

pub enum Value {
    STX(Structure),
    PMV(Primitive),
    REF(Ref<Value>),
}

pub enum Primitive {
    STR(String),
    INT(i64),
    BLN(bool),
    DEC(f64),
    NIL,
}

pub struct Entry {
    key: Key,
    val: Srs<Value>,
    // The parent/super structure of this entry.
    srs: Ref<Structure>,
}

impl Entry {
    #[allow(non_snake_case)]
    pub(crate) fn Root() -> Srs<Entry> {
        let root = Entry {
            key: Key,
            val: Srs::New(Value::PMV(Primitive::NIL)),
            srs: Ref::New(Structure::Void()),
        };

        let root_srs = Srs::New(root);
        let root_val = Srs::New(Value::STX(Structure::New(root_srs.get_ref())));

        (*root_srs).borrow_mut().borrow_mut().val = root_val;

        root_srs
    }

    #[allow(non_snake_case)]
    pub fn New(key: Key, val: Srs<Value>, srs: Ref<Structure>) -> Self {
        Entry { key, val, srs }
    }
}

pub struct Trait {
    // The key of this trait within its parent/super structure.
    key: Key,
    // The source entry of this trait.
    var: Ref<Entry>,
    // The parent/super structure of this trait.
    srs: Ref<Structure>,
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
