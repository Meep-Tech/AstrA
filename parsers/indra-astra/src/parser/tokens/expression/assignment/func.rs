use crate::parser::tokens::token;

token! {
    func => |cursor: &mut Cursor| {
        End::Not_Implemented()
    }
}
