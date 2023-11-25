use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
};

use super::parser::Parser;

pub mod dot_lookup;
pub mod escape_sequence;
pub mod indent;
pub mod mutable_field_assigner;
pub mod naked_text;
pub mod name;
/// named-entry
///   - key: name
///   - ?increase-indent | ?gap
///   - operator: assigner
///   - ?increase-indent | ?gap
///   - value: value
pub mod named_entry;
pub mod slash_lookup;

pub type Instance<TParser = dyn Parser + Sync>
where
    TParser: Parser + ?Sync,
= Rc<TParser>;

pub static PARSERS: [&(dyn Parser + Sync); 8] = [
    &named_entry::PARSER,
    &name::PARSER,
    &mutable_field_assigner::PARSER,
    &indent::PARSER,
    &dot_lookup::PARSER,
    &slash_lookup::PARSER,
    &escape_sequence::PARSER,
    &naked_text::PARSER,
];

pub fn get_by_key(key: &str) -> Instance {
    unsafe {
        _PARSERS_BY_KEY
            .as_ref()
            .unwrap()
            .get(key)
            .unwrap()
            .downcast_ref::<Instance>()
            .unwrap()
            .clone()
    }
}

pub fn instance<TType>() -> Instance<TType>
where
    TType: Parser + Sync + 'static,
{
    unsafe {
        let key = _PARSERS_BY_TYPE
            .as_ref()
            .unwrap()
            .get(&TypeId::of::<TType>())
            .unwrap();
        _PARSERS_BY_KEY
            .as_ref()
            .unwrap()
            .get(key)
            .unwrap()
            .downcast_ref::<Instance<TType>>()
            .unwrap()
            .clone()
    }
}

pub(crate) fn initalize_all() {
    init(&PARSERS);
}

pub(crate) fn init(parsers: &'static [&'_ (dyn Parser + Sync)]) {
    unsafe {
        match &mut _PARSERS_BY_KEY {
            Some(_) => {
                panic!("Parsers already initialized");
            }
            None => {
                _PARSERS_BY_KEY = Some(HashMap::new());

                for parser in parsers {
                    let key: &'static str = parser.get_name();
                    let type_id: TypeId = parser.type_id();
                    _PARSERS_BY_KEY
                        .as_mut()
                        .unwrap()
                        .insert(key, Rc::new(parser));
                    _PARSERS_BY_TYPE
                        .get_or_insert(HashMap::new())
                        .insert(type_id, key);
                }
            }
        }
    }
}

static mut _PARSERS_BY_KEY: Option<HashMap<&'static str, Rc<dyn Any>>> = None;
static mut _PARSERS_BY_TYPE: Option<HashMap<TypeId, &'static str>> = None;
