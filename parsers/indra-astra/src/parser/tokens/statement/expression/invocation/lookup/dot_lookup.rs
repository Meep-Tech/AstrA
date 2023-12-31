use crate::parser::tokens::token;

token! {
    dot_lookup => |cursor: &mut Cursor| {
        if cursor.try_read('.') {
            match crate::parser::tokens::name::Parser::Parse_At(cursor) {
                Parsed::Pass(name) => return End::New().prop("key", name).end(),
                Parsed::Fail(error) => return End::Unexpected_Child_Of(End::New(), error),
            }
        } else {
            return End::Missing("prefix", ".", &cursor.curr_str());
        }
    }
}
