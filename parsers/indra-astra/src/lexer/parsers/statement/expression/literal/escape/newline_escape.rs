use crate::lexer::parsers::parser;

parser! {
    newline_escape => |cursor: &mut Cursor| {
        if cursor.try_read('\\') {
            if cursor.try_read('n') {
                return End::Token();
            }
        }

        End::None
    }
}
