use crate::parser::tokens::{
    expression::assignment,
    expression::{self, literal::structure::tree},
    statement::branch,
    token,
    whitespace::indent,
};

token! {
    branch => |cursor: &mut Cursor| {
        let initial_indent = cursor.curr_indent();
        let mut branch = Token::Of_Type::<branch::Parser>();

        let first_entry;
        match expression::value_expression::Parser::Parse_Opt_At(cursor) {
            Parsed::Pass(token) => {
                first_entry = token;
            },
            Parsed::Fail(e) =>{
                 return End::Error_In_Child_Of(branch, e);
            }
        }

        let first_end = cursor.prev_non_ws_pos();
        if cursor.is_eof() || cursor.curr_indent() <= initial_indent {
            return branch.child(first_entry).end(cursor.prev_non_ws_pos()).to_end();
        } else {
            match tree::Parser::Parse_At(cursor) {
                Parsed::Pass(token) => {
                    let mut tree = token.to_builder();
                    let start = first_entry.start;
                    let first_branch = Token::Of_Type::<branch::Parser>().child(first_entry).build_from(start, first_end);

                    tree.prepend_child(first_branch);
                    branch.add_child(tree.build_from(
                        start,
                        cursor.prev_non_ws_pos()
                    ));

                    return branch.end(cursor.prev_non_ws_pos()).to_end();
                },
                Parsed::Fail(e) => match e {
                    Some(e) => return End::Error_In_Child_Of(branch, Some(e)),
                    None => return branch.child(first_entry).end(cursor.prev_non_ws_pos()).to_end(),
                }
            }
        }
    }
}
