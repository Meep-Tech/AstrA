use crate::parser::tokens::token;

token! {
    current_indent => |cursor: &mut Cursor| {
        cursor.skip_ws();
        if cursor.indent().curr == cursor.indent().prev() {
            return End::Token();
        } else {
            return End::Missing(
                "level",
                &format!("indent level :{}", cursor.indent().curr),
                &format!("indent level :{}", cursor.indent().prev()),
            );
        }
    }
}
