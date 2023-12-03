use crate::lexer::{cursor::Cursor, parser, parsers::statement::tree, results::end::End};

pub const KEY: &str = "struct";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Variant(&KEY, tree::Parser::Parse_At(cursor))
    }
}
