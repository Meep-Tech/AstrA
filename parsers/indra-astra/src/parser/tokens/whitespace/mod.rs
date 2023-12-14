use crate::parser::{self, cursor::Cursor, results::end::End};

pub mod indent;

pub const KEY: &str = "whitespace";

pub struct Parser {}
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Splay(&KEY, cursor, &[&indent::Parser::Get()])
    }
}
