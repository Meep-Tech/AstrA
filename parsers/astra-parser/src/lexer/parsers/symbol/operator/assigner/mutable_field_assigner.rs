use crate::{
    lexer::{context::Language, parser},
    Cursor, End,
};

pub const KEY: &str = "mutable-field-assigner";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        match cursor.lang() {
            Language::StruX => {
                if cursor.try_read(':') {
                    if cursor.curr().is_whitespace() {
                        return End::Token();
                    } else {
                        End::Missing("trailing-whitespace", "\\s", &cursor.curr_str())
                    }
                } else {
                    End::Missing("prefix", ":", &cursor.curr_str())
                }
            }
            _ => {
                if cursor.try_read('~') {
                    End::Token()
                } else {
                    End::Missing("prefix", "=", &cursor.curr_str())
                }
            }
        }
    }
}
