use std::collections::HashMap;

use self::node::Srs;

pub mod node;
pub mod project;

pub struct Runtime {
    pub root: Srs<node::Entry>,
    pub fs: FileSystem,
    pub env: HashMap<String, String>,
    pub args: Vec<String>,
}

impl Runtime {
    #[allow(non_snake_case)]
    pub fn New() -> Self {
        Runtime {
            env: HashMap::new(),
            args: Vec::new(),
            root: node::Entry::Root(),
            fs: FileSystem {
                root: Directory {
                    name: "/".to_string(),
                },
            },
        }
    }
}

pub struct FileSystem {
    pub root: Directory,
}

pub struct Directory {
    pub name: String,
}
