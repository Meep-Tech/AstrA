pub mod colors;
pub mod context;
pub mod cursor;
pub mod fs;
pub mod indents;
pub mod results;
pub mod tokens;

use crate::utils::log::{self};
use cursor::Cursor;
use meep_tech_log::{self as Log, Styleable};
use results::{builder::Builder, end::End, parsed::Parsed, r#match::Match};
use std::{any::TypeId, collections::HashMap, rc::Rc};

#[cfg(feature = "verbose")]
use crate::utils::log::Styleable;

pub trait Type: Sync {
    // #region Static
    // #region Get
    #[allow(non_snake_case)]
    fn Instance() -> &'static Rc<Self>
    where
        Self: Sync + 'static + Sized,
    {
        get_by_type::<Self>()
    }

    #[allow(non_snake_case)]
    fn Get() -> &'static Rc<dyn Type>
    where
        Self: Sync + 'static + Sized,
    {
        get_for_type::<Self>()
    }

    // #endregion
    // #region Parse Methods

    #[allow(non_snake_case)]
    fn Parse(input: &str) -> Parsed
    where
        Self: Sync + 'static + Sized,
    {
        Self::Instance().parse(input)
    }

    #[allow(non_snake_case)]
    fn Parse_At(cursor: &mut Cursor) -> Parsed
    where
        Self: Sync + 'static + Sized,
    {
        Self::Instance().parse_at(cursor)
    }

    #[allow(non_snake_case)]
    fn Try_Parse(input: &str) -> Option<Match>
    where
        Self: Sync + 'static + Sized,
    {
        Self::Instance().try_parse(input)
    }

    #[allow(non_snake_case)]
    fn Try_Parse_At(cursor: &mut Cursor) -> Option<Match>
    where
        Self: Sync + 'static + Sized,
    {
        Self::Instance().try_parse_at(cursor)
    }

    #[allow(non_snake_case)]
    fn Parse_Opt(input: &str) -> Parsed
    where
        Self: Sync + 'static + Sized,
    {
        return Self::Instance().parse_opt(input);
    }

    #[allow(non_snake_case)]
    fn Parse_Opt_At(cursor: &mut Cursor) -> Parsed
    where
        Self: Sync + 'static + Sized,
    {
        return Self::Instance().parse_opt_at(cursor);
    }
    // #endregion
    // #endregion

    // #region Data
    fn name(&self) -> &'static str;

    fn rule(&self, start: &mut Cursor) -> End;

    fn type_id(&self) -> TypeId
    where
        Self: 'static,
    {
        std::any::TypeId::of::<Self>()
    }

    fn type_name(&self) -> &'static str
    where
        Self: 'static,
    {
        std::any::type_name::<Self>()
    }
    // #endregion

    // #region Parser Methods

    fn parse(&self, input: &str) -> Parsed {
        let mut cursor = Cursor::new(input);
        self.parse_at(&mut cursor)
    }

    fn parse_at(&self, cursor: &mut Cursor) -> Parsed {
        self.parse_with_options_at(cursor, false, false)
    }

    fn parse_opt(&self, input: &str) -> Parsed {
        self.parse_with_options(input, true, true)
    }

    fn parse_opt_or_skip(&self, input: &str) -> Parsed {
        self.parse_with_options(input, false, true)
    }

    fn parse_opt_or_skip_at(&self, cursor: &mut Cursor) -> Parsed {
        self.parse_with_options_at(cursor, false, true)
    }

    fn parse_opt_at(&self, cursor: &mut Cursor) -> Parsed {
        self.parse_with_options_at(cursor, true, true)
    }

    fn try_parse_at(&self, cursor: &mut Cursor) -> Option<Match> {
        match self.parse_opt_at(cursor) {
            Parsed::Pass(token) => Some(token),
            _ => None,
        }
    }

    fn try_parse(&self, input: &str) -> Option<Match> {
        match self.parse_opt(input) {
            Parsed::Pass(token) => Some(token),
            _ => None,
        }
    }

    fn parse_with_options(&self, input: &str, optional: bool, ignored: bool) -> Parsed {
        let mut cursor = Cursor::new(input);
        self.parse_with_options_at(&mut cursor, optional, ignored)
    }

    fn parse_with_options_at(&self, cursor: &mut Cursor, optional: bool, ignored: bool) -> Parsed {
        log::color!("PARSE", log::Color::Green);
        log::push_unique!("PARSE");
        log::push!(self.name());
        log::push_div!(":", log::Color::Green);
        log::info!(&[":START"], &format!("@ {}", cursor.pos));

        let start = if optional { cursor.save() } else { cursor.pos };

        let result = match self.rule(cursor) {
            End::Match(token) => {
                let token = token
                    .assure_name(self.name())
                    .build(start, cursor.prev_pos());
                log::info!(
                    &[":END", "MATCH"],
                    &format!("@ {} = {:#?}", cursor.prev_pos(), token).color(log::Color::Green),
                );
                Parsed::Pass(token)
            }
            End::Fail(error) => {
                let error = error
                    .tag(self.name())
                    .assure_name(self.name())
                    .build(start, cursor.prev_pos());

                if optional {
                    cursor.restore();
                }

                if ignored {
                    if log::IS_VV {
                        log::info!(
                            &[
                                ":END",
                                &"IGNORED"
                                    .effect(log::Effect::Strikethrough)
                                    .color(log::Color::BrightBlack)
                            ],
                            &format!("@ {} = {:#?}", cursor.prev_pos(), err)
                                .effect(log::Effect::Strikethrough)
                                .color(log::Color::BrightBlack),
                        );
                    } else {
                        log::info!(
                            &[
                                ":END",
                                &"IGNORED"
                                    .color(log::Color::BrightBlack)
                                    .effect(log::Effect::Strikethrough)
                            ],
                            &format!("@ {}", cursor.prev_pos()).color(log::Color::BrightBlack),
                        );
                    }
                } else {
                    log::info!(
                        &[":END", "FAIL"],
                        &format!("@ {} = {:#?}", cursor.prev_pos(), err)
                            .color(log::Color::Red)
                            .effect(log::Effect::Underline),
                    );
                }

                Parsed::Fail(error)
            }
            End::None => {
                if optional {
                    cursor.restore();
                }

                log::info!(
                    &[":END", &"NONE".color(log::Color::BrightBlack)],
                    &format!("@ {}", cursor.prev_pos())
                );
                Parsed::Fail(None)
            }
        };

        log::pop!();
        log::pop!();
        log::pop_unique!("PARSE");

        result
    }

    // #endregion

    // #region Tests

    fn get_tests(&self) -> Vec<crate::tests::parser::tokens::tests::Test> {
        Vec::new()
    }

    // #endregion
}

// #region Get
static mut _BY_KEY: Option<HashMap<&'static str, Rc<dyn Type>>> = None;
static mut _BY_TYPE: Option<HashMap<TypeId, &'static str>> = None;

pub fn get_by_key<TType>(key: &str) -> &'static Rc<TType>
where
    TType: Type + 'static,
{
    let result: &Rc<TType>;
    log::push_unique!("PARSERS");
    log::vvv!(&["GET", "BY-KEY"], &format!("by key: {:?}", key));

    unsafe {
        let parser = _BY_KEY.as_ref().unwrap().get(key).unwrap();
        result = std::mem::transmute::<&Rc<dyn Type>, &Rc<TType>>(parser);
    }

    log::pop_unique!("PARSERS");

    return result;
}

pub fn get_for_key(key: &str) -> &'static Rc<dyn Type> {
    let result: &'static Rc<dyn Type>;
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
    TType: Type + Sync + 'static,
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
                    &"...Did you add it to the all parsers list".color(Log::Color::Yellow)
                )
            });
        result = get_by_key::<TType>(key);
    }

    log::pop_unique!("PARSERS");

    return result;
}

pub fn get_for_type<TType>() -> &'static Rc<dyn Type>
where
    TType: Type + Sync + 'static,
{
    log::push_unique!("PARSERS");

    let result: &'static Rc<dyn Type>;
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
                    &"...Did you add it to the all parsers list".color(Log::Color::Yellow)
                )
            });
        result = get_for_key(key);
    }

    log::pop_unique!("PARSERS");

    return result;
}

pub fn get_all() -> &'static HashMap<&'static str, Rc<dyn Type>> {
    let result: &'static HashMap<&'static str, Rc<dyn Type>>;
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

static mut _INITIALIZED: bool = false;
pub fn init_all() {
    if unsafe { _INITIALIZED } {
        panic!("Global AstrA Token Parsers already initialized!");
    }
    let all: Vec<Rc<dyn Type>> = vec![
        Rc::new(tokens::source::Token {}),
        Rc::new(tokens::source::file::Token {}),
        Rc::new(tokens::source::file::data::Token {}),
        Rc::new(tokens::source::file::markup::Token {}),
        Rc::new(tokens::source::file::mote::Token {}),
        Rc::new(tokens::source::file::prox::Token {}),
        Rc::new(tokens::source::file::r#trait::Token {}),
        Rc::new(tokens::attribute::Token {}),
        Rc::new(tokens::attribute::tag::Token {}),
        Rc::new(tokens::statement::Token {}),
        Rc::new(tokens::statement::assignment::Token {}),
        Rc::new(tokens::statement::assignment::entry::Token {}),
        Rc::new(tokens::statement::assignment::entry::named_entry::Token {}),
        Rc::new(tokens::statement::expression::Token {}),
        Rc::new(tokens::statement::expression::attribute_expression::Token {}),
        Rc::new(tokens::statement::expression::entry_expression::Token {}),
        Rc::new(tokens::statement::expression::invocation::Token {}),
        Rc::new(tokens::statement::expression::invocation::identifier::Token {}),
        Rc::new(tokens::statement::expression::invocation::identifier::key::Token {}),
        Rc::new(tokens::statement::expression::invocation::identifier::key::name::Token {}),
        Rc::new(tokens::statement::expression::invocation::identifier::lookup::Token {}),
        Rc::new(
            tokens::statement::expression::invocation::identifier::lookup::dot_lookup::Token {},
        ),
        Rc::new(
            tokens::statement::expression::invocation::identifier::lookup::slash_lookup::Token {},
        ),
        Rc::new(tokens::statement::expression::literal::Token {}),
        Rc::new(tokens::statement::expression::literal::escape::Token {}),
        Rc::new(tokens::statement::expression::literal::escape::escape_sequence::Token {}),
        Rc::new(tokens::statement::expression::literal::escape::newline_escape::Token {}),
        Rc::new(tokens::statement::expression::literal::escape::tab_escape::Token {}),
        Rc::new(tokens::statement::expression::literal::escape::backtick_escape::Token {}),
        Rc::new(tokens::statement::expression::literal::escape::quote_escape::Token {}),
        Rc::new(tokens::statement::expression::literal::escape::quote_escape::double::Token {}),
        Rc::new(tokens::statement::expression::literal::escape::quote_escape::single::Token {}),
        Rc::new(tokens::statement::expression::literal::markup::Token {}),
        Rc::new(tokens::statement::expression::literal::markup::element::Token {}),
        Rc::new(tokens::statement::expression::literal::markup::element::text::Token {}),
        Rc::new(tokens::statement::expression::literal::primitive::Token {}),
        Rc::new(tokens::statement::expression::literal::primitive::string::Token {}),
        Rc::new(tokens::statement::expression::literal::primitive::string::simple_string::Token {}),
        Rc::new(tokens::statement::expression::literal::structure::Token {}),
        Rc::new(tokens::statement::expression::literal::structure::tree::Token {}),
        Rc::new(tokens::statement::expression::literal::structure::closure::Token {}),
        Rc::new(tokens::statement::branch::Token {}),
        Rc::new(tokens::symbol::Token {}),
        Rc::new(tokens::symbol::operator::Token {}),
        Rc::new(tokens::symbol::operator::assigner::Token {}),
        Rc::new(tokens::symbol::operator::assigner::mutable_field_assigner::Token {}),
        Rc::new(tokens::whitespace::Token {}),
        Rc::new(tokens::whitespace::indent::Token {}),
        Rc::new(tokens::whitespace::indent::increase::Token {}),
        Rc::new(tokens::whitespace::indent::decrease::Token {}),
        Rc::new(tokens::whitespace::indent::current::Token {}),
    ];

    init(all);
}

pub(crate) fn init(parsers: Vec<Rc<dyn Type>>) {
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
