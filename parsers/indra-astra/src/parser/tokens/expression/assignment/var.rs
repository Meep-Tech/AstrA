use crate::parser::tokens::token;

token! {
    var => |cursor: &mut Cursor| {
        End::Not_Implemented()
    }
}
