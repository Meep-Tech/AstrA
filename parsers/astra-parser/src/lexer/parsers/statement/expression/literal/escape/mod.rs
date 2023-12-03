pub mod backtick_escape;
pub mod escape_sequence;
pub mod newline_escape;
pub mod quote_escape;
pub mod tab_escape;

use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub const KEY: &str = "escape";

pub struct Parser;
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, start: &mut Cursor) -> End {
        End::Choice(&KEY, start, &[&escape_sequence::Parser::Get()])
    }
}
