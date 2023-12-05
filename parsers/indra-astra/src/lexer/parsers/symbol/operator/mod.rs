use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod assigner;

pub const KEY: &str = "operator";

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Splay(&KEY, cursor, &[&assigner::Parser::Get()])
    }
}
