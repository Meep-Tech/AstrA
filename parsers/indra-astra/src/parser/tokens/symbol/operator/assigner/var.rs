use crate::parser::tokens::token;

pub const MUT_TAG: &str = "mut";
pub const CONST_TAG: &str = "const";

token! {
  var_assigner => |cursor: &mut Cursor| {
    if cursor.try_read('=') {
      return End::New().tag(CONST_TAG).end();
    } else if cursor.try_read('~') && cursor.try_read('=') {
      return End::New().tag(MUT_TAG).end();
    } else {
      return End::Missing("symbol", "= or ~=", &cursor.curr_str());
    }
  }
}
