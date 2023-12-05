/// A boilerplate template for a parser.
use crate::{lexer::parser, Cursor, End};

pub const KEY: &str = "indent-increase";

pub struct Parser {}
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        cursor.skip_ws();

        if cursor.indents.curr > cursor.indents.prev() {
            return End::Token();
        } else {
            return End::Missing(
                "level",
                &format!("indent leve to be above :{}", cursor.indents.prev()),
                &format!("indent level :{}", cursor.indents.curr),
            );
        }
    }
}
