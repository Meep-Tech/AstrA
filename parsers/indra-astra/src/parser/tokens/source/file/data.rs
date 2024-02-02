use crate::parser::tokens::token;

token! {
    data_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Data(file_type) => match file_type {
                fs::Data::Data => {
                    End::ToDo("read as data file")
                },
                fs::Data::Value => {
                    End::ToDo("read as value file")
                },
                fs::Data::StruX(struct_type) => match struct_type {
                    _ => End::ToDo("read as strux file")
                },
            },
            fs::Type::Unknown => {
                End::ToDo("try to read as data file")
            },
            _ => {
                End::Mismatch("file-type",
                    &format!("{:?}", fs::Data::Data),
                    &format!("{:?}", cursor.file_type())
                )
            },
        }
    }
}
