use crate::parser::{self, cursor::Cursor, results::end::End};

pub const KEY: &str = "simple_string";

pub struct Parser;
impl parser::Type for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        if cursor.try_read('\'') {
            loop {}
        } else {
            End::Missing("start-delimiter", "\'", &cursor.curr_str())
        }
    }
}
