/// A boilerplate template for a parser.
use crate::{lexer::parser, Cursor, End};

pub const KEY: &str = "indent-current";

pub struct Parser {}
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        cursor.skip_ws();
        if cursor.indents.curr == cursor.indents.prev() {
            return End::Token();
        } else {
            return End::Missing(
                "level",
                &format!("indent level :{}", cursor.indents.curr),
                &format!("indent level :{}", cursor.indents.prev()),
            );
        }
    }
}
