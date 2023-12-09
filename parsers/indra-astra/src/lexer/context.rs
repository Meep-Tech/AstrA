use super::fs::{get_lang, get_type, File};
pub enum Language {
    StruX,
    ProX,
    BloX,
}

pub struct Context {
    pub lang: Language,
    pub file: Option<File>,
}

impl Context {
    pub fn new_empty() -> Context {
        Context {
            lang: Language::StruX,
            file: None,
        }
    }

    pub fn new_for(lang: Language) -> Context {
        Context { lang, file: None }
    }

    pub fn new_from(file: File) -> Context {
        Context {
            lang: get_lang(&file.kind),
            file: Some(file),
        }
    }

    pub fn new_from_file(name: &str) -> Context {
        let file = File {
            path: name.to_string(),
            name: name.to_string(),
            kind: get_type(name),
        };

        Context {
            lang: get_lang(&file.kind),
            file: Some(file),
        }
    }

    pub fn new_from_path(path: &str) -> Context {
        let name = path.split("/").last().unwrap_or(path);
        let file = File {
            path: path.to_string(),
            name: name.to_string(),
            kind: get_type(name),
        };

        Context {
            lang: get_lang(&file.kind),
            file: Some(file),
        }
    }
}
