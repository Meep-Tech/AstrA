use crate::lexer::parsers::parser;

parser! {
  command => |_: &mut Cursor| {
    End::TODO()
  }
}
