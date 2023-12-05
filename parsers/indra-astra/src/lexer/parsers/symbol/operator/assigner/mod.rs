use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod mutable_field_assigner;

pub const KEY: &str = "assigner";

pub struct Parser;
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        super::End::Splay(&KEY, cursor, &[&mutable_field_assigner::Parser::Get()])
    }
}
