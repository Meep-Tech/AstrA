use crate::parser::tokens::{
    expression::assignment,
    expression::{self},
    statement::branch,
    token,
};

token! {
    branch => |cursor: &mut Cursor| {
        let branch = Token::Of_Type::<branch::Parser>();

        match expression::Parser::Parse_Opt_At(cursor) {
            Parsed::Pass(token) => branch.child(token).end(),
            Parsed::Fail(e) => End::Error_In_Child_Of(branch, e)
        }
    }
}
