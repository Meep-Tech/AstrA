use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod invocation;
pub mod literal;

pub const KEY: &str = "expression";

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Splay(
            &KEY,
            start,
            &[&literal::Parser::Get(), &invocation::Parser::Get()],
        )
    }
}
