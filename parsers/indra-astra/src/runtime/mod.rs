use slotmap::{DefaultKey, SlotMap};
use std::{collections::HashMap, path::Path, sync::Mutex};

use self::{
    cell::{Id, Src},
    node::{Any, Entry},
};

pub mod cell;
pub mod node;

pub struct Runtime<'rt> {
    root: Src<Entry>,
    fs: FileSystem<'rt>,
    env: HashMap<String, String>,
    args: Vec<String>,

    __: SlotMap<DefaultKey, Mutex<Any>>,
}

impl<'rt> Runtime<'rt> {
    #[allow(non_snake_case)]
    pub fn New(source: &'rt Path) -> Self {
        let mut rt = Runtime::<'rt> {
            __: SlotMap::with_key(),
            env: HashMap::new(),
            args: Vec::new(),
            root: Src::Empty(),
            fs: FileSystem::<'rt> {
                source,
                root: Directory {
                    name: "/".to_string(),
                },
            },
        };

        rt.root = Entry::Root(&mut rt);

        rt
    }

    #[allow(non_snake_case)]
    pub fn Init(source: &'rt Path) -> Self {
        let mut rt = Runtime::New(source);
        rt.init();
        rt.load();
        rt
    }

    #[allow(non_snake_case)]
    pub fn Load(source: &'rt Path) -> Self {
        let mut rt = Runtime::New(source);
        rt.load();
        rt
    }

    pub fn init(&mut self) {}

    pub fn load(&mut self) {}

    pub fn root(&self) -> &Src<Entry> {
        &self.root
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn fs(&self) -> &FileSystem<'rt> {
        &self.fs
    }

    // #region Internal
    fn _add_node(&mut self, node: Any) -> Id {
        self.__.insert(Mutex::new(node))
    }

    fn _get_node(&self, id: Id) -> &Mutex<Any> {
        self.__.get(id).unwrap()
    }
    // #endregion
}

pub struct FileSystem<'rt> {
    pub source: &'rt Path,
    pub root: Directory,
}

pub struct Directory {
    pub name: String,
}

fn moddify() {}

fn mutate() {}
