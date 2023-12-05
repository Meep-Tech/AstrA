use crate::{
    lexer::{
        cursor::Cursor,
        parser,
        parsers::{
            indent::{self, Indents},
            statement::branch,
        },
        results::{builder::Builder, end::End, parsed::Parsed, token::Token},
    },
    tests::lexer::parsers::test::Testable,
};

pub const KEY: &str = "tree";

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        &KEY
    }

    fn as_tests(&self) -> Option<&dyn Testable> {
        Some(self)
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        let mut result = Token::new();
        match indent::Parse_At(cursor) {
            Indents::Increase(token) => {
                result = result.child(token);
            }
            Indents::Current(token) => {
                result = result.child(token);
            }
            Indents::Decrease(_) => {
                return End::Unexpected("initial-indent-decrease", &cursor.curr_str())
            }
            _ => {}
        };

        loop {
            match branch::Parser::Parse_At(cursor) {
                Parsed::Pass(token) => {
                    result = result.child(token);
                    match indent::Parse_Opt_At(cursor) {
                        Indents::Current(token) => {
                            result = result.child(token);
                        }
                        _ => {
                            return result.end();
                        }
                    };
                }
                Parsed::Fail(error) => match error {
                    Some(error) => return End::Error_In_Child(result, error),
                    None => return result.end(),
                },
            }
        }
    }
}
