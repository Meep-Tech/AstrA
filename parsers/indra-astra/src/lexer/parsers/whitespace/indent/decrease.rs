use crate::lexer::parsers::parser;

parser! {
    decrease_indent => |cursor: &mut Cursor| {
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
