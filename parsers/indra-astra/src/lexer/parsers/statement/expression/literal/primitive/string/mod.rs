pub mod simple_string;

use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub const KEY: &str = "string";

pub struct Parser;
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Splay(&KEY, cursor, &[&simple_string::Parser::Get()])
    }
}
