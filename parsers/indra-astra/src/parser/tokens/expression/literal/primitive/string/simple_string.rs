use crate::parser::tokens::token;

token! {
    simple_string => |cursor: &mut Cursor| End::Missing("start-delimiter", "\'", &cursor.curr_str())
}
