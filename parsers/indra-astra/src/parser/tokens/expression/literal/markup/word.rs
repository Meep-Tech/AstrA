use crate::parser::tokens::{
    expression::{invocation::lookup::dot_lookup, literal::markup::word, tailing_expression},
    token,
};

token! {
  word => |cursor: &mut Cursor| {
    let mut result = Token::Of_Type::<word::Parser>()
      .start(cursor.curr_pos());

    loop {
      if cursor.is_eof() {
        break;
      }

      let curr = cursor.curr();
      if curr.is_whitespace() {
        break;
      } else if curr.is_alphanumeric() {
        cursor.read();
        continue;
      } else {
        match curr {
          '.' => {
            if cursor.next_is_ws() {
              break;
            } else if cursor.next_is('.') {
              break;
            } else {
              let tailing_expression_result = tailing_expression::Parser::Try_Parse_At(cursor);
              match tailing_expression_result {
                Some(token) => {
                  result.add_child(token);
                  break;
                },
                None => {
                  cursor.read();
                  continue;
                }
              }
            }
          }
          '\\' => {
            todo!("escape")
          }
          '{' => {
            todo!("metadata")
          }
          '[' => {
            todo!("block")
          }
          '(' => {
            todo!("addendum")
          }
          _ => {
            cursor.read();
            continue;
          }
        }
      }

    }

    if result.start.unwrap() == cursor.curr_pos() {
      return End::None;
    } else {
      return result.to_end();
    }
  }
}
