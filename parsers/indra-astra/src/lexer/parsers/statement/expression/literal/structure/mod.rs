use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod closure;
pub mod tree;

pub const KEY: &str = "struct";

pub struct Parser {}
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Variant(&KEY, tree::Parser::Parse_At(cursor))
    }
}
