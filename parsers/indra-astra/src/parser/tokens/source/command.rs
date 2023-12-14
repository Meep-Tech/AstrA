use crate::parser::tokens::token;

token! {
  command => |_: &mut Cursor| {
    End::TODO()
  }
}
