use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod key;
pub mod lookup;

pub const KEY: &str = "identifier";

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Splay(&KEY, start, &[&key::Parser::Get(), &lookup::Parser::Get()])
    }
}
