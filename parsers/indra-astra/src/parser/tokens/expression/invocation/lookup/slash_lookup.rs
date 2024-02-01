use crate::parser::tokens::token;

token! {
    slash_lookup => |cursor: &mut Cursor| {
        if cursor.try_read('/') {
            match crate::parser::tokens::expression::identifier::key::name::Parser::Parse_At(cursor) {
                Parsed::Pass(name) => {
                    return End::New().prop("key", name).end();
                }
                Parsed::Fail(error) => return End::Error_In_Prop_Of(End::New(), "key", error),
            }
        } else {
            return End::Missing("prefix", "/", &cursor.curr_str());
        }
    }
}
