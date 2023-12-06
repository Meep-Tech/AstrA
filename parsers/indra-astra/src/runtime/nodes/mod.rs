use std::collections::HashSet;

use indexmap::IndexMap;
use lazy_static::lazy_static;

pub mod attributes;
pub mod entries;
pub mod procs;
pub mod structs;

lazy_static! {
    pub static ref _EMPTY_ENTRIES: IndexMap<Key, Entry<'static>> = IndexMap::new();
    pub static ref _EMPTY_TAGS: HashSet<attributes::tag::Node<'static>> = HashSet::new();
}

pub trait Node<'rt> {
    fn get_type(&self) -> Any;

    fn get_tags(&self) -> &HashSet<attributes::tag::Node> {
        &_EMPTY_TAGS
    }

    fn has_tag(&self, tag: &attributes::tag::Node) -> bool {
        self.get_tags().contains(tag)
    }

    fn get_entries(&self) -> &'rt IndexMap<Key, Entry<'rt>> {
        &_EMPTY_ENTRIES
    }

    fn get_entry(&self, key: Key) -> Option<&Entry<'rt>> {
        let get = self.get_entries().get(&key);
        get
    }
}

pub enum Any<'rt> {
    Id(Identifier),
    Prim(Primitive),
    Struct(Structure<'rt>),
    Proc(Procedural<'rt>),
    Entry(Entry<'rt>),
    Attr(Attribute<'rt>),
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Identifier {
    Key(Key),
}

pub enum Primitive {
    Bol(bool),
    Num(Number),
    Str,
}

pub enum Structure<'rt> {
    Struct(&'rt structs::Node),
    Group(&'rt structs::group::Node),
}

pub enum Procedural<'rt> {
    Pro(&'rt procs::prototype::Node),
    Arc(&'rt procs::archetype::Node),
    Ref(&'rt procs::reference::Node<'rt>),
    Fun(&'rt procs::function::Node),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Entry<'rt> {
    Named(&'rt entries::named_entry::Node),
}

pub enum Attribute<'rt> {
    Tag(&'rt procs::reference::Node<'rt>),
    Alias(Entry<'rt>),
}

pub enum Number {
    Int(i64),
    Dec(f64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Name(String),
    Index(i64),
}
