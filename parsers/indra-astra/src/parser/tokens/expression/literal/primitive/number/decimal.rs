use crate::parser::tokens::token;

token! {
  decimal => |cursor: &mut Cursor| {
    End::Not_Implemented()
  }
}
