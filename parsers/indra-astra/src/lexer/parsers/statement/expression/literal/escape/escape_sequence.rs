use crate::{lexer::parser, Cursor, End};

use super::newline_escape;

pub const KEY: &str = "escape-sequence";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        if cursor.try_read('\\') {
            cursor.read();
            match cursor.prev() {
                'n' => End::Build_Token_For_Variant_Of_Type::<newline_escape::Parser>(&KEY),
                _ => End::Token(),
            }
        } else {
            return End::Missing("prefix", "\\", &cursor.curr_str());
        }
    }
}
