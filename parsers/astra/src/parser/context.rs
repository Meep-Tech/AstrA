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
    #[allow(non_snake_case)]
    pub fn New_Empty() -> Context {
        Context {
            lang: Language::StruX,
            file: None,
        }
    }

    #[allow(non_snake_case)]
    pub fn New_For(lang: Language) -> Context {
        Context { lang, file: None }
    }

    #[allow(non_snake_case)]
    pub fn New_From(file: File) -> Context {
        Context {
            lang: get_lang(&file.kind),
            file: Some(file),
        }
    }

    #[allow(non_snake_case)]
    pub fn New_From_File(name: &str) -> Context {
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

    #[allow(non_snake_case)]
    pub fn New_From_Path(path: &str) -> Context {
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
