//use super::cell::{Opt, Rfr, Src};
use super::{
    cell::{Cell, Opt, Rfr, Src},
    Runtime,
};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Key;

type TNode = Any;

impl Node for Key {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self {
        match any {
            Any::Key(key) => key,
            _ => panic!("Expected Any::Key"),
        }
    }

    fn as_any(self) -> Any {
        Any::Key(self)
    }
}

pub struct Structure {
    // The entry that this structure is contained within.
    source: Opt<Rfr<Entry>>,
    // The traits that this structure has implemented on its `own`.
    traits: Opt<HashMap<Key, Src<Trait>>>,
    // The entries that this structure has implemented on its `own`.
    entries: Opt<HashMap<Key, Src<Entry>>>,
}

impl Structure {
    #[allow(non_snake_case)]
    pub fn In_Entry(source: &Src<Entry>) -> Self {
        Structure::New(source, None, None)
    }

    #[allow(non_snake_case)]
    pub fn New(
        source: &Src<Entry>,
        traits: Opt<HashMap<Key, Src<Trait>>>,
        own: Opt<HashMap<Key, Src<Entry>>>,
    ) -> Self {
        Structure {
            traits,
            entries: own,
            source: Some(Rfr::To(source)),
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn Root(rt: &mut Runtime) -> Src<Self> {
        Src::Of(
            Structure {
                traits: None,
                entries: None,
                source: None,
            },
            rt,
        )
    }
}

impl Node for Structure {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self {
        match any {
            Any::Val(Value::Stx(stx)) => stx,
            _ => panic!("Expected Any::Val(Value::Stx)"),
        }
    }

    fn as_any(self) -> Any {
        Any::Val(Value::Stx(self))
    }
}

pub trait Node {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self;
    fn as_any(self) -> Any;
}

pub enum Any {
    Key(Key),
    Val(Value),
    Var(Entry),
    Trt(Trait),
}

impl Node for Any {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self {
        any
    }

    fn as_any(self) -> Any {
        self
    }
}

pub enum Value {
    Stx(Structure),
    Pmv(Primitive),
    Ref(Cell<Any>),
}

impl Node for Value {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self {
        match any {
            Any::Val(value) => value,
            _ => panic!("Expected Any::Val"),
        }
    }

    fn as_any(self) -> Any {
        Any::Val(self)
    }
}

pub enum Primitive {
    Str(String),
    Int(i64),
    Bln(bool),
    Dec(f64),
    Nil,
}

impl Node for Primitive {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self {
        match any {
            Any::Val(Value::Pmv(pmv)) => pmv,
            _ => panic!("Expected Any::Val(Value::Pmv)"),
        }
    }

    fn as_any(self) -> Any {
        Any::Val(Value::Pmv(self))
    }
}

pub struct Entry {
    // The key of this entry within its parent/super structure.
    key: Key,
    // The value of this entry.
    value: Src<Value>,
    // The parent/super structure of this entry.
    source: Rfr<Structure>,
}

impl Entry {
    #[allow(non_snake_case)]
    pub(crate) fn Root(rt: &mut Runtime) -> Src<Entry> {
        let root_entry = Entry {
            key: Key,
            value: Src::Of(Value::Pmv(Primitive::Nil), rt),
            source: Rfr::To(&Structure::Root(rt)),
        };

        let root_source = Src::Of(root_entry, rt);
        let globals = Structure::In_Entry(&root_source);
        let globals_source = Src::Of(Value::Stx(globals), rt);
        Entry::Unwrap(&mut root_source.get(&rt)).value = globals_source;

        root_source
    }

    #[allow(non_snake_case)]
    pub fn New(source: Src<Structure>, key: Key, value: Value, rt: &mut Runtime) -> Self {
        Entry {
            key,
            value: Src::Of(value, rt),
            source: Rfr::To(&source),
        }
    }

    pub fn value(&mut self) -> &Src<Value> {
        &self.value
    }
}

impl Node for Entry {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self {
        match any {
            Any::Var(var) => var,
            _ => panic!("Expected Any::Var"),
        }
    }

    fn as_any(self) -> Any {
        Any::Var(self)
    }
}

pub struct Trait {
    // The key of this trait within its parent/super structure.
    key: Key,
    // The source entry of this trait.
    value: Rfr<Entry>,
    // The parent/super structure that this trait is applied to.
    source: Rfr<Structure>,
}

impl Node for Trait {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self {
        match any {
            Any::Trt(trt) => trt,
            _ => panic!("Expected Any::Trt"),
        }
    }

    fn as_any(self) -> Any {
        Any::Trt(self)
    }
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
