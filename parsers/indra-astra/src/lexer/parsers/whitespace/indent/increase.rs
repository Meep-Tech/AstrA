use crate::lexer::parsers::parser;

parser! {
    increase_indent => |cursor: &mut Cursor| {
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
