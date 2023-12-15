use crate::parser::tokens::{
    statement::branch,
    token,
    whitespace::indent::{self, Indents},
};

token! {
    tree => |cursor: &mut Cursor| {
        let mut result = Match::New();
        match indent::Parse_At(cursor) {
            Indents::Increase(token) => {
                result.add_child(token);
            }
            Indents::Current(token) => {
                result.add_child(token);
            }
            Indents::Decrease(_) => {
                return End::Unexpected("initial-indent-decrease", &cursor.curr_str())
            }
            _ => {}
        };

        loop {
            match branch::Token::Parse_At(cursor) {
                Parsed::Pass(token) => {
                    result.add_child(token);
                    match indent::Parse_Opt_At(cursor) {
                        Indents::Current(token) => {
                            result.add_child(token);
                        }
                        _ => {
                            return result.end();
                        }
                    };
                }
                Parsed::Fail(error) => match error {
                    Some(error) => return End::Error_In_Child_Of(result, error),
                    None => return result.end(),
                },
            }
        }
    }
}
