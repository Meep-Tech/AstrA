use crate::{
    lexer::{
        parser,
        results::{builder::Builder, parsed::Parsed, token::Token},
    },
    Cursor, End,
};

pub const KEY: &str = "dot-lookup";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        if cursor.try_read('.') {
            match crate::lexer::parsers::name::Parser::Parse_At(cursor) {
                Parsed::Pass(name) => return Token::new().prop("key", name).end(),
                Parsed::Fail(error) => return End::Error_In_Child(Token::new(), error),
            }
        } else {
            return End::Missing("prefix", ".", &cursor.curr_str());
        }
    }
}
