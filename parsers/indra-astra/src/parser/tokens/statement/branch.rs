use super::{assignment, expression};
use crate::parser::tokens::token;

token! {
    branch => |cursor: &mut Cursor| {
        match assignment::Parser::Parse_Opt_At(cursor) {
            Parsed::Pass(token) => End::Token_Variant(&KEY, token),
            Parsed::Fail(_) => End::As_Variant(&KEY, expression::Parser::Parse_At(cursor)),
        }
    }
}
