use std::{
    any::{Any, TypeId},
    borrow::BorrowMut,
    collections::HashMap,
    rc::Rc,
};

use crate::utils::log;

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

pub type ParserType = dyn Parser + Sync + 'static;
pub type Instance<TParser = ParserType> = TParser;

pub(crate) static PARSERS: [&ParserType; 8] = [
    &named_entry::PARSER,
    &name::PARSER,
    &mutable_field_assigner::PARSER,
    &indent::PARSER,
    &dot_lookup::PARSER,
    &slash_lookup::PARSER,
    &escape_sequence::PARSER,
    &naked_text::PARSER,
];

pub fn get_by_key<TType>(key: &str) -> Rc<TType>
where
    TType: Parser + Sync + 'static,
{
    let result: Option<Rc<TType>>;
    log::push_unique_key("PARSERS");
    log::info(&["GET", "BY-KEY"], &format!("by key: {:?}", key));

    unsafe {
        let instance = &_PARSERS_BY_KEY.as_ref().unwrap().get(key);
        match instance {
            Some(mut instance) => {
                log::info(
                    &["GET", "BY-KEY", "FOUND"],
                    &format!("{:?}: {:?}", key, instance),
                );

                let rc = instance.borrow_mut();

                let downcast = rc.clone().downcast::<TType>();
                result = match downcast {
                    Ok(downcast) => Some(downcast.to_owned()),
                    Err(error) => {
                        panic!(
                            "Failed to downcast parser by key: {:?}, from type: {:?}, to type: {:?}. Error: {:?}",
                            key,
                            std::any::type_name_of_val(rc.borrow_mut()),
                            std::any::type_name::<TType>(),
                            error.to_owned()
                        );
                    }
                }
            }
            None => {
                panic!("Failed to get parser by key: {:?}", key);
            }
        }
    }

    log::pop_unique_key("PARSERS");

    return result.unwrap();
}

pub fn get_by_type<TType>() -> Rc<TType>
where
    TType: Parser + Sync + 'static,
{
    log::push_unique_key("PARSERS");

    let result: Option<Rc<TType>>;
    log::info(
        &["GET", "BY-TYPE"],
        &format!("by type: {:?}", std::any::type_name::<TType>()),
    );
    let type_id = TypeId::of::<TType>();
    log::info(
        &["GET", "BY-TYPE-ID"],
        &format!("with type id: {:?}", type_id),
    );

    unsafe {
        let key = _PARSERS_BY_TYPE.as_ref().unwrap().get(&type_id).unwrap();
        result = Some(get_by_key::<TType>(key));
    }

    log::pop_unique_key("PARSERS");

    return result.unwrap();
}

pub(crate) fn init_all() {
    init(&PARSERS);
}

pub(crate) fn init(parsers: &'static [&'_ ParserType]) {
    log::push_unique_key("INIT");
    log::push_unique_key("PARSERS");

    unsafe {
        match &mut _PARSERS_BY_KEY {
            Some(_) => {
                panic!("Parsers already initialized");
            }
            None => {
                log::info(&[":START"], "Initializing parsers");
                log::push_key_div("-", &log::Color::Green);

                _PARSERS_BY_KEY = Some(HashMap::new());
                for parser in parsers {
                    let key: &'static str = parser.get_name();
                    let type_id: TypeId = parser.get_type_id();

                    log::push_key(key);
                    log::push_key_div("-", &log::Color::Green);
                    log::info(&[":START"], "Initializing parser");
                    log::push_key_div("-", &log::Color::Green);

                    log::info(&["KEY"], key);

                    _PARSERS_BY_KEY
                        .as_mut()
                        .unwrap()
                        .insert(key, Rc::new(parser));

                    log::info(
                        &["TYPE"],
                        &format!("{:?}: {:?}", std::any::type_name_of_val(parser), type_id),
                    );

                    _PARSERS_BY_TYPE
                        .get_or_insert(HashMap::new())
                        .insert(type_id, key);

                    log::pop_key();
                    log::info(&[":END"], "Initialized parser");
                    log::pop_key();
                    log::pop_key();
                }

                log::pop_key();
                log::info(&[":END"], "Initialized parsers");
            }
        }
    }

    log::pop_unique_key("PARSERS");
    log::pop_unique_key("INIT");
}

static mut _PARSERS_BY_KEY: Option<HashMap<&'static str, Rc<Box<dyn Parser>>>> = None;
static mut _PARSERS_BY_TYPE: Option<HashMap<TypeId, &'static str>> = None;
