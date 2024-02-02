use crate::parser::tokens::token;

token! {
    prox_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Trait(file_type) => match file_type {
                fs::Trait::ProX => {
                    End::ToDo("read as prox file")
                },
                _ => {
                    End::Mismatch("file-type",
                        &format!("{:?}", fs::Trait::ProX),
                        &format!("{:?}", cursor.file_type())
                    )
                },
            },
            fs::Type::Unknown => {
                End::ToDo("try to read as prox file")
            },
            _ => {
                End::Mismatch("file-type",
                    &format!("{:?}", fs::Trait::ProX),
                    &format!("{:?}", cursor.file_type())
                )
            },
        }
    }
}
