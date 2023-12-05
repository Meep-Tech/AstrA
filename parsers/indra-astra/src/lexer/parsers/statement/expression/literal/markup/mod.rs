use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod element;

pub const KEY: &str = "markup";

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Splay(&KEY, start, &[&element::Parser::Get()])
    }
}
