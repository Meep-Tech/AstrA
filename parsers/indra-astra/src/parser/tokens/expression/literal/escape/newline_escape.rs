use crate::parser::tokens::token;

token! {
    newline_escape => |cursor: &mut Cursor| {
        if cursor.try_read('\\') {
            if cursor.try_read('n') {
                return End::Token();
            }
        }

        End::None
    }
}
