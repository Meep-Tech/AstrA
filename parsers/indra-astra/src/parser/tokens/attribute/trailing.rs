use crate::parser::tokens::{
    attribute::{self, trailing},
    token,
};

token! {
    trailing_attributes => |cursor: &mut Cursor| {
        let mut result = Token::Of_Type::<trailing::Parser>();

        loop {
          while cursor.curr_is_ws() {
            if cursor.curr() == '\n' || cursor.is_eof() {
              break;
            }
            cursor.skip();
          }

          if let Some(attribute) = attribute::Parser::Try_Parse_At(cursor) {
            result.add_child(attribute);
          } else {
            break;
          }
        }

        if result.children.is_none() || result.children.as_ref().unwrap().is_empty() {
          return End::None;
        } else {
          return result.end(cursor.prev_non_ws_pos()).to_end();
        }
    }
}
