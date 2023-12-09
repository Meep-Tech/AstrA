use std::{collections::HashSet, rc::Rc};

use indexmap::IndexMap;
use lazy_static::lazy_static;

pub mod attributes;
pub mod entries;
pub mod procs;
pub mod structs;

pub type Struct<'rt> = structs::Node<'rt>;
pub type SBuilder<'rt> = structs::Builder<'rt>;
pub type NEntries<'rt> = IndexMap<Key, Entry<'rt>>;
pub type NTags<'rt> = HashSet<attributes::tag::Node<'rt>>;

lazy_static! {
    pub static ref _EMPTY_ENTRIES: NEntries<'static> = IndexMap::new();
    pub static ref _EMPTY_TAGS: NTags<'static> = HashSet::new();
    //pub static ref _NONE: Option<&'static dyn Node<'static>> = None;
}

//pub static _NONE: ONode<'static> = None;

pub trait Node<'rt> {
    fn to_any(&self) -> Any;
    fn to_struct(&self) -> Struct<'rt>;

    fn get_tags(&self) -> &NTags<'rt> {
        &_EMPTY_TAGS
    }

    fn has_tag(&self, tag: &attributes::tag::Node) -> bool {
        self.get_tags().contains(tag)
    }

    fn get_entries(&self) -> &NEntries<'rt> {
        &_EMPTY_ENTRIES
    }

    fn get_entry(&self, key: Key) -> Option<&Entry<'rt>> {
        let get = self.get_entries().get(&key);
        get
    }
}

pub trait Builder<'rt, TNode>
where
    TNode: Node<'rt>,
{
    fn build(self) -> TNode;
}

pub type ANode<'rt> = &'rt dyn Node<'rt>;
pub type ONode<'rt> = Option<ANode<'rt>>;
pub type RNode<'rt> = Rc<ANode<'rt>>;
pub type ORNode<'rt> = Option<RNode<'rt>>;

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
    Struct(&'rt structs::Node<'rt>),
    Map(&'rt structs::map::Node<'rt>),
    Array(&'rt structs::array::Node<'rt>),
    Group(&'rt structs::group::Node<'rt>),
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

macro_rules! node {
    (
        $name:ident { $($key:ident: $type:ty = $d:expr;)* }
        $(type: $a:expr)?
    ) => {
        pub mod $name {
            use crate::runtime::nodes;

            pub struct Node<'rt> {
                own: nodes::Struct<'rt>,
                $(pub $key: $type,)*
            }

            pub struct Builder<'rt> {
                own: nodes::SBuilder<'rt>,
                $(pub $key: $type,)*
            }

            impl<'rt> Node<'rt> {
                #[allow(non_snake_case)]
                fn New() -> Builder<'rt> {
                    Builder {
                        own: nodes::Struct::<'rt>::New(),
                        $($key: $d,)*
                    }
                }
            }
        }
    }
}
pub(crate) use node;

macro_rules! getter {
    ($typ:ident, $field:ident) => {
        pub fn get_$field(&self) -> ONode<'rt> {
            self.get_entry(Key::Name(stringify!($field).to_string()))
        }
    };
}
pub(crate) use getter;

// macro_rules! snode {
//     (
//         $k:ident { $($i:ident: $t:ty = $d:expr)* } $(,)?
//         type: $a:expr,
//         struct: $s:expr
//     ) => {
//         pub mod $k {
//             use crate::runtime::nodes;
//             use crate::runtime::nodes::Node as _;

//             pub struct Node<'rt> {
//                 _own: nodes::Struct<'rt>,
//                 $(pub $i: $t,)*
//             }

//             impl<'rt> nodes::Node<'rt> for Node<'rt> {
//                 fn to_any(&self) -> nodes::Any {
//                     let to_any = |current_node: &Self| -> nodes::Any { $a(current_node) };
//                     to_any(self)
//                 }

//                 fn to_struct(&self) -> nodes::Struct<'rt> {
//                     use crate::runtime::nodes::Builder;
//                     let mut stx = nodes::Struct::<'rt>::New();
//                     let to_struct = |current_node: &Self, result_builder: &mut nodes::SBuilder<'rt>| { $s(current_node, result_builder) };
//                     $s(&self, &mut stx);

//                     stx.build()
//                 }
//             }

//             pub struct Builder<'rt> {
//                 pub own: nodes::SBuilder<'rt>,
//                 $(pub $i: $t,)*
//             }

//             impl<'rt> Builder<'rt> {
//             }

//             impl<'rt> nodes::Builder<'rt, Node<'rt>> for Builder<'rt> {
//                 fn build(self) -> Node<'rt> {
//                     let mut node = Node::<'rt> {
//                         _own: nodes::Struct::<'rt>::New::<nodes::SBuilder>().build(),
//                         $($i,)*
//                     };

//                     node._own = self.own.build();

//                     node
//                 }
//             }
//         }
//     };

//     (
//         $k:ident { $($i:ident: $t:ty = $d:expr)* } $(,)?
//         struct: $s:expr
//     ) => {
//         snode!(
//             $k { $($i: $t = $d)* }
//             type: |n: &Self| crate::runtime::nodes::Any::Struct(nodes::Structure::Struct(&n.to_struct())),
//             struct: $s
//         );
//     };
// }

// pub(crate) use snode;
// (
//     $t:ident { $($nf:tt)* } $(,)?
//     struct: $s:expr
// ) => {
//     snode!(
//         $t { $($nf)* }
//         type: |n: &Self| crate::runtime::nodes::Any::Struct(nodes::Structure::Struct(&n.to_struct())),
//         struct: $s
//     );
// };

// (
//     $typ:ident { $($nf:tt)* } $(,)?
//     type: $a:expr,
//     struct: $s:expr,
//     build: $b:expr,
//     builder: { $($bf:tt)* }
// ) => {
//     pub mod $typ {
//         use crate::runtime::nodes;
//         use crate::runtime::nodes::Node as _;

//         pub struct Node<'rt> {
//             _own: nodes::Struct<'rt>,
//             $($nf)*
//         }

//         impl<'rt> nodes::Node<'rt> for Node<'rt> {
//             fn to_any(&self) -> nodes::Any {
//                 let to_any = |me: &Self| -> nodes::Any { $a(me) };
//                 to_any(self)
//             }

//             fn to_struct(&self) -> &nodes::Struct<'rt> {
//                 &$s(self)
//             }
//         }

//         pub struct Builder<'rt> {
//             pub own: nodes::SBuilder<'rt>,
//             $($bf)*
//         }

//         impl<'rt> Builder<'rt> {
//             #[allow(non_snake_case)]
//             pub fn With_Own<'rt>(builder: nodes::SBuilder<'rt>) -> Builder<'rt> {
//                 Self {own}
//             }
//         }

//         impl<'rt> nodes::Builder<'rt, Node<'rt>> for Builder<'rt> {
//             fn build(self) -> Node<'rt> {
//                 let mut node = Node::<'rt> {
//                     _own: nodes::Struct::<'rt>::New::<nodes::SBuilder>().build(),
//                 };
//                 node._own = self.own.build();
//                 let build
//                     = |mut node: Node<'rt>, builder: Self|
//                     -> Node<'rt> { ($b)(node, builder) };
//                 build(
//                     node,
//                     self,
//                 )
//             }
//         }
//     }
// };
