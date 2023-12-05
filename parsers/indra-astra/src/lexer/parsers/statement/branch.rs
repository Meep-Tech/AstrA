use crate::lexer::{
    cursor::Cursor,
    parser,
    results::{end::End, parsed::Parsed},
};

use super::{assignment, expression};

pub const KEY: &str = "branch";

pub struct Parser {}
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, _cursor: &mut Cursor) -> End {
        match assignment::Parser::Parse_Opt_At(_cursor) {
            Parsed::Pass(token) => End::Token_Variant(&KEY, token),
            Parsed::Fail(_) => End::Variant(&KEY, expression::Parser::Parse_At(_cursor)),
        }
    }
}
