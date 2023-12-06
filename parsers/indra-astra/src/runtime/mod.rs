use std::{collections::HashMap, rc::Rc};

use self::nodes::Node;

pub struct Runtime<'rt> {
    pub root: Rc<dyn Node<'rt>>,
    pub fs: FileSystem,
    pub env: HashMap<String, String>,
    pub args: Vec<String>,
}

impl<'rt> Runtime<'rt> {}

pub struct FileSystem {
    pub root: Directory,
}

pub struct Directory {
    pub name: String,
}

pub mod nodes;
