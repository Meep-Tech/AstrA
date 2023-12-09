use crate::lexer::parsers::{parser, tests};

parser! {
    #testable,
    data_file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            fs::Type::Data(file_type) => match file_type {
                fs::Data::Data => {
                    End::TODO()
                },
                fs::Data::Value => {
                    End::TODO()
                },
                fs::Data::StruX(struct_type) => match struct_type {
                    _ => End::TODO()
                },
            },
            _ => End::TODO()
        }
    }
}

tests! {}
