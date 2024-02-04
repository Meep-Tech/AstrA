use crate::parser::tokens::token;

token! {
    own_archetype => |cursor: &mut Cursor| {
        if cursor.try_read('.') && cursor.try_read('#') {
            return End::Token();
        } else {
            return End::Missing("symbol", ".#", &cursor.curr_str());
        }
    }
}
