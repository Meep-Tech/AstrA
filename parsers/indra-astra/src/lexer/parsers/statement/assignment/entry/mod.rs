use crate::lexer::{cursor::Cursor, parser, results::end::End};

/// named-entry
///   - key: name
///   - ?increase-indent | ?gap
///   - operator: assigner
///   - ?increase-indent | ?gap
///   - value: value
pub mod named_entry;

pub const KEY: &str = "entry";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Variant(&KEY, named_entry::Parser::Parse_At(cursor))
    }
}
