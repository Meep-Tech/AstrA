use crate::parser::tokens::token;

token! {
    prefixed_invokation => |cursor: &mut Cursor| {
        End::Not_Implemented()
    }
}
