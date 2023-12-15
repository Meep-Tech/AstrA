use crate::parser::{self, cursor::Cursor, results::end::End};

pub const KEY: &str = "simple_string";

pub struct Token;
impl parser::Type for Token {
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
