use crate::parser::tokens::token;

pub mod command;
pub mod file;

token! {
  source => |cursor: &mut Cursor| {
    End::Splay(
      &KEY,
      cursor,
      &[
        &command::Parser::Get(),
        &file::Parser::Get(),
      ]
    )
  }
}
