use crate::parser::tokens::token;

token! {
    decrease_indent => |cursor: &mut Cursor| {
        cursor.skip_ws();

        if cursor.indent().curr < cursor.indent().prev() {
            return End::Token();
        } else {
            return End::Missing(
                "level",
                &format!("indent level to be below :{}", cursor.indent().prev()),
                &format!("indent level :{}", cursor.indent().curr),
            );
        }
    }
}
