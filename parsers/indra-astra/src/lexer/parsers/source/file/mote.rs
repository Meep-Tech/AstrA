use crate::lexer::parsers::parser;

parser! {
    mote_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Mote => {
                End::TODO()
            },
            _ => End::TODO()
        }
    }
}
