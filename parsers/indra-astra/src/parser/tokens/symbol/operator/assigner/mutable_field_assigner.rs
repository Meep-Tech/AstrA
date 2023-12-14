use crate::parser::tokens::token;

token! {
    mutable_field_assigner => |cursor: &mut Cursor| {
        match cursor.lang() {
            Language::StruX => {
                if cursor.try_read(':') {
                    if cursor.curr().is_whitespace() {
                        return End::Token();
                    } else {
                        End::Missing("trailing_whitespace", "\\s", &cursor.curr_str())
                    }
                } else {
                    End::Missing("prefix", ":", &cursor.curr_str())
                }
            }
            _ => {
                if cursor.try_read('~') {
                    End::Token()
                } else {
                    End::Missing("prefix", "=", &cursor.curr_str())
                }
            }
        }
    }
}
