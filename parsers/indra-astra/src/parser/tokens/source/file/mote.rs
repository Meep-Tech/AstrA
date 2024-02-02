use crate::parser::tokens::token;

token! {
    mote_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Mote => {
                End::ToDo("read as mote file")
            },
            fs::Type::Unknown => {
                End::ToDo("try to read as mote file")
            },
            _ => {
                End::Mismatch("file-type",
                    &format!("{:?}", fs::Type::Mote),
                    &format!("{:?}", cursor.file_type())
                )
            },
        }
    }
}
