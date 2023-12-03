pub mod single_quote_escape {
    use crate::lexer::{cursor::Cursor, parser, results::end::End};

    pub const KEY: &str = "single-quote-escape";

    pub struct Parser;
    impl parser::Parser for Parser {
        fn get_name(&self) -> &'static str {
            &KEY
        }

        fn rule(&self, cursor: &mut Cursor) -> End {
            if cursor.try_read('\\') {
                if cursor.try_read('\'') {
                    return End::Token();
                }
            }

            End::None
        }
    }
}

pub mod double_quote_escape {
    use crate::lexer::{cursor::Cursor, parser, results::end::End};

    pub const KEY: &str = "double-quote-escape";

    pub struct Parser;
    impl parser::Parser for Parser {
        fn get_name(&self) -> &'static str {
            &KEY
        }

        fn rule(&self, cursor: &mut Cursor) -> End {
            if cursor.try_read('\\') {
                if cursor.try_read('"') {
                    return End::Token();
                }
            }

            End::None
        }
    }
}
