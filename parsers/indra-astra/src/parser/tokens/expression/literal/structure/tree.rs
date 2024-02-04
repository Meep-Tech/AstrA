use crate::parser::tokens::{
    statement::branch,
    token,
    whitespace::indent::{self, Indents},
};

token! {
    tree => |cursor: &mut Cursor| {
        let initial_indent = cursor.curr_indent();
        let mut result = Token::New();
        cursor.save();
        match indent::Parse_At(cursor) {
            Indents::Increase(token) => {
                result.add_child(token);
            }
            Indents::Decrease(_) => {
                cursor.restore();
                return End::None;
            }
            _ => {}
        };
        cursor.pop();

        loop {
            match branch::Parser::Parse_At(cursor) {
                Parsed::Pass(token) => {
                    result.add_child(token);
                    cursor.save();
                    match indent::Parse_Opt_At(cursor) {
                        Indents::Current(token) => {
                            result.add_child(token);
                        }
                        Indents::Decrease(token) => {
                            if cursor.curr_indent() < initial_indent {
                                cursor.restore();
                                break;
                            } else {
                                result.add_child(token);
                            }
                        }
                        _ => {
                            cursor.pop();
                            break;
                        }
                    };
                    cursor.pop();
                }
                Parsed::Fail(error) => match error {
                    Some(error) => return End::Error_In_Child_Of(result, Some(error)),
                    None => {
                        break;
                    },
                },
            }

            if cursor.is_eof() {
                break;
            }
        }

        match &result.children {
            Some(children) => {
                if children.is_empty() {
                    return End::None;
                } else if children.len() == 1 && children[0].tag(whitespace::indent::KEY) {
                    return End::None;
                }

                return End::Match(result.end(cursor.prev_non_ws_pos()));
            }
            None => {
                return End::None;
            }
        }

    }
}
