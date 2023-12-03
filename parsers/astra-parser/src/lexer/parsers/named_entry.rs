use crate::{
    lexer::results::token::Token,
    lexer::{parser, parsers::name},
    lexer::{parsers::naked_text, results::builder::Builder},
    Cursor, End, Parsed,
};

use super::{
    indent::{self, Indents},
    mutable_field_assigner,
};

pub const KEY: &str = "named-entry";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return &KEY;
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        let mut result = Token::new();

        let key = name::Parser::Parse_At(cursor);
        let mut indent_increased = false;
        match key {
            Parsed::Token(key) => {
                result = result.prop("key", key);
                match indent::Parse_Opt_Or_Skip_At(cursor) {
                    Indents::Increase(token) => {
                        result = result.child(token);
                        indent_increased = true;
                    }
                    Indents::Decrease(_) => {
                        return result.end();
                    }
                    Indents::Current(_) => {
                        return result.end();
                    }
                    _ => {}
                }

                let operator = mutable_field_assigner::Parser::Parse_At(cursor);
                match operator {
                    Parsed::Token(operator) => {
                        result = result.prop("operator", operator);
                        match indent::Parse_Opt_Or_Skip_At(cursor) {
                            Indents::Increase(token) => {
                                result = result.child(token);
                            }
                            Indents::Current(token) => {
                                if !indent_increased {
                                    return result.end();
                                } else {
                                    result = result.child(token);
                                }
                            }
                            Indents::Decrease(_) => {
                                return result.end();
                            }
                            _ => {}
                        }

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
