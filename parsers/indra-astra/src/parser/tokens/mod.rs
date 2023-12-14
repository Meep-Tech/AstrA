#![allow(unused_imports)]
use std::{any::TypeId, collections::HashMap, rc::Rc};

use crate::utils::log::{self, Styleable};

#[cfg(feature = "log")]
use crate::utils::log::Color;

use self::{
    statement::expression::invocation::identifier::key::name,
    symbol::operator::assigner::mutable_field_assigner, whitespace::indent,
};

use super::Parser;

pub mod attribute;
pub mod source;
pub mod statement;
pub mod symbol;
pub mod whitespace;

pub type ParserType = dyn Parser + Sync + 'static;
pub type Instance<TParser = ParserType> = TParser;

macro_rules! token {
    ($key:ident => $rule:expr) => {
        pub const KEY: &str = stringify!($key);

        pub struct Parser;
        impl crate::parser::Parser for Parser {
            fn name(&self) -> &'static str {
                crate::parser::tokens::imports!();
                &KEY
            }

            fn rule(
                &self,
                cursor: &mut crate::parser::cursor::Cursor,
            ) -> crate::parser::results::end::End {
                crate::parser::tokens::imports!();
                let rule = |cursor: &mut Cursor| -> End { $rule(cursor) };
                rule(cursor)
            }
        }
    };

    ($key:ident => $rule:expr, tests: $($tests:expr)*) => {
        pub const KEY: &str = stringify!($key);

        pub struct Parser;
        impl crate::parser::Parser for Parser {
            fn name(&self) -> &'static str {
                crate::parser::tokens::imports!();
                &KEY
            }

            fn rule(
                &self,
                cursor: &mut crate::parser::cursor::Cursor,
            ) -> crate::parser::results::end::End {
                crate::parser::tokens::imports!();
                let rule = |cursor: &mut Cursor| -> End { $rule(cursor) };
                rule(cursor)
            }

            fn get_tests(&self) -> Vec<crate::tests::parser::tokens::tests::Test> {
                crate::parser::tokens::imports!();
                use crate::tests::parser::tokens::tests::{pattern, unit, Mock, Mockable, Test};
                let tests: Vec<crate::tests::parser::tokens::tests::Test> = vec![$($tests,)*];

                tests
            }
        }
    };
}
pub(crate) use token;

macro_rules! splay_mods {
    ($key:ident: [$($parsers:ident $(,)?)*]) => {
        $(pub mod $parsers;)*

        crate::parser::tokens::splay! {
            $key: [$($parsers,)*]
        }
    };

    (#testable, $key:ident: [$($parsers:ident $(,)?)*]) => {

        crate::parser::tokens::splay! {
            #testable,
            $key: [$($parsers,)*]
        }
    };
}
pub(crate) use splay_mods;

macro_rules! splay {
    ($key:ident: [$($parsers:ident $(,)?)*]) => {
        crate::parser::tokens::token! {
            $key => |cursor: &mut Cursor| {
                End::Splay(
                    &KEY,
                    cursor,
                    &[
                        $(
                            &$parsers::Parser::Get(),
                        )*
                    ]
                )
            }
        }
    };

    (#testable, $key:ident: [$($parsers:ident $(,)?)*]) => {
        crate::parser::tokens::token! {
            #testable,
            $key => |cursor: &mut Cursor| {
                End::Splay(
                    &KEY,
                    cursor,
                    &[
                        $(
                            &$parsers::Parser::Get(),
                        )*
                    ]
                )
            }
        }
    };
}
pub(crate) use splay;

// #region Get

pub fn get_by_key<TType>(key: &str) -> &'static Rc<TType>
where
    TType: Parser + 'static,
{
    let result: &Rc<TType>;
    log::push_unique!("PARSERS");
    log::vvv!(&["GET", "BY-KEY"], &format!("by key: {:?}", key));

    unsafe {
        let parser = _BY_KEY.as_ref().unwrap().get(key).unwrap();
        result = std::mem::transmute::<&Rc<dyn Parser>, &Rc<TType>>(parser);
    }

    log::pop_unique!("PARSERS");

    return result;
}

pub fn get_for_key(key: &str) -> &'static Rc<dyn Parser> {
    let result: &'static Rc<dyn Parser>;
    log::push_unique!("PARSERS");
    log::vvv!(&["GET", "FOR-KEY"], &format!("for key: {:?}", key));

    unsafe {
        result = _BY_KEY.as_ref().unwrap().get(key).unwrap();
    }

    log::pop_unique!("PARSERS");

    return result;
}

pub fn get_by_type<TType>() -> &'static Rc<TType>
where
    TType: Parser + Sync + 'static,
{
    log::push_unique!("PARSERS");

    let result: &'static Rc<TType>;
    log::vvv!(
        &["GET", "BY-TYPE"],
        &format!("by type: {:?}", std::any::type_name::<TType>()),
    );
    let type_id = TypeId::of::<TType>();
    log::vvv!(
        &["GET", "BY-TYPE-ID"],
        &format!("with type id: {:?}", type_id),
    );

    unsafe {
        let key = _BY_TYPE
            .as_ref()
            .unwrap_or_else(|| panic!("Parsers not initialized"))
            .get(&type_id)
            .unwrap_or_else(|| {
                panic!(
                    "Parser key not found for type: {:?} with id: {:?}.\n\t {}?",
                    std::any::type_name::<TType>(),
                    type_id,
                    &"...Did you add it to the all parsers list".color(log::Color::Yellow)
                )
            });
        result = get_by_key::<TType>(key);
    }

    log::pop_unique!("PARSERS");

    return result;
}

pub fn get_for_type<TType>() -> &'static Rc<dyn Parser>
where
    TType: Parser + Sync + 'static,
{
    log::push_unique!("PARSERS");

    let result: &'static Rc<dyn Parser>;
    log::vvv!(
        &["GET", "FOR-TYPE"],
        &format!("for type: {:?}", std::any::type_name::<TType>()),
    );
    log::vvv!(
        &["GET", "FOR-TYPE-ID"],
        &format!("with type id: {:?}", TypeId::of::<TType>()),
    );

    unsafe {
        let key = _BY_TYPE
            .as_ref()
            .unwrap_or_else(|| panic!("Parsers not initialized"))
            .get(&TypeId::of::<TType>())
            .unwrap_or_else(|| {
                panic!(
                    "Parser key not found for type: {:?} with id: {:?}.\n\t {}?",
                    std::any::type_name::<TType>(),
                    TypeId::of::<TType>(),
                    &"...Did you add it to the all parsers list".color(log::Color::Yellow)
                )
            });
        result = get_for_key(key);
    }

    log::pop_unique!("PARSERS");

    return result;
}

pub fn get_all() -> &'static HashMap<&'static str, Rc<dyn Parser>> {
    let result: &'static HashMap<&'static str, Rc<dyn Parser>>;
    log::push_unique!("PARSERS");
    log::vvv!(&["GET", "ALL"], &format!("borrowing all parsers."));

    unsafe {
        result = _BY_KEY.as_ref().unwrap();
    }

    log::pop_unique!("PARSERS");

    return result;
}

// #endregion

// #region Init

pub(crate) fn init_all() {
    let all: Vec<Rc<dyn Parser>> = vec![
        Rc::new(source::Parser {}),
        Rc::new(source::file::Parser {}),
        Rc::new(source::file::data::Parser {}),
        Rc::new(source::file::markup::Parser {}),
        Rc::new(source::file::mote::Parser {}),
        Rc::new(source::file::prox::Parser {}),
        Rc::new(source::file::r#trait::Parser {}),
        Rc::new(attribute::Parser {}),
        Rc::new(attribute::tag::Parser {}),
        Rc::new(statement::Parser {}),
        Rc::new(statement::assignment::Parser {}),
        Rc::new(statement::assignment::entry::Parser {}),
        Rc::new(statement::assignment::entry::named_entry::Parser {}),
        Rc::new(statement::expression::Parser {}),
        Rc::new(statement::expression::attribute_expression::Parser {}),
        Rc::new(statement::expression::entry_expression::Parser {}),
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
        Rc::new(statement::expression::literal::structure::closure::Parser {}),
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
    log::color!("INIT", Color::Cyan);
    log::color!("PARSERS", Color::Green);
    log::color!(":START", Color::BrightMagenta);
    log::color!(":END", Color::BrightMagenta);
    log::color!(":NEW", Color::BrightMagenta);
    log::color!(":EOF", Color::BrightMagenta);

    log::push_unique!("INIT");
    log::push_unique!("PARSERS");

    unsafe {
        match &mut _BY_KEY {
            Some(_) => {
                panic!("Parsers already initialized");
            }
            None => {
                log::info!(&[":START"], "Initializing parsers");
                log::push_div!("-", Color::Green);

                _BY_KEY = Some(HashMap::new());
                for p in parsers {
                    //let parser = Box::new(p);
                    let key: &'static str = p.name();
                    let type_id: TypeId = p.type_id();

                    log::push!(key);
                    log::random_style!(key);
                    log::push_div!("-", Color::Green);
                    log::info!(&[":START"], "Initializing parser");
                    log::push_div!("-", Color::Green);

                    log::vv!(&["KEY"], key);
                    log::vv!(&["TYPE"], &format!("{:?}: {:?}", p.type_name(), type_id));

                    _BY_KEY.as_mut().unwrap().insert(key, p);

                    _BY_TYPE.get_or_insert(HashMap::new()).insert(type_id, key);

                    log::pop!();
                    log::info!(&[":END"], "Initialized parser");
                    log::pop!();
                    log::pop!();
                }

                log::pop!();
                log::info!(&[":END"], "Initialized parsers");
            }
        }
    }

    log::pop_unique!("PARSERS");
    log::pop_unique!("INIT");
}

// #endregion

// #region Internal
macro_rules! imports {
    () => {
        use crate::parser::{
            context::{self, Context, Language},
            cursor::Cursor,
            fs,
            results::{
                builder::Builder, end::End, error::Error, node::Node, parsed::Parsed, token::Token,
            },
            tokens::{self, source, statement, symbol, whitespace},
            Parser as _,
        };
    };
}
pub(crate) use imports;

static mut _BY_KEY: Option<HashMap<&'static str, Rc<dyn Parser>>> = None;
static mut _BY_TYPE: Option<HashMap<TypeId, &'static str>> = None;
// #endregion
