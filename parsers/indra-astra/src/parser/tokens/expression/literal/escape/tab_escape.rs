use crate::parser::tokens::token;

token! {
    tab_escape => |cursor: &mut Cursor| {
        if cursor.try_read('\\') {
            if cursor.try_read('t') {
                return End::Token();
            }
        }

        End::None
    }
}
