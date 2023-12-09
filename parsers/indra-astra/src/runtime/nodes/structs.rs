use super::{super::nodes, node, Any, Key, NEntries, Structure};

pub struct Node<'rt> {
    entries: NEntries<'rt>,
    //named are not added to ordered by default, need to be explicit too.
}

impl<'rt> Node<'rt> {
    #[allow(non_snake_case)]
    pub fn New() -> Builder<'rt> {
        Builder {
            entries: NEntries::new(),
        }
    }
}

impl<'rt> nodes::Node<'rt> for Node<'rt> {
    fn to_any(&self) -> nodes::Any {
        Any::Struct(Structure::Struct(&self))
    }

    fn to_struct(&self) -> nodes::Struct<'rt> {
        *self
    }
}

pub struct Builder<'rt> {
    entries: NEntries<'rt>,
}

impl<'rt> Builder<'rt> {
    pub fn group(self) -> group::Builder<'rt> {
        return group::Builder::With_Own(self);
    }
    pub fn array(self) -> array::Builder<'rt> {
        return array::Builder::With_Own(self);
    }
}

impl<'rt> nodes::Builder<'rt, Node<'rt>> for Builder<'rt> {
    fn build(self) -> Node<'rt> {
        End::TODO()
    }
}

trait IBuilder<'rt> {
    fn entry(key: Key, entry: Entry) -> Self;
}

//Also.... rust has made me re-think doing the whole default return via omitted semi-colon thing, because it can be really annoying to find which one of the match statements like 5 deep is the one with the missing or added semicolon xD

// snode!(
//   group {}
//   type: |n| nodes::Any::Struct(nodes::Structure::Group(n)),
//   struct: |n, b| b
// );

node! {
  group {}
}

node!(
  array {}
  type: |n| nodes::Any::Struct(nodes::Structure::Array(n))
);

node!(
  map {}
  type: |n| nodes::Any::Struct(nodes::Structure::Map(n))
);
