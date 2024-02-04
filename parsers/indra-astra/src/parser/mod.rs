pub mod colors;
pub mod context;
pub mod cursor;
pub mod fs;
pub mod indents;
pub mod results;
pub mod tokens;

use self::tokens::{attribute, expression};
use crate::{
    parser::tokens::{source, statement, symbol, whitespace},
    utils::log::{self},
};
use cursor::Cursor;
use results::{builder::Builder, end::End, parsed::Parsed, token::Token};
use std::sync::LazyLock;
use std::{any::TypeId, sync::Mutex};
use std::{cell::LazyCell, collections::HashMap};

#[cfg(feature = "log")]
use crate::utils::ansi::Color;
#[cfg(feature = "log")]
use crate::utils::ansi::Effect;
#[cfg(feature = "log")]
use crate::utils::ansi::Styleable;
#[cfg(feature = "log")]
use crate::utils::sexp::SExpressable;

// #region All
/// A hashmap of all parsers.
pub static ALL: LazyLock<HashMap<String, Box<dyn Parser>>> = LazyLock::new(|| get_each());

static mut _ALL: LazyCell<Mutex<HashMap<String, Box<dyn Parser>>>> =
    LazyCell::new(|| Mutex::new(HashMap::new()));

/// Used to initialize all parsers.
/// MUST be called before the static ALL collection above or any of the get functions below are used.
pub fn init_all() {
    log::color!("INIT", Color::Cyan);
    log::push_unique!("INIT");
    log::push_div!("::", Color::Cyan);
    log::info!(&["::START"], &"Initializing all parsers".color(Color::Cyan));
    add_r!(expression);
    add_r!(statement);
    add_r!(symbol);
    add_r!(whitespace);
    add_r!(attribute);
    add_r!(source);
    log::info!(
        &["::END"],
        &"Finished initializing all parsers".color(Color::Cyan)
    );
    log::pop!();
    log::pop_unique!("INIT");
}

/// Borrow the hashmap of all parsers.
pub fn get_all() -> &'static std::collections::HashMap<String, Box<dyn Parser>> {
    &ALL
}

/// Get a copy of the hashmap of all parsers.
pub fn get_each() -> std::collections::HashMap<String, Box<dyn Parser>> {
    let all: &std::collections::HashMap<String, Box<dyn Parser>> = unsafe { &_ALL.lock().unwrap() };
    let mut map = HashMap::new();
    for (key, value) in all {
        map.insert(key.to_string(), value.get());
    }

    map
}

/// Get all sub-parsers of the given parser; recursively.
pub fn get_recursive_subs(parser: &dyn Parser) -> Vec<Box<dyn Parser>> {
    let mut subs = Vec::new();
    for sub in parser.subs() {
        subs.push(sub.get());
        subs.extend(get_recursive_subs(&*sub.get()).into_iter());
    }
    subs
}

/// Gets a parser by name
pub fn get_by_name(name: &str) -> Option<Box<dyn Parser>> {
    get_all().get(name).map(|p| p.get())
}

macro_rules! add_r {
    ($i:ident) => {
        let lock = unsafe { _ALL.lock() };
        let mut all = lock.ok().unwrap();
        all.insert(stringify!($i).to_string(), Box::new($i::Parser));
        log::info!(
            &["ADD"],
            &format!("Added parser: {}", stringify!($i)).color(Color::Cyan)
        );
        all.extend(get_recursive_subs(&$i::Parser::Get()).into_iter().map(|p| {
            log::info!(
                &["ADD"],
                &format!("Added parser: {}", p.name()).color(Color::Cyan)
            );
            (p.name().to_string(), p.get())
        }));
        drop(all);
    };
}
pub(super) use add_r;
// #endregion

pub trait Parser: Sync + Send {
    // #region Static
    // #region Get
    #[allow(non_snake_case)]
    fn Get() -> Self
    where
        Self: Sized;
    // #endregion

    // #region Parse Methods
    #[allow(non_snake_case)]
    fn Parse(input: &str) -> Parsed
    where
        Self: Sync + 'static + Sized,
    {
        Self::Get().parse(input)
    }

    #[allow(non_snake_case)]
    fn Parse_At(cursor: &mut Cursor) -> Parsed
    where
        Self: Sync + 'static + Sized,
    {
        Self::Get().parse_at(cursor)
    }

    #[allow(non_snake_case)]
    fn Parse_Opt(input: &str) -> Parsed
    where
        Self: Sync + 'static + Sized,
    {
        return Self::Get().parse_opt(input);
    }

    #[allow(non_snake_case)]
    fn Parse_Opt_At(cursor: &mut Cursor) -> Parsed
    where
        Self: Sync + 'static + Sized,
    {
        return Self::Get().parse_opt_at(cursor);
    }

    #[allow(non_snake_case)]
    fn Try_Parse(input: &str) -> Option<Token>
    where
        Self: Sync + 'static + Sized,
    {
        Self::Get().try_parse(input)
    }

    #[allow(non_snake_case)]
    fn Try_Parse_At(cursor: &mut Cursor) -> Option<Token>
    where
        Self: Sync + 'static + Sized,
    {
        Self::Get().try_parse_at(cursor)
    }
    // #endregion
    // #endregion

    // #region Data
    fn name(&self) -> &'static str;

    fn tags(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn rule(&self, start: &mut Cursor) -> End;

    fn subs(&self) -> Vec<Box<dyn crate::parser::Parser>>;

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
    /// Attempt to parse the input (as required).
    /// - Will log fail messages.
    /// - Will NOT revert to the previous state on fail.
    fn parse(&self, input: &str) -> Parsed {
        let mut cursor = Cursor::New(input);
        self.parse_at(&mut cursor)
    }

    /// Attempt to parse (as required) using the cursor.
    /// - Will log fail messages.
    /// - Will NOT revert to the previous state on fail.
    fn parse_at(&self, cursor: &mut Cursor) -> Parsed {
        self.parse_with_options_at(cursor, false, false)
    }

    /// Attempt to parse the input as optional.
    /// - Will NOT log fail messages.
    /// - Will revert to the previous state on fail.
    fn parse_opt(&self, input: &str) -> Parsed {
        self.parse_with_options(input, true, true)
    }

    /// Attempt to parse as optional; using the cursor.
    /// - Will NOT log fail messages.
    /// - Will revert to the previous state on fail.
    fn parse_opt_at(&self, cursor: &mut Cursor) -> Parsed {
        self.parse_with_options_at(cursor, true, true)
    }

    /// Attempt to parse the input.
    /// - Will NOT log fail messages.
    /// - Will NOT revert to the previous state on fail.
    fn parse_opt_or_skip(&self, input: &str) -> Parsed {
        self.parse_with_options(input, false, true)
    }

    /// Attempt to parse using the cursor.
    /// - Will NOT log fail messages.
    /// - Will NOT revert to the previous state on fail.
    fn parse_opt_or_skip_at(&self, cursor: &mut Cursor) -> Parsed {
        self.parse_with_options_at(cursor, false, true)
    }

    /// Attempt to parse the input; discarding any fail messages/results.
    /// - Will NOT log fail messages.
    /// - Will revert to the previous state on fail.
    fn try_parse(&self, input: &str) -> Option<Token> {
        match self.parse_opt(input) {
            Parsed::Pass(token) => Some(token),
            _ => None,
        }
    }

    /// Attempt to parse using the cursor; discarding any fail messages/results.
    /// - Will NOT log fail messages.
    /// - Will revert to the previous state on fail.
    fn try_parse_at(&self, cursor: &mut Cursor) -> Option<Token> {
        match self.parse_opt_at(cursor) {
            Parsed::Pass(token) => Some(token),
            _ => None,
        }
    }

    /// Attempt to parse the input; discarding any fail messages/results.
    /// - Will NOT log fail messages.
    /// - Will NOT revert to the previous state on fail.
    fn try_parse_or_skip(&self, input: &str) -> Option<Token> {
        match self.parse_opt_or_skip(input) {
            Parsed::Pass(token) => Some(token),
            _ => None,
        }
    }

    /// Attempt to parse using the cursor; discarding any fail messages/results.
    /// - Will NOT log fail messages.
    /// - Will NOT revert to the previous state on fail.
    fn try_parse_or_skip_at(&self, cursor: &mut Cursor) -> Option<Token> {
        match self.parse_opt_or_skip_at(cursor) {
            Parsed::Pass(token) => Some(token),
            _ => None,
        }
    }

    /// Parses the input with the given options.
    /// * `input` - The input to parse.
    /// * `optional` - If true; the parser will revert to the previous state on fail.
    /// * `ignored` - If true; prints a verbose ignored message instead of a fail message to the logs.
    fn parse_with_options(&self, input: &str, optional: bool, ignored: bool) -> Parsed {
        let mut cursor = Cursor::New(input);
        self.parse_with_options_at(&mut cursor, optional, ignored)
    }

    /// Parses the input using the cursor with the given options.
    /// * `cursor` - The cursor to start parsing with (Always starts at the current cursor position (`.curr_pos()`)).
    /// * `optional` - If true; the parser will revert to the previous state on fail.
    /// * `ignored` - If true; prints a verbose ignored message instead of a fail message to the logs.
    fn parse_with_options_at(&self, cursor: &mut Cursor, optional: bool, ignored: bool) -> Parsed {
        log::color!("PARSE", Color::Green);
        log::push_unique!("PARSE");
        log::push!(self.name().own_color().as_str());
        log::push_div!(":", Color::Green);
        log::info!(&[":START"], &format!("@ {}", cursor.curr_pos()));

        let start = if optional {
            cursor.save()
        } else {
            cursor.curr_pos()
        };

        let result = match self.rule(cursor) {
            End::Match(token) => {
                let end = if start >= cursor.curr_pos() {
                    start
                } else {
                    cursor.prev_pos()
                };

                let token = token
                    .assure_name(self.name())
                    .build_with_defaults(start, end);
                log::info!(
                    &[":END", "MATCH"],
                    &format!(
                        "@ {} => {}{}",
                        cursor.prev_pos(),
                        if log::IS_VV { "\n" } else { "" },
                        if log::IS_VV {
                            token.to_sexp_str(&cursor._src)
                        } else {
                            token.name.clone()
                        }
                    )
                    .color(Color::Green),
                );
                Parsed::Pass(token)
            }
            End::Fail(error) => {
                let end = if start >= cursor.curr_pos() {
                    start
                } else {
                    cursor.prev_pos()
                };
                let error = error
                    .tag(self.name())
                    .assure_name(self.name())
                    .build_with_defaults(start, end);

                if optional {
                    cursor.restore();
                }

                if ignored {
                    if log::IS_VV {
                        log::info!(
                            &[
                                ":END",
                                &"IGNORED"
                                    .effect(Effect::Strikethrough)
                                    .color(Color::BrightBlack)
                            ],
                            &format!(
                                "@ {} => \n{}",
                                cursor.prev_pos(),
                                match error {
                                    Some(ref e) => e.to_sexp_str(&cursor._src),
                                    None => "<None>".to_string(),
                                }
                            )
                            .effect(Effect::Strikethrough)
                            .color(Color::BrightBlack),
                        );
                    } else {
                        log::info!(
                            &[
                                ":END",
                                &"IGNORED"
                                    .color(Color::BrightBlack)
                                    .effect(Effect::Strikethrough)
                            ],
                            &format!("@ {}", cursor.prev_pos()).color(Color::BrightBlack),
                        );
                    }
                } else {
                    log::info!(
                        &[":END", "FAIL"],
                        &format!(
                            "@ {} => {}{}",
                            cursor.prev_pos(),
                            if log::IS_VV { "\n" } else { "" },
                            if log::IS_VV {
                                match error {
                                    Some(ref e) => e.to_sexp_str(&cursor._src),
                                    None => "<None>".to_string(),
                                }
                            } else {
                                match error {
                                    Some(ref e) => e.name.clone(),
                                    None => "<None>".to_string(),
                                }
                            }
                        )
                        .color(Color::Red)
                        .effect(Effect::Underline),
                    );
                }

                Parsed::Fail(error)
            }
            End::None => {
                if optional {
                    cursor.restore();
                }

                #[cfg(feature = "log")]
                let end = if start >= cursor.curr_pos() {
                    start
                } else {
                    cursor.prev_pos()
                };

                log::info!(
                    &[":END", &"NONE".color(Color::BrightBlack)],
                    &format!("@ {}", end)
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

    // #region Utils
    fn get(&self) -> Box<dyn Parser>;
    // #endregion

    // #region Tests
    fn get_tests(&self) -> Vec<crate::tests::parser::tokens::tests::Test> {
        Vec::new()
    }
    // #endregion
}
// #endregion
