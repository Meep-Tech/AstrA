pub mod escape;
pub mod markup;
pub mod primitive;
pub mod structure;

use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub const KEY: &str = "literal";

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Splay(
            &KEY,
            start,
            &[
                &escape::Parser::Get(),
                &markup::Parser::Get(),
                &primitive::Parser::Get(),
                &structure::Parser::Get(),
            ],
        )
    }
}
