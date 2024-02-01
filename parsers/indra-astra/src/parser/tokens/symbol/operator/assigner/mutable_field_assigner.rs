use crate::parser::tokens::token;

token! {
    mutable_field_assigner => |cursor: &mut Cursor| {
        if cursor.try_read(':') {
            if cursor.curr().is_whitespace() {
                return End::Token();
            } else {
                End::Missing("trailing_whitespace", "\\s", &cursor.curr_str())
            }
        } else {
            End::Missing("symbol", ":", &cursor.curr_str())
        }
    },
    tests:
        unit!(["Space After"]
            : ": "
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .build(0, 0)))
        unit!(["Tab After"]
            : ":\t"
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .build(0, 0)))
        unit!(["Newline After" & "Tab After"]
            : ":\n\t"
            => Parsed::Pass(Token::New()
                .name(&KEY)
                .build(0, 0)))
}
