use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub const KEY: &str = "trait";

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, _start: &mut Cursor) -> End {
        todo!()
    }
}
