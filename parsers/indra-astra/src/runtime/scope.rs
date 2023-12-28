use std::{borrow::Borrow, path::Path};

use super::{cell::Rfr, nodes::Structure, Runtime};

pub struct Scope<'rt> {
    rt: &'rt Runtime<'rt>,
    own: Rfr<Structure>,
    path: &'rt Path,
}

impl<'rt> Scope<'rt> {
    #[allow(non_snake_case)]
    pub fn Root(rt: &'rt Runtime<'rt>) -> Self {
        Self {
            rt,
            own: Rfr::<Structure>::To(rt.root.get_value(rt).to_structure(rt).borrow()),
            path: rt.fs.source,
        }
    }
}
