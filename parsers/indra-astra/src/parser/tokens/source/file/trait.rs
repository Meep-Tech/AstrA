use crate::parser::tokens::token;

token! {
    prox_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Trait(file_type) => match file_type {
                fs::Trait::Trait => {
                    End::ToDo("read as trait file")
                },
                fs::Trait::Prototype => {
                    End::ToDo("read as prototype file")
                },
                fs::Trait::Archetype => {
                    End::ToDo("read as archetype file")
                },
                fs::Trait::Enum => {
                    End::ToDo("read as enum file")
                },
                fs::Trait::ProX => {
                    End::ToDo("read as prox file")
                },
            },
            fs::Type::Unknown => {
                End::ToDo("try to read as trait file")
            },
            _ => {
                End::Mismatch("file-type",
                    &format!("{:?}", fs::Trait::Trait),
                    &format!("{:?}", cursor.file_type())
                )
            },
        }
    }
}
