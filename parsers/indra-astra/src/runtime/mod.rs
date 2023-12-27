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

    pub fn root(&mut self) -> &Src<Entry> {
        &self.root
    }

    pub fn env(&mut self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn args(&mut self) -> &Vec<String> {
        &self.args
    }

    pub fn fs(&mut self) -> &FileSystem<'rt> {
        &self.fs
    }

    // pub fn global(&mut self) -> &mut Structure {
    //     let root_source = &mut self.root();
    //     let mut root = root_source.get(&mut self);
    //     let root_entry = Entry::Unwrap(root.borrow_mut());
    //     let global_source = root_entry.value();
    //     let mut global = global_source.get(self);
    //     let global_struct = Structure::Unwrap(global.borrow_mut());

    //     global_struct
    // }

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
