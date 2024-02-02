use crate::parser::tokens::token;

token! {
  command => |_: &mut Cursor| {
    End::Not_Implemented()
  }
}
