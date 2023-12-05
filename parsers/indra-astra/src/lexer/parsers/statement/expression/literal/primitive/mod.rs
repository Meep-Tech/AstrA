pub mod string;

use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub const KEY: &str = "primitive";

pub struct Parser;
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Splay(&KEY, start, &[string::Parser::Get()])
    }
}
