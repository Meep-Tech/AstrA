use crate::{
    lexer::{
        parser::{self},
        results::{builder::Builder, parsed::Parsed, token::Token},
    },
    Cursor, End,
};

pub const KEY: &str = "slash-lookup";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        if cursor.try_read('/') {
            match crate::lexer::parsers::name::Parser::Parse_At(cursor) {
                Parsed::Token(name) => {
                    return Token::new().prop("key", name).end();
                }
                Parsed::Error(error) => return End::Error_In_Prop(Token::new(), "key", error),
            }
        } else {
            return End::Missing("prefix", "/", &cursor.curr_str());
        }
    }
}
