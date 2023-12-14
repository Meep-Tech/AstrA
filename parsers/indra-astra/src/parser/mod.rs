pub mod colors;
pub mod context;
pub mod cursor;
pub mod fs;
pub mod indents;
pub mod results;
pub mod tokens;

use crate::utils::log::{self};
use cursor::Cursor;
use results::{builder::Builder, end::End, parsed::Parsed, token::Token};
use std::{any::TypeId, rc::Rc};

#[cfg(feature = "verbose")]
use crate::utils::log::Styleable;

pub trait Parser: Sync {
    // #region Static
    // #region Get
    #[allow(non_snake_case)]
    fn Instance() -> &'static Rc<Self>
    where
        Self: Sync + 'static + Sized,
    {
        tokens::get_by_type::<Self>()
    }

    #[allow(non_snake_case)]
    fn Get() -> &'static Rc<dyn Parser>
    where
        Self: Sync + 'static + Sized,
    {
        tokens::get_for_type::<Self>()
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
    fn Try_Parse(input: &str) -> Option<Token>
    where
        Self: Sync + 'static + Sized,
    {
        Self::Instance().try_parse(input)
    }

    #[allow(non_snake_case)]
    fn Try_Parse_At(cursor: &mut Cursor) -> Option<Token>
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
    fn rule(&self, start: &mut Cursor) -> End;

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

    fn try_parse_at(&self, cursor: &mut Cursor) -> Option<Token> {
        match self.parse_opt_at(cursor) {
            Parsed::Pass(token) => Some(token),
            _ => None,
        }
    }

    fn try_parse(&self, input: &str) -> Option<Token> {
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
