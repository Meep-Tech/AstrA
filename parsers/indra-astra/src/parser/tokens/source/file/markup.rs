use crate::parser::tokens::token;

token! {
    markup_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Markup(file_type) => match file_type {
                fs::Markup::Markup => {
                    End::TODO()
                },
                fs::Markup::Component => {
                    End::TODO()
                },
                fs::Markup::BloX => {
                    End::TODO()
                },
            },
            _ => End::TODO()
        }
    }
}
