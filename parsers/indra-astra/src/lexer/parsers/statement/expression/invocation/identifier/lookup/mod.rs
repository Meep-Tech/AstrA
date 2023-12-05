use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod dot_lookup;
pub mod slash_lookup;

pub const KEY: &str = "lookup";

pub struct Parser;
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Splay(
            &KEY,
            start,
            &[&dot_lookup::Parser::Get(), &slash_lookup::Parser::Get()],
        )
    }
}
