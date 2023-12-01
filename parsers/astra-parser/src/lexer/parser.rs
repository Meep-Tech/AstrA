use std::{any::TypeId, rc::Rc};

use crate::utils::log::{self, Styleable};

use super::{
    cursor::Cursor,
    parsers,
    results::{
        builder::Builder,
        end::End,
        parsed::{Optional, Parsed},
        token::Token,
    },
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
    fn Parse_Opt(input: &str) -> Optional
    where
        Self: Sync + 'static + Sized,
    {
        let result = Self::Parse(input);
        match result {
            Parsed::Token(token) => Optional::Token(token),
            Parsed::Error(err) => Optional::Ignored(err),
        }
    }

    #[allow(non_snake_case)]
    fn Parse_Opt_At(cursor: &mut Cursor) -> Optional
    where
        Self: Sync + 'static + Sized,
    {
        let result = Self::Parse_At(cursor);
        match result {
            Parsed::Token(token) => Optional::Token(token),
            Parsed::Error(err) => Optional::Ignored(err),
        }
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
        self.parse_with_options_at(cursor, false)
    }

    fn parse_opt(&self, input: &str) -> Optional {
        let result = self.parse_with_options(input, true);
        match result {
            Parsed::Token(token) => Optional::Token(token),
            Parsed::Error(err) => Optional::Ignored(err),
        }
    }

    fn parse_opt_at(&self, cursor: &mut Cursor) -> Optional {
        let result = self.parse_with_options_at(cursor, true);
        match result {
            Parsed::Token(token) => Optional::Token(token),
            Parsed::Error(err) => Optional::Ignored(err),
        }
    }

    fn try_parse_at(&self, cursor: &mut Cursor) -> Option<Token> {
        match self.parse_opt_at(cursor) {
            Optional::Token(token) => Some(token),
            _ => None,
        }
    }

    fn try_parse(&self, input: &str) -> Option<Token> {
        match self.parse_opt(input) {
            Optional::Token(token) => Some(token),
            _ => None,
        }
    }

    fn parse_with_options(&self, input: &str, optional: bool) -> Parsed {
        let mut cursor = Cursor::new(input);
        self.parse_with_options_at(&mut cursor, optional)
    }

    fn parse_with_options_at(&self, cursor: &mut Cursor, optional: bool) -> Parsed {
        log::push_unique_key(&"PARSE".color(log::Color::Green));
        log::push_key(self.get_name());
        log::push_key_div(":", log::Color::Green);
        log::info(&[":START"], &format!("@ {}", cursor.pos));

        let start = cursor.save();

        let result = match self.rule(cursor) {
            End::Match(token) => {
                let token = token.assure_name(self.get_name()).build(start, cursor.pos);
                log::info(
                    &[":END", "MATCH"],
                    &format!("@ {} = {:#?}", cursor.pos, token).color(log::Color::Green),
                );
                Parsed::Token(token)
            }
            End::Fail(error) => {
                let err = error
                    .tag(self.get_name())
                    .assure_name(self.get_name())
                    .build(start, cursor.pos);
                if optional {
                    log::info(
                        &[":END", &"IGNORED".effect(log::Effect::Strikethrough)],
                        &format!("@ {} = {:#?}", cursor.pos, err)
                            .effect(log::Effect::Strikethrough),
                    );
                } else {
                    log::info(
                        &[":END", "FAIL"],
                        &format!("@ {} = {:#?}", cursor.pos, err)
                            .color(log::Color::Red)
                            .effect(log::Effect::Underline),
                    );
                }

                cursor.restore();
                Parsed::Error(err)
            }
        };

        log::pop_key();
        log::pop_key();
        log::pop_unique_key("PARSE");

        result
    }
}
