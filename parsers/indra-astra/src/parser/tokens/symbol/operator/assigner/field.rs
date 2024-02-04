use crate::parser::tokens::token;

pub const MUT_TAG: &'static str = "mutable";
pub const CONST_TAG: &'static str = "constant";

token! {
  field_assigner => |cursor: &mut Cursor| {
        if cursor.try_read(':') {
          if cursor.try_read(':') {
            if cursor.curr().is_whitespace() {
              return End::New().tag(CONST_TAG).end();
            } else {
              return End::Missing("trailing_whitespace", "\\s", &cursor.curr_str());
            }
          } else {
            if cursor.curr().is_whitespace() {
              return End::New().tag(MUT_TAG).end();
            } else {
              return End::Missing("trailing_whitespace", "\\s", &cursor.curr_str());
            }
          }
        } else {
          return End::Missing("symbol", ":", &cursor.curr_str());
        }
    },
    tests:
        unit!(["Mutable" & "Space After"]
            : ": "
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .tag(MUT_TAG)
                .build(0, 0)))
        unit!(["Mutable" & "Tab After"]
            : ":\t"
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .tag(MUT_TAG)
                .build(0, 0)))
        unit!(["Mutable" & "Newline After" & "Tab After"]
            : ":\n\t"
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .tag(MUT_TAG)
                .build(0, 0)))
}
