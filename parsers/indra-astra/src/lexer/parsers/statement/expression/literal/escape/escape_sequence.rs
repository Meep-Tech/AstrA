use super::newline_escape;
use crate::lexer::parsers::parser;

parser! {
    escape_sequence => |cursor: &mut Cursor| {
        if cursor.try_read('\\') {
            cursor.read();
            match cursor.prev() {
                'n' => End::Build_Token_For_Variant_Of_Type::<newline_escape::Parser>(&KEY),
                _ => End::Token(),
            }
        } else {
            return End::Missing("prefix", "\\", &cursor.curr_str());
        }
    }
}
