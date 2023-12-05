use crate::{
    lexer::results::builder::Builder,
    lexer::{
        parser,
        parsers::{indent, mutable_field_assigner, name, statement::expression},
    },
    lexer::{parsers::indent::Indents, results::token::Token},
    tests::lexer::parsers::test::Testable,
    Cursor, End, Parsed,
};

pub const KEY: &str = "named-entry";

pub struct Parser {}
impl parser::Parser for Parser {
    fn name(&self) -> &'static str {
        &KEY
    }

    fn as_tests(&self) -> Option<&dyn Testable> {
        Some(self)
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        let mut result = Token::new();

        let key = name::Parser::Parse_At(cursor);
        let mut indent_increased = false;
        match key {
            Parsed::Pass(key) => {
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
                    Parsed::Pass(operator) => {
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

                        let value = expression::Parser::Parse_At(cursor);

                        match value {
                            Parsed::Pass(value) => {
                                return result.prop("value", value).end();
                            }
                            Parsed::Fail(error) => {
                                return End::Error_In_Prop(result, "value", error)
                            }
                        }
                    }
                    Parsed::Fail(error) => return End::Error_In_Prop(result, "operator", error),
                }
            }
            Parsed::Fail(error) => return End::Error_In_Prop(result, "key", error),
        }
    }
}
