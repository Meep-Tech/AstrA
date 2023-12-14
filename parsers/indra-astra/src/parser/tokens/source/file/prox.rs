use crate::parser::tokens::token;

token! {
    prox_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::AstrA => {
                End::TODO()
            },
            fs::Type::Trait(file_type) => match file_type {
                fs::Trait::ProX => {
                    End::TODO()
                },
                _ => End::TODO()
            },
            _ => End::TODO()
        }
    }
}
