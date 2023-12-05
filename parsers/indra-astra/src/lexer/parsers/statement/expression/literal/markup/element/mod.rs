use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod text;

pub const KEY: &str = "element";

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Splay(&KEY, start, &[&text::Parser::Get()])
    }
}
