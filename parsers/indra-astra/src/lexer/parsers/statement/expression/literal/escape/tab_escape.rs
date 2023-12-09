use crate::lexer::parsers::parser;

parser! {
    tab_escape => |cursor: &mut Cursor| {
        if cursor.try_read('\\') {
            if cursor.try_read('t') {
                return End::Token();
            }
        }

        End::None
    }
}
