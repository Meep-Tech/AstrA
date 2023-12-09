use crate::lexer::parsers::parser;

pub mod command;
pub mod file;

parser! {
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
