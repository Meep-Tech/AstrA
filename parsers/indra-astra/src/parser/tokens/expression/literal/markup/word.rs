use crate::parser::{
    req_child,
    tokens::{
        attribute::{alias, tag},
        expression::{
            invocation::lookup::dot_lookup,
            literal::{escape, markup::word},
            tailing_expression,
        },
        token,
    },
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
           if cursor.is_at(result.start.unwrap()) {
              return End::None;
            } else if cursor.next_is_ws() {
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
          '|' => {
            if cursor.is_at(result.start.unwrap()) {
              return End::None;
            } else {
              req_child!([cursor] alias => result);
            }
          }
          '#' => {
            if cursor.is_at(result.start.unwrap()) {
              return End::None;
            } else {
              req_child!([cursor] tag => result);
            }
          }
          '\\' => {
            req_child!([cursor] escape => result);
          }
          '>' => {
            todo!("input if first letter is >")
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

    if cursor.is_at(result.start.unwrap()) {
      return End::None;
    } else if result.len() == 1 {
      match result.children {
        Some(mut children) => {
          let first = children.pop().unwrap();
          return End::As_Variant(KEY, Parsed::Pass(first));
        }
        None => {
          unreachable!()
        }
      }
    } else {
      return result.to_end();
    }
  }
}
