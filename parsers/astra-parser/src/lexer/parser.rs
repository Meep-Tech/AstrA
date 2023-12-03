use std::{any::TypeId, rc::Rc};

use crate::utils::log::{self, Styleable};

use super::{
    cursor::Cursor,
    parsers,
    results::{builder::Builder, end::End, parsed::Parsed, token::Token},
};

pub trait Parser: Sync {
    #[allow(non_snake_case)]
    fn Instance() -> &'static Rc<Self>
    where
        Self: Sync + 'static + Sized,
    {
        parsers::get_by_type::<Self>()
    }

    #[allow(non_snake_case)]
    fn Get() -> &'static Rc<dyn Parser>
    where
        Self: Sync + 'static + Sized,
    {
        parsers::get_for_type::<Self>()
    }

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

    fn get_name(&self) -> &'static str;
    fn get_type_id(&self) -> TypeId
    where
        Self: 'static,
    {
        std::any::TypeId::of::<Self>()
    }
    fn get_type_name(&self) -> &'static str
    where
        Self: 'static,
    {
        std::any::type_name::<Self>()
    }

    fn is_ignored(&self) -> bool {
        false
    }

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
        log::add_color("PARSE", log::Color::Green);
        log::push_unique_key("PARSE");
        log::push_key(self.get_name());
        log::push_key_div(":", log::Color::Green);
        log::info!(&[":START"], &format!("@ {}", cursor.pos));

        let start = if optional { cursor.save() } else { cursor.pos };

        let result = match self.rule(cursor) {
            End::Match(token) => {
                let token = token
                    .assure_name(self.get_name())
                    .build(start, cursor.prev_pos());
                log::info!(
                    &[":END", "MATCH"],
                    &format!("@ {} = {:#?}", cursor.prev_pos(), token).color(log::Color::Green),
                );
                Parsed::Pass(token)
            }
            End::Fail(error) => {
                let err = error
                    .tag(self.get_name())
                    .assure_name(self.get_name())
                    .build(start, cursor.prev_pos());

                if optional {
                    cursor.restore();
                }

                if ignored {
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
                        &[":END", "FAIL"],
                        &format!("@ {} = {:#?}", cursor.prev_pos(), err)
                            .color(log::Color::Red)
                            .effect(log::Effect::Underline),
                    );
                }

                Parsed::Fail(err)
            }
            End::None => {
                if optional {
                    cursor.restore();
                }

                log::info!(&[":END", "NONE"], &format!("@ {}", cursor.prev_pos()));
                Parsed::Fail(None)
            }
        };

        log::pop_key();
        log::pop_key();
        log::pop_unique_key("PARSE");

        result
    }
}
