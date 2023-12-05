use crate::lexer::{cursor::Cursor, parser, results::end::End};

pub mod single {
    use crate::lexer::{cursor::Cursor, parser, results::end::End};

    pub const KEY: &str = "single-quote-escape";

    pub struct Parser;
    impl parser::Parser for Parser {
        fn name(&self) -> &'static str {
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

pub mod double {
    use crate::lexer::{cursor::Cursor, parser, results::end::End};

    pub const KEY: &str = "double-quote-escape";

    pub struct Parser;
    impl parser::Parser for Parser {
        fn name(&self) -> &'static str {
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

pub struct Parser;
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        "quote-escape"
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        End::Splay(
            "quote-escape",
            cursor,
            &[&single::Parser::Get(), &double::Parser::Get()],
        )
    }
}
