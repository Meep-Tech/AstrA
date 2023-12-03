use crate::lexer::{
    cursor::Cursor,
    parser,
    parsers::indent::{self, Indents},
    results::{builder::Builder, end::End, parsed::Parsed, token::Token},
};

use super::branch;

pub struct Parser {}
impl parser::Parser for Parser {
    fn get_name(&self) -> &'static str {
        return "tree";
    }

    fn rule(&self, cursor: &mut Cursor) -> End {
        let mut result = Token::new();
        match indent::Parse_At(cursor) {
            Indents::Increase(token) => {
                result = result.child(token);
            }
            Indents::Decrease(_) => {
                return End::Unexpected("initial-indent-decrease", &cursor.curr_str())
            }
            Indents::Current(_) => {
                return End::Unexpected("initial-current-indent", &cursor.curr_str())
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
