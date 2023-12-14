use crate::parser::tokens::token;

token! {
    current_indent => |cursor: &mut Cursor| {
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
