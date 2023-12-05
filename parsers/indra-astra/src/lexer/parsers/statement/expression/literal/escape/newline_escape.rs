use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub const KEY: &str = "newline-escape";

pub struct Parser;
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        if cursor.try_read('\\') {
            if cursor.try_read('n') {
                return End::Token();
            }
        }

        End::None
    }
}
