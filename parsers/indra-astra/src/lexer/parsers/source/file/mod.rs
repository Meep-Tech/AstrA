use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod mote;
pub mod r#trait;

pub const KEY: &str = "file";

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Splay(
            &KEY,
            start,
            &[&mote::Parser::Get(), &r#trait::Parser::Get()],
        )
    }
}
