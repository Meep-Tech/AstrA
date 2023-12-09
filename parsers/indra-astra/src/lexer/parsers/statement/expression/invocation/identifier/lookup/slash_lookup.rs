use crate::lexer::parsers::parser;

parser! {
    slash_lookup => |cursor: &mut Cursor| {
        if cursor.try_read('/') {
            match crate::lexer::parsers::name::Parser::Parse_At(cursor) {
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
