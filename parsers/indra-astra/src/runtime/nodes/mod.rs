pub mod prj;
use super::{
    cell::{Cell, Opt, Rfr, Src},
    Runtime,
};
use std::{borrow::BorrowMut, collections::HashMap};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Key;

type TNode = Any;

pub trait Node {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self;
    fn Downcast<'rt>(any: &'rt TNode) -> &Self;

    fn as_any(self) -> Any;
}

impl Node for Key {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self {
        match any {
            Any::Key(key) => key,
            _ => panic!("Expected Any::Key"),
        }
    }

    #[allow(non_snake_case)]
    fn Downcast<'rt>(any: &'rt TNode) -> &Self {
        match any {
            Any::Key(key) => key,
            _ => panic!("Expected Any::Key"),
        }
    }

    fn as_any(self) -> Any {
        Any::Key(self)
    }
}

pub struct Procedural {
    // The entry that this structure is contained within.
    source: Opt<Rfr<Entry>>,
    // The body/logic/resulting/prototype level properties and traits of this procedural.
    body: Structure,
    // The archetypical/static/type-level properties and traits of this procedural.
    meta: Structure,
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
            Any::Val(Value::Stx(stx)) => match stx {
                Struct::Stx(stx) => stx,
                _ => panic!("Expected Struct::Stx"),
            },
            _ => panic!("Expected Any::Val(Value::Stx)"),
        }
    }

    #[allow(non_snake_case)]
    fn Downcast<'rt>(any: &'rt TNode) -> &Self {
        match any {
            Any::Val(Value::Stx(stx)) => match stx {
                Struct::Stx(stx) => stx,
                _ => panic!("Expected Struct::Stx"),
            },
            _ => panic!("Expected Any::Val(Value::Stx)"),
        }
    }

    fn as_any(self) -> Any {
        Any::Val(Value::Stx(Struct::Stx(self)))
    }
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

    #[allow(non_snake_case)]
    fn Downcast<'rt>(any: &'rt TNode) -> &Self {
        any.as_any().borrow_mut()
    }

    fn as_any(self) -> Any {
        self
    }
}

pub enum Value {
    Stx(Struct),
    Pmv(Primitive),
    Ref(Cell<Any>),
}

pub enum Struct {
    Stx(Structure),
    Prx(Procedural),
}

impl Node for Value {
    #[allow(non_snake_case)]
    fn Unwrap(any: &mut TNode) -> &mut Self {
        match any {
            Any::Val(value) => value,
            _ => panic!("Expected Any::Val"),
        }
    }

    #[allow(non_snake_case)]
    fn Downcast<'rt>(any: &'rt TNode) -> &Self {
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

    #[allow(non_snake_case)]
    fn Downcast<'rt>(any: &'rt TNode) -> &Self {
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
    pub(crate) fn Root<'rt>(rt: &mut Runtime) -> Src<Entry> {
        let root_entry = Entry {
            key: Key,
            value: Src::Empty(),
            source: Rfr::To(&Structure::Root(rt)),
        };

        let root_source = Src::Of(root_entry, rt);
        let globals = Structure::In_Entry(&root_source);
        let globals_source = Src::Of(Value::Stx(Struct::Stx(globals)), rt);
        Entry::Unwrap(&mut root_source.get(rt)).value = globals_source;

        root_source
    }

    #[allow(non_snake_case)]
    pub fn Empty() -> Self {
        Entry {
            key: Key,
            value: Src::Empty(),
            source: Rfr::Empty(),
        }
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
            Any::Var(ref mut var) => var,
            _ => panic!("Expected Any::Var"),
        }
    }

    #[allow(non_snake_case)]
    fn Downcast<'rt>(any: &'rt TNode) -> &Self {
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

    #[allow(non_snake_case)]
    fn Downcast<'rt>(any: &'rt TNode) -> &Self {
        match any {
            Any::Trt(trt) => trt,
            _ => panic!("Expected Any::Trt"),
        }
    }

    fn as_any(self) -> Any {
        Any::Trt(self)
    }
}
