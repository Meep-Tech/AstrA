use crate::parser::tokens::token;

token! {
    increase_indent => |cursor: &mut Cursor| {
        cursor.skip_ws();

        if cursor.indent().curr > cursor.indent().prev() {
            return End::Token();
        } else {
            return End::Missing(
                "level",
                &format!("indent leve to be above :{}", cursor.indent().prev()),
                &format!("indent level :{}", cursor.indent().curr),
            );
        }
    }
}
