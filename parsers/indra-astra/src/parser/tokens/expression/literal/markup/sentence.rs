use crate::parser::{
    opt_child, req_child,
    tokens::{
        attribute,
        expression::{
            literal::{
                escape,
                markup::{sentence, word},
                primitive::number,
            },
            tailing_expression,
        },
        token,
    },
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

      while cursor.curr_is_ws() {
        if cursor.is_eof() || cursor.curr() == '\n' {
          break;
        }

        cursor.skip();
        if only_ws {
          start = cursor.curr_pos();
        }
      }

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
            result.add_child(
              Token::With_Name("period")
              .tag(word::KEY)
              .tag("punctuation")
              .build_from(
                cursor.prev_pos(),
                cursor.prev_pos()
              )
            );
            break;
          } else if cursor.next_is('.') {
            opt_child!([cursor] tailing_expression => result
              ?: break;
              !: { cursor.read(); continue; };
            );
          }
        }
        '\\' => {
          req_child!([cursor] escape => result);
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
          req_child!([cursor] attribute::tag => result);
        }
        // alias
        '|' => {
          req_child!([cursor] attribute::tag => result);
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

    result.start(start).to_end()
  }
}
