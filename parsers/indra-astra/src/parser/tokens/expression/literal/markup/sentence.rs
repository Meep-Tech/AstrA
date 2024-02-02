use crate::parser::tokens::{
    expression::literal::{
        markup::{sentence, word},
        primitive::number,
    },
    token,
};

token! {
  sentence => |cursor: &mut Cursor| {
    let mut result = Token::Of_Type::<sentence::Parser>();
    let mut start = cursor.curr_pos();
    let mut only_ws = false;
    loop {
      if cursor.is_eof() {
        break;
      }

      if cursor.curr_is_ws() {
        cursor.skip_ws();
        if only_ws {
          start = cursor.curr_pos();
        }
      } else {
        only_ws = false;

        // try to parse number
        if cursor.curr().is_numeric() {
          let number_result = number::Parser::Try_Parse_At(cursor);
          match number_result {
            Some(token) => {
              result.add_child(token);
              continue;
            }
            None => {}
          }
        }

        // check for special characters
        match cursor.curr() {
          // end of sentence
          '.' => {
            if cursor.next_is_ws() {
              cursor.read();
              break;
            }
          }
          // end of sentence
          '\n' => {
            break;
          }
          // metadata
          '{' => {
            todo!("metadata")
          }
          // block
          '[' => {
            todo!("block")
          }
          // addendum
          '(' => {
            todo!("addendum")
          }
          // tag
          '#' => {
            todo!("tag")
          }
          // alias
          '|' => {
            todo!("alias")
          }
          // italic and bold markup formatting
          '*' => {
            todo!("italic and bold")
          }
          // inline code markup formatting
          '`' => {
            todo!("inline code")
          }
          // strikethrough markup formatting
          '~' => {
            todo!("strikethrough")
          }
          // emoji
          ':' => {
            todo!("emoji")
          }
          '=' => {
            todo!("hilighted text")
          }
          '@' => {
            todo!("ref")
          }
          _ => {}
        }

        // fall-back to a word
        let word_result = word::Parser::Try_Parse_At(cursor);
        match word_result {
          Some(token) => {
            result.add_child(token);
            continue;
          }
          None => {}
        }
      }
    }

    result.start(start).end()
  }
}
