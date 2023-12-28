pub mod prj;

use super::{
    rfr::{Rfr, Source},
    Runtime,
};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Key;

pub trait Node {
    fn as_node(self) -> Any;
}

pub struct Procedural {
    // The entry that this structure is contained within.
    source: Option<Rfr<Entry>>,
    // The body/logic/resulting/prototype level properties and traits of this procedural.
    body: Struct,
    // The archetypical/static/type-level properties and traits of this procedural.
    meta: Struct,
}

impl Node for Procedural {
    fn as_node(self) -> Any {
        Any::Val(Value::Stx(Structure::Prx(self)))
    }
}

pub struct Struct {
    // The entry that this structure is contained within.
    source: Option<Rfr<Entry>>,
    // The traits that this structure has implemented on its `own`.
    traits: Option<HashMap<Key, Rfr<Trait>>>,
    // The entries that this structure has implemented on its `own`.
    entries: Option<HashMap<Key, Rfr<Entry>>>,
}

impl Struct {
    #[allow(non_snake_case)]
    pub fn In_Entry(source: Rfr<Entry>) -> Self {
        Struct::New(source, None, None)
    }

    #[allow(non_snake_case)]
    pub fn New(
        source: Rfr<Entry>,
        traits: Option<HashMap<Key, Rfr<Trait>>>,
        own: Option<HashMap<Key, Rfr<Entry>>>,
    ) -> Self {
        Struct {
            traits,
            entries: own,
            source: Some(source),
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn Root(rt: &mut Runtime) -> Rfr<Self> {
        Rfr::New(
            rt,
            Struct {
                traits: None,
                entries: None,
                source: None,
            },
        )
    }
}

impl Node for Struct {
    fn as_node(self) -> Any {
        Any::Val(Value::Stx(Structure::Stx(self)))
    }
}

pub enum Any {
    Key(Key),
    Val(Value),
    Var(Entry),
    Trt(Trait),
}

impl Node for Any {
    fn as_node(self) -> Any {
        self
    }
}

pub enum Value {
    Stx(Structure),
    Pmv(Primitive),
    Ref(Rfr<Any>),
}

impl Node for Value {
    fn as_node(self) -> Any {
        Any::Val(self)
    }
}

pub enum Structure {
    Stx(Struct),
    Prx(Procedural),
}

impl Node for Structure {
    fn as_node(self) -> Any {
        Any::Val(Value::Stx(self))
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
    fn as_node(self) -> Any {
        Any::Val(Value::Pmv(self))
    }
}

pub struct Entry {
    // The key of this entry within its parent/super structure.
    key: Key,
    // The value of this entry.
    value: Rfr<Value>,
    // The parent/super structure of this entry.
    source: Rfr<Struct>,
}

impl Entry {
    #[allow(non_snake_case)]
    pub(crate) fn Root<'rt>(rt: &mut Runtime) -> Rfr<Entry> {
        let root_entry = Entry {
            key: Key,
            value: Rfr::Empty(),
            source: Struct::Root(rt),
        };

        let root_source = rt.add_node(root_entry);
        let globals = Struct::In_Entry(root_source.clone());
        let globals_source = Value::Stx(Structure::Stx(globals));
        rt.set_value(&root_source, globals_source);

        root_source
    }

    #[allow(non_snake_case)]
    pub fn Empty() -> Self {
        Entry {
            key: Key,
            value: Rfr::Empty(),
            source: Rfr::Empty(),
        }
    }

    #[allow(non_snake_case)]
    pub fn New(source: Rfr<Struct>, key: Key, value: Value, rt: &mut Runtime) -> Self {
        Entry {
            key,
            value: Rfr::New(rt, value),
            source: source,
        }
    }

    pub fn get_key(&self) -> Key {
        self.key
    }

    pub fn get_value(&self) -> &Rfr<Value> {
        &self.value
    }

    pub(crate) fn set_value(&mut self, value: Value, rt: &mut Runtime) -> Rfr<Value> {
        self.value = Rfr::New(rt, value);
        self.value.clone()
    }
}

impl Node for Entry {
    fn as_node(self) -> Any {
        Any::Var(self)
    }
}

pub struct Trait {
    // The key of this trait within its parent/super structure.
    key: Key,
    // The source entry of this trait.
    value: Rfr<Entry>,
    // The parent/super structure that this trait is applied to.
    source: Rfr<Struct>,
}

impl Node for Trait {
    fn as_node(self) -> Any {
        Any::Trt(self)
    }
}

pub trait Modder {
    fn set_value(&mut self, entry: &Rfr<Entry>, value: Value) -> Rfr<Value>;
    fn set_value_to(&self, entry: &Rfr<Entry>, value: Rfr<Value>);
}

impl<'rt> Modder for Runtime<'rt> {
    fn set_value(&mut self, entry: &Rfr<Entry>, value: Value) -> Rfr<Value> {
        let source = self.add_node(value);
        let entry = entry.get_mut(self);
        entry.value = source;

        entry.value.clone()
    }

    fn set_value_to(&self, source: &Rfr<Entry>, value: Rfr<Value>) {
        let entry = source.get_mut(self);
        entry.value = value;
    }
}
