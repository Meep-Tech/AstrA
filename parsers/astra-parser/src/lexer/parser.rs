use std::{any::TypeId, rc::Rc};

use crate::utils::log::{self, Colorable};

use super::{
    cursor::Cursor,
    parsers,
    results::{builder::Builder, end::End, parsed::Parsed, token::Token},
};

pub trait Parser: Sync {
    fn instance() -> &'static Rc<Self>
    where
        Self: Sync + 'static + Sized,
    {
        parsers::get_by_type::<Self>()
    }

    fn get_name(&self) -> &'static str;
    fn get_type_id(&self) -> TypeId
    where
        Self: 'static,
    {
        return std::any::TypeId::of::<Self>();
    }
    fn get_type_name(&self) -> &'static str
    where
        Self: 'static,
    {
        return std::any::type_name::<Self>();
    }

    fn is_ignored(&self) -> bool {
        return false;
    }

    fn rule(&self, start: &mut Cursor) -> Option<End>;

    fn parse(&self, input: &str) -> Option<Parsed> {
        let mut cursor = Cursor::new(input);
        return self.parse_at(&mut cursor);
    }

    fn parse_at(&self, cursor: &mut Cursor) -> Option<Parsed> {
        log::push_unique_key(&"PARSE".color(log::Color::Green));
        log::push_key(self.get_name());
        log::push_key_div(":", log::Color::Green);
        log::info(&[":START"], &format!("@ {}", cursor.pos));

        let start = cursor.save();

        let result = match self.rule(cursor) {
            Some(result) => match result {
                End::Match(token) => {
                    let token = token.assure_name(self.get_name()).build(start, cursor.pos);
                    log::info(
                        &[":END", "MATCH"],
                        &format!("@ {} = {:#?}", cursor.pos, token),
                    );
                    Some(Parsed::Token(token))
                }
                End::Fail(error) => {
                    let err = error.tag(self.get_name()).build(start, cursor.pos);
                    log::info(&[":END", "FAIL"], &format!("@ {} = {:#?}", cursor.pos, err));

                    let result = Some(Parsed::Error(err));
                    cursor.restore();

                    result
                }
            },
            None => {
                log::info(&[":END", "NONE"], &format!("@ {}", cursor.pos));
                cursor.restore();
                None
            }
        };

        log::pop_key();
        log::pop_key();
        log::pop_unique_key("PARSE");

        return result;
    }

    fn try_parse_at(&self, cursor: &mut Cursor) -> Option<Token> {
        return match self.parse_at(cursor) {
            Some(Parsed::Token(token)) => Some(token),
            _ => None,
        };
    }
}
