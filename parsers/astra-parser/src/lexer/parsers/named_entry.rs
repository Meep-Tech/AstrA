use crate::{
    lexer::results::token::Token,
    lexer::{parser, parsers::name},
    lexer::{parsers::naked_text, results::builder::Builder},
    Cursor, End, Parsed,
};

use super::mutable_field_assigner;

pub const KEY: &str = "named-entry";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        let mut result = Token::new();

        let key = name::Parser::Parse_At(cursor);
        match key {
            Parsed::Token(key) => {
                result = result.prop("key", key);
                cursor.skip_ws();

                let assigner = mutable_field_assigner::Parser::Parse_At(cursor);
                match assigner {
                    Parsed::Token(assigner) => {
                        result = result.prop("operator", assigner);
                        cursor.skip_ws();

                        let value = naked_text::Parser::Parse_At(cursor);

                        match value {
                            Parsed::Token(value) => {
                                return result.prop("value", value).end();
                            }
                            Parsed::Error(error) => {
                                return End::Error_In_Prop(result, "value", error)
                            }
                        }
                    }
                    Parsed::Error(error) => return End::Error_In_Prop(result, "operator", error),
                }
            }
            Parsed::Error(error) => return End::Error_In_Prop(result, "key", error),
        }
    }
}
