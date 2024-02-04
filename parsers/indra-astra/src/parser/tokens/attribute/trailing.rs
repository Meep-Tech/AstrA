use crate::parser::tokens::token;

token! {
    trailing_attributes => |cursor: &mut Cursor| {
        cursor.skip_ws();
        End::Not_Implemented()
    }
}
