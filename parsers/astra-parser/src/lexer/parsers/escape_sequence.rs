use crate::{lexer::parser, Cursor, End};

pub const KEY: &str = "escape-sequence";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        if cursor.try_read('\\') {
            cursor.read();
            return End::Token();
        } else {
            return End::Missing("prefix", "\\", &cursor.curr_str());
        }
    }
}
