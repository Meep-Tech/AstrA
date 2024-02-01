use slotmap::SlotMap;
use std::{collections::HashMap, fs::File, io::Read, path::Path, sync::Mutex};

use crate::parser::{self, results::parsed::Parsed, Parser};

pub mod nodes;
pub mod rfr;
pub mod scope;

use self::{
    nodes::{Any, Entry},
    rfr::{RId, Rfr},
    scope::Scope,
};

pub struct Runtime<'rt> {
    root: Rfr<Entry>,
    fs: FileSystem<'rt>,
    env: HashMap<String, String>,
    args: Vec<String>,

    __: SlotMap<RId, Mutex<Any>>,
}

impl<'rt> Runtime<'rt> {
    /// Used to initialize an empty runtime.
    #[allow(non_snake_case)]
    pub(crate) fn Empty(source: &'rt Path) -> Self {
        let mut rt = Runtime::<'rt> {
            __: SlotMap::with_key(),
            env: HashMap::new(),
            args: Vec::new(),
            root: Rfr::Empty(),
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

    /// Used to initialize a runtime with the standard library.
    #[allow(non_snake_case)]
    pub(crate) fn Std(source: &'rt Path) -> Self {
        let mut rt = Runtime::Empty(source);
        rt._add_std_lib();
        rt
    }

    /// Used to initialize a runtime for a new project.
    #[allow(non_snake_case)]
    pub fn Init(source: &'rt Path) -> Self {
        let mut rt = Runtime::Std(source);
        rt._load_config();
        rt._init();
        rt._load();
        rt
    }

    /// Used to load a runtime for an existing project.
    #[allow(non_snake_case)]
    pub fn Load(source: &'rt Path) -> Self {
        let mut rt = Runtime::Std(source);
        rt._load_config();
        rt._load();
        rt
    }

    pub fn root(&self) -> &Rfr<Entry> {
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
    fn _add_node(&mut self, node: Any) -> RId {
        self.__.insert(Mutex::new(node))
    }

    fn _get_node(&self, id: RId) -> &Mutex<Any> {
        self.__.get(id).unwrap()
    }

    fn _find_prj_file_path(&self) -> Option<File> {
        let root = self.fs.source;
        let root_dir_name = root.file_name().unwrap().to_str().unwrap();

        let potential_dirs = vec!["/", "src", "prj", ".prj", ".", "()", root_dir_name];
        let potential_names = vec!["prj", ".prj", ".", "()", "", root_dir_name];
        let potential_exts = vec![
            "prj",
            "axa",
            "prj.axa",
            "bin.axa",
            "lib.axa",
            "bin.prj.axa",
            "lib.prj.axa",
            "prj.bin.axa",
            "prj.lib.axa",
        ];

        for dir in &potential_dirs {
            for name in &potential_names {
                for ext in &potential_exts {
                    let path = root.join(dir).join(name).with_extension(ext);
                    if path.exists() {
                        return Some(File::open(path).unwrap());
                    }
                }
            }
        }

        None
    }

    fn _load_config(&mut self) {
        let prj_file = self._find_prj_file_path();
        let mut prj_file_contents = String::new();
        if let Result::Err(err) = prj_file.unwrap().read_to_string(&mut prj_file_contents) {
            panic!("Failed to read project file: {}", err);
        }

        let prj_file_parse_result = parser::tokens::source::file::Parser::Parse(&prj_file_contents);

        match prj_file_parse_result {
            Parsed::Pass(prj_file_root_token) => {
                let prj_file_analysis_result =
                    nodes::prj::Analyze(&prj_file_root_token, Scope::Root(self));
            }
            Parsed::Fail(err) => {
                panic!("Failed to parse project file: {:?}", err);
            }
        }
    }

    pub fn _init(&mut self) {}

    pub fn _load(&mut self) {}

    fn _add_std_lib(&mut self) {}
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
