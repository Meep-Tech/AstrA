/// A boilerplate template for a parser.
use crate::{lexer::parser, Cursor, End};

pub const KEY: &str = "indent-current";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        let current_indent = cursor.indents.curr_levels();
        cursor.skip_ws();
        let next_indent = cursor.indents.curr_levels();
        if next_indent == current_indent {
            return End::Token();
        } else {
            return End::Missing(
                "level",
                &format!("indent level :{:}", current_indent),
                &format!("indent level :{:}", next_indent),
            );
        }
    }
}
