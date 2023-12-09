use crate::lexer::parsers::{
    parser,
    statement::branch,
    whitespace::indent::{self, Indents},
};

parser! {
    #testable,
    tree => |cursor: &mut Cursor| {
        let mut result = Token::New();
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
                    Some(error) => return End::Error_In_Child_Of(result, error),
                    None => return result.end(),
                },
            }
        }
    }
}
