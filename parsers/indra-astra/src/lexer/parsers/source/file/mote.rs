use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub const KEY: &str = "mote";

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, _start: &mut Cursor) -> End {
        todo!()
    }
}
