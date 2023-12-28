use std::path::Path;

use super::{nodes::Struct, rfr::Rfr, Runtime};

pub struct Scope<'rt> {
    rt: &'rt Runtime<'rt>,
    own: Rfr<Struct>,
    path: &'rt Path,
}

impl<'rt> Scope<'rt> {
    #[allow(non_snake_case)]
    pub fn Root(rt: &'rt Runtime<'rt>) -> Self {
        Self {
            rt,
            own: rt.root.get_value(rt).as_struct(),
            path: rt.fs.source,
        }
    }
}
