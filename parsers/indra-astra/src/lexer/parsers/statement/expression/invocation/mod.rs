use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod identifier;

pub const KEY: &str = "invocation";

pub struct Parser;
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Splay(&KEY, start, &[&identifier::Parser::Get()])
    }
}
