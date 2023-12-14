use crate::parser::tokens::token;

token! {
    prox_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Trait(file_type) => match file_type {
                fs::Trait::Trait => {
                    End::TODO()
                },
                fs::Trait::Prototype => {
                    End::TODO()
                },
                fs::Trait::Archetype => {
                    End::TODO()
                },
                fs::Trait::Enum => {
                    End::TODO()
                },
                _ => End::TODO()
            },
            _ => End::TODO()
        }
    }
}
