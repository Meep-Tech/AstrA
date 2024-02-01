use super::context::Language;

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub name: String,
    pub kind: Type,
}

impl File {
    #[allow(non_snake_case)]
    pub fn New(path: &str) -> File {
        let name = path.split("/").last().unwrap_or(path);
        let kind = get_type(name);

        File {
            path: path.to_string(),
            name: name.to_string(),
            kind,
        }
    }

    pub fn get_lang(&self) -> Language {
        get_lang(&self.kind)
    }

    pub fn get_extension(&self) -> String {
        get_extension(&self.kind)
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Unknown,
    AstrA,
    Mote, // Markdown Oriented Trait Entry/ MOdular-TExt
    Data(Data),
    Trait(Trait),
    Markup(Markup),
}

#[derive(Debug, Clone)]
pub enum Data {
    Data,
    Value,
    StruX(Struct),
}

#[derive(Debug, Clone)]
pub enum Struct {
    StruX,
    Group,
    Map,
    OrderedMap,
    Array,
    Set,
    Table,
    Dex,
}

#[derive(Debug, Clone)]
pub enum Trait {
    Trait,
    ProX,
    Prototype,
    Archetype,
    Enum,
}

#[derive(Debug, Clone)]
pub enum Markup {
    Markup,
    BloX,
    Component,
}

pub fn get_extension(file_type: &Type) -> String {
    let prefix = match file_type {
        Type::Unknown => "",
        Type::AstrA => "",
        Type::Trait(trt) => match trt {
            Trait::Trait => "trt",
            Trait::ProX => "prx",
            Trait::Prototype => "pro",
            Trait::Archetype => "arc",
            Trait::Enum => "enm",
        },
        Type::Markup(mup) => match mup {
            Markup::Markup => "mup",
            Markup::BloX => "blx",
            Markup::Component => "cmp",
        },
        Type::Data(data) => match data {
            Data::Data => "dta",
            Data::Value => "val",
            Data::StruX(strux) => match strux {
                Struct::StruX => "stx",
                Struct::Group => "grp",
                Struct::Map => "map",
                Struct::OrderedMap => "map.ord",
                Struct::Array => "arr",
                Struct::Set => "hsh",
                Struct::Table => "tbl",
                Struct::Dex => "dex",
            },
        },
        Type::Mote => "mote",
    };

    let suffix = match file_type {
        Type::Mote => "",
        _ => ".axa",
    };

    format!("{}{}", prefix, suffix)
}

pub fn get_type(extension: &str) -> Type {
    let mut to_find: &str = &extension.to_lowercase();
    if extension.ends_with(".axa") {
        to_find = &extension[0..extension.len() - 4];
    }

    match to_find {
        "trt" => Type::Trait(Trait::Trait),
        "prx" => Type::Trait(Trait::ProX),
        "pro" => Type::Trait(Trait::Prototype),
        "arc" => Type::Trait(Trait::Archetype),
        "enm" => Type::Trait(Trait::Enum),
        "mup" => Type::Markup(Markup::Markup),
        "blx" => Type::Markup(Markup::BloX),
        "cmp" => Type::Markup(Markup::Component),
        "dta" => Type::Data(Data::Data),
        "val" => Type::Data(Data::Value),
        "stx" => Type::Data(Data::StruX(Struct::StruX)),
        "grp" => Type::Data(Data::StruX(Struct::Group)),
        "map" => Type::Data(Data::StruX(Struct::Map)),
        "map.ord" => Type::Data(Data::StruX(Struct::OrderedMap)),
        "arr" => Type::Data(Data::StruX(Struct::Array)),
        "hsh" => Type::Data(Data::StruX(Struct::Set)),
        "set" => Type::Data(Data::StruX(Struct::Set)),
        "tbl" => Type::Data(Data::StruX(Struct::Table)),
        "dex" => Type::Data(Data::StruX(Struct::Dex)),

        "mote" => Type::Mote,

        _ => Type::Unknown,
    }
}

pub fn get_lang(file_type: &Type) -> Language {
    match file_type {
        Type::AstrA => Language::ProX,
        Type::Markup(_) => Language::BloX,
        Type::Trait(trait_file) => match trait_file {
            Trait::ProX => Language::ProX,
            _ => Language::StruX,
        },
        _ => Language::StruX,
    }
}
