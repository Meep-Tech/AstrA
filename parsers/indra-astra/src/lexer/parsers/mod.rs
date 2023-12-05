use std::{any::TypeId, collections::HashMap, rc::Rc};

use crate::{
    tests::lexer::parsers::test::Testable,
    utils::log::{self, Color},
};

use self::{
    statement::expression::invocation::identifier::key::name,
    symbol::operator::assigner::mutable_field_assigner, whitespace::indent,
};

use super::parser::Parser;

pub mod statement;
pub mod symbol;
pub mod whitespace;

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

pub fn get_for_key(key: &str) -> &'static Rc<dyn Parser> {
    let result: &'static Rc<dyn Parser>;
    log::push_unique_key("PARSERS");
    log::info!(&["GET", "FOR-KEY"], &format!("for key: {:?}", key));

    unsafe {
        result = _BY_KEY.as_ref().unwrap().get(key).unwrap();
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

pub fn get_for_type<TType>() -> &'static Rc<dyn Parser>
where
    TType: Parser + Sync + 'static,
{
    log::push_unique_key("PARSERS");

    let result: &'static Rc<dyn Parser>;
    log::info!(
        &["GET", "FOR-TYPE"],
        &format!("for type: {:?}", std::any::type_name::<TType>()),
    );
    log::info!(
        &["GET", "FOR-TYPE-ID"],
        &format!("with type id: {:?}", TypeId::of::<TType>()),
    );

    unsafe {
        let key = _BY_TYPE
            .as_ref()
            .unwrap()
            .get(&TypeId::of::<TType>())
            .unwrap();
        result = get_for_key(key);
    }

    log::pop_unique_key("PARSERS");

    return result;
}

pub fn get_tests_for(key: &str) -> &'static dyn Testable {
    let result: &'static dyn Testable;
    log::push_unique_key("PARSERS");
    log::info!(&["GET", "TESTS"], &format!("for key: {:?}", key));
    let parser = get_for_key(key);
    result = parser.as_tests().unwrap();

    log::pop_unique_key("PARSERS");

    return result;
}

pub(crate) fn init_all() {
    let all: Vec<Rc<dyn Parser>> = vec![
        Rc::new(statement::Parser {}),
        Rc::new(statement::assignment::Parser {}),
        Rc::new(statement::assignment::entry::Parser {}),
        Rc::new(statement::assignment::entry::named_entry::Parser {}),
        Rc::new(statement::expression::Parser {}),
        Rc::new(statement::expression::invocation::Parser {}),
        Rc::new(statement::expression::invocation::identifier::Parser {}),
        Rc::new(statement::expression::invocation::identifier::key::Parser {}),
        Rc::new(statement::expression::invocation::identifier::key::name::Parser {}),
        Rc::new(statement::expression::invocation::identifier::lookup::Parser {}),
        Rc::new(statement::expression::invocation::identifier::lookup::dot_lookup::Parser {}),
        Rc::new(statement::expression::invocation::identifier::lookup::slash_lookup::Parser {}),
        Rc::new(statement::expression::literal::Parser {}),
        Rc::new(statement::expression::literal::escape::Parser {}),
        Rc::new(statement::expression::literal::escape::escape_sequence::Parser {}),
        Rc::new(statement::expression::literal::escape::newline_escape::Parser {}),
        Rc::new(statement::expression::literal::escape::tab_escape::Parser {}),
        Rc::new(statement::expression::literal::escape::backtick_escape::Parser {}),
        Rc::new(statement::expression::literal::escape::quote_escape::Parser {}),
        Rc::new(statement::expression::literal::escape::quote_escape::double::Parser {}),
        Rc::new(statement::expression::literal::escape::quote_escape::single::Parser {}),
        Rc::new(statement::expression::literal::markup::Parser {}),
        Rc::new(statement::expression::literal::markup::element::Parser {}),
        Rc::new(statement::expression::literal::markup::element::text::Parser {}),
        Rc::new(statement::expression::literal::primitive::Parser {}),
        Rc::new(statement::expression::literal::primitive::string::Parser {}),
        Rc::new(statement::expression::literal::primitive::string::simple_string::Parser {}),
        Rc::new(statement::expression::literal::structure::Parser {}),
        Rc::new(statement::expression::literal::structure::tree::Parser {}),
        Rc::new(statement::branch::Parser {}),
        Rc::new(symbol::Parser {}),
        Rc::new(symbol::operator::Parser {}),
        Rc::new(symbol::operator::assigner::Parser {}),
        Rc::new(symbol::operator::assigner::mutable_field_assigner::Parser {}),
        Rc::new(whitespace::Parser {}),
        Rc::new(whitespace::indent::Parser {}),
        Rc::new(whitespace::indent::increase::Parser {}),
        Rc::new(whitespace::indent::decrease::Parser {}),
        Rc::new(whitespace::indent::current::Parser {}),
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
static mut _TESTABLE: Option<HashMap<&'static str, Rc<dyn Testable>>> = None;
