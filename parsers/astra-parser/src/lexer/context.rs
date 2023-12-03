pub struct Context {
    pub lang: Language,
}

pub enum Language {
    StruX,
    ProX,
    BloX,
}

impl Context {
    pub fn new_empty() -> Context {
        Context {
            lang: Language::StruX,
        }
    }

    pub fn new_for(lang: Language) -> Context {
        Context { lang }
    }
}
