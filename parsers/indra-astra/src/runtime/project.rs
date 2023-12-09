use super::nodes::snode;

snode!(
  project {
    lib: nodes::ONode::<'rt> = None
    exe: nodes::ONode::<'rt> = None
    root: nodes::ONode::<'rt> = None
  },
  struct: |n: &Self, b: &mut nodes::SBuilder<'rt>| {
    // b.entry("lib", n.lib);
    // b.entry("exe", n.exe);
    // b.entry("root", n.root);
  }
);

// pub struct Project<'rt> {
//     _struct: structs::map::Node<'rt>,
// }

// impl<'rt> Project<'rt> {
//   snode_getter!()
//     fn lib(&self) -> ONode<'rt> {
//         self._struct.get_entry("lib");
//     }
//     fn exe(&self) -> ONode<'rt> {}
//     fn root(&self) -> ONode<'rt> {}
// }

// impl<'rt> Node<'rt> for Project<'rt> {
//     fn to_any(&self) -> Any {
//         Any::Struct(Structure::Struct(structs::Node::New::<Builder>()))
//     }
// }

// pub struct Builder {}
// impl Builder {
//     // pub fn lib() -> lib::Builder {
//     //     todo!()
//     // }
//     // pub fn exe() -> exe::Builder {
//     //     todo!()
//     // }
// }

// impl<'rt> super::nodes::Builder<'rt, Project<'rt>> for Builder {
//     fn build(&self) -> Project<'rt> {
//         todo!()
//     }
// }
