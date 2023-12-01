use std::{any::TypeId, collections::HashMap, rc::Rc};

use crate::utils::log::{self, Color};

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

pub fn get_by_key<TType>(key: &str) -> &'static Rc<TType>
where
    TType: Parser + 'static,
{
    let result: &Rc<TType>;
    log::push_unique_key("PARSERS");
    log::info!(&["GET", "BY-KEY"], &format!("by key: {:?}", key));

    unsafe {
        let parser = _BY_KEY.as_ref().unwrap().get(key).unwrap();
        result = std::mem::transmute::<&Rc<dyn Parser>, &Rc<TType>>(parser);
    }

    log::pop_unique_key("PARSERS");

    return result;
}

pub fn get_by_type<TType>() -> &'static Rc<TType>
where
    TType: Parser + Sync + 'static,
{
    log::push_unique_key("PARSERS");

    let result: &'static Rc<TType>;
    log::info!(
        &["GET", "BY-TYPE"],
        &format!("by type: {:?}", std::any::type_name::<TType>()),
    );
    let type_id = TypeId::of::<TType>();
    log::info!(
        &["GET", "BY-TYPE-ID"],
        &format!("with type id: {:?}", type_id),
    );

    unsafe {
        let key = _BY_TYPE.as_ref().unwrap().get(&type_id).unwrap();
        result = get_by_key::<TType>(key);
    }

    log::pop_unique_key("PARSERS");

    return result;
}

pub(crate) fn init_all() {
    let all: Vec<Rc<dyn Parser>> = vec![
        Rc::new(named_entry::Parser {}),
        Rc::new(name::Parser {}),
        Rc::new(mutable_field_assigner::Parser {}),
        Rc::new(indent::Parser {}),
        Rc::new(dot_lookup::Parser {}),
        Rc::new(slash_lookup::Parser {}),
        Rc::new(escape_sequence::Parser {}),
        Rc::new(naked_text::Parser {}),
    ];

    init(all);
}

pub(crate) fn init(parsers: Vec<Rc<dyn Parser>>) {
    log::add_color("INIT", Color::Cyan);
    log::add_color("PARSERS", Color::Green);
    log::add_color(":START", Color::BrightMagenta);
    log::add_color(":END", Color::BrightMagenta);
    log::add_color(":NEW", Color::BrightMagenta);
    log::add_color(":EOF", Color::BrightMagenta);

    log::push_unique_key("INIT");
    log::push_unique_key("PARSERS");

    unsafe {
        match &mut _BY_KEY {
            Some(_) => {
                panic!("Parsers already initialized");
            }
            None => {
                log::info!(&[":START"], "Initializing parsers");
                log::push_key_div("-", Color::Green);

                _BY_KEY = Some(HashMap::new());
                for p in parsers {
                    //let parser = Box::new(p);
                    let key: &'static str = p.get_name();
                    let type_id: TypeId = p.get_type_id();
                    let type_name: &'static str = p.get_type_name();

                    log::push_key(key);
                    log::set_random_style(key);
                    log::push_key_div("-", Color::Green);
                    log::info!(&[":START"], "Initializing parser");
                    log::push_key_div("-", Color::Green);

                    log::info!(&["KEY"], key);
                    log::info!(&["TYPE"], &format!("{:?}: {:?}", type_name, type_id));

                    _BY_KEY.as_mut().unwrap().insert(key, p);

                    _BY_TYPE.get_or_insert(HashMap::new()).insert(type_id, key);

                    log::pop_key();
                    log::info!(&[":END"], "Initialized parser");
                    log::pop_key();
                    log::pop_key();
                }

                log::pop_key();
                log::info!(&[":END"], "Initialized parsers");
            }
        }
    }

    log::pop_unique_key("PARSERS");
    log::pop_unique_key("INIT");
}

static mut _BY_KEY: Option<HashMap<&'static str, Rc<dyn Parser>>> = None;
static mut _BY_TYPE: Option<HashMap<TypeId, &'static str>> = None;
