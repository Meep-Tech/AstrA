use crate::parser::tokens::token;

pub const MUT_TAG: &'static str = "mutable";
pub const CONST_TAG: &'static str = "constant";

token! {
  proc_assigner => |cursor: &mut Cursor| {
        if cursor.try_read(':') {
          if cursor.try_read(':') {
            if cursor.try_read('>') && cursor.try_read('>') {
              // ::>>
              return End::New().tag(CONST_TAG).to_end()
            } else if cursor.curr_is_ws() {
              cursor.read();
              if cursor.try_read('>') && cursor.try_read('>') {
                // :: >>
                return End::New().tag(MUT_TAG).to_end()
              } else {
                return End::Missing("symbol", ">", &cursor.curr_str())
              }
            } else {
              return End::Missing("symbol", ">", &cursor.curr_str())
            }
          } else {
            if cursor.try_read('>') && cursor.try_read('>') {
              // :>>
              return End::New().tag(MUT_TAG).to_end()
            } else if cursor.curr_is_ws() {
              cursor.read();
              if cursor.try_read('>') && cursor.try_read('>') {
                // : >>
                return End::New().tag(MUT_TAG).to_end()
              } else {
                return End::Missing("symbol", ">", &cursor.curr_str())}
            } else {
              return End::Missing("symbol", ">", &cursor.curr_str())
            }
          }
        } else if cursor.try_read('>') && cursor.try_read('>') {
          // >>
          return End::New().tag(MUT_TAG).to_end()
        } else {
          return End::Missing("symbol", ">", &cursor.curr_str())
        }
    },
    tests:
        unit!(["Mutable" & "Space After"]
            : ">>"
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .tag(MUT_TAG)
                .build_from(0, 0)))
        unit!(["Mutable" & "Spaces Around"]
            : " >> "
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .tag(MUT_TAG)
                .build_from(0, 0)))
        unit!(["Mutable" & "Tab After"]
            : ">>\t"
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .tag(MUT_TAG)
                .build_from(0, 0)))
        unit!(["Mutable" & "Tabs Around"]
            : "\t>>\t"
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .tag(MUT_TAG)
                .build_from(0, 0)))
        unit!(["Mutable" & "Space Before" & "Newline After" & "Tab After"]
            : " >>\n\t"
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .tag(MUT_TAG)
                .build_from(0, 0)))
}
