use crate::parser::tokens::token;

token! {
    prefixed_expression => |cursor: &mut Cursor| {
        End::Not_Implemented()
    }
}
