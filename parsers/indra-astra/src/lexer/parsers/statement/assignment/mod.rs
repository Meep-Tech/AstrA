use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod entry;

pub const KEY: &str = "assignment";

pub struct Parser {}
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Variant(&KEY, entry::Parser::Parse_At(cursor))
    }
}
