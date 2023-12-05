use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod operator;

pub const KEY: &str = "symbol";

pub struct Parser {}

impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Splay(&KEY, cursor, &[&operator::Parser::Get()])
    }
}
