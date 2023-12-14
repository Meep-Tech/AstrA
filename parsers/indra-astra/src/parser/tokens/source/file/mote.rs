use crate::parser::tokens::token;

token! {
    mote_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Mote => {
                End::TODO()
            },
            _ => End::TODO()
        }
    }
}
