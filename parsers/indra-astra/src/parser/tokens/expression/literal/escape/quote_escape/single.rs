use crate::parser::tokens::token;

token! {
    single_quote_escape => |cursor: &mut Cursor| {
        if cursor.try_read('\\') {
            if cursor.try_read('\'') {
                return End::Token();
            }
        }

        End::None
    }
}
