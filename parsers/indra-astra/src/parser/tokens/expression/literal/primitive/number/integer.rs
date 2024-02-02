use crate::parser::tokens::token;

token! {
  integer => |cursor: &mut Cursor| {
    End::Not_Implemented()
  }
}
