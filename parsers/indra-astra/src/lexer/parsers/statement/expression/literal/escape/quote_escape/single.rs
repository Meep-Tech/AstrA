use crate::lexer::parsers::parser;

parser! {
    single_quote_escape => |cursor: &mut Cursor| {
        if cursor.try_read('\\') {
            if cursor.try_read('\'') {
                return End::Token();
            }
        }

        End::None
    }
}
