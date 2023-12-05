use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod assignment;
pub mod branch;
pub mod expression;

pub const KEY: &str = "statement";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Splay(
            &KEY,
            cursor,
            &[&assignment::Parser::Get(), &expression::Parser::Get()],
        )
    }
}
