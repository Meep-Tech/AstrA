use crate::parser::tokens::token;

pub mod command;
pub mod file;

token! {
  source => |cursor: &mut Cursor| {
    End::Splay(
      &KEY,
      cursor,
      &[
        &command::Token::Get(),
        &file::Token::Get(),
      ]
    )
  }
}
