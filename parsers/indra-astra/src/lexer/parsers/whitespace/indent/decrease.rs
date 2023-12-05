use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub const KEY: &str = "indent-decrease";

pub struct Parser {}
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        cursor.skip_ws();

        if cursor.indents.curr < cursor.indents.prev() {
            return End::Token();
        } else {
            return End::Missing(
                "level",
                &format!("indent level to be below :{}", cursor.indents.prev()),
                &format!("indent level :{}", cursor.indents.curr),
            );
        }
    }
}
