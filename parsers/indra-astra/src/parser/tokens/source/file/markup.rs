use crate::parser::tokens::token;

token! {
    markup_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Markup(file_type) => match file_type {
                fs::Markup::Markup => {
                    End::ToDo("read as markup file")
                },
                fs::Markup::Component => {
                    End::ToDo("read as component file")
                },
                fs::Markup::BloX => {
                    End::ToDo("read as blox file")
                },
            },
            fs::Type::Unknown => {
                End::ToDo("try to read as markup file")
            },
            _ => {
                End::Mismatch("file-type",
                    &format!("{:?}", fs::Markup::Markup),
                    &format!("{:?}", cursor.file_type())
                )
            },
        }
    }
}
