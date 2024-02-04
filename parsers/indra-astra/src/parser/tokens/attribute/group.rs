use crate::parser::tokens::{
    attribute::{alias, group, input, tag},
    token,
};

token! {
    attribute_group => |cursor: &mut Cursor| {
        let initial_indent = cursor.curr_indent();
        let mut result = Token::Of_Type::<group::Parser>();
        loop {
            if cursor.is_eof() {
                break;
            } else if cursor.curr_is_ws() {
               cursor.skip_ws();
               if !cursor.indent().is_reading && cursor.curr_indent() > initial_indent {
                  return End::Indent_Mismatch(
                    "preceding-tags",
                    initial_indent,
                    cursor.curr_indent()
                  );
               }
               continue;
            } else {
              match cursor.curr() {
                '#' => {
                  let tag_result = tag::Parser::Parse_At(cursor);
                  match tag_result {
                   Parsed::Pass(token) => {
                      result.add_child(token);
                      continue;
                    },
                    Parsed::Fail(err) => {
                      return End::Error_In_Child_Of(result, err);
                    }
                  }
                }
                '|' => {
                  let alias_result = alias::Parser::Parse_At(cursor);
                  match alias_result {
                   Parsed::Pass(token) => {
                      result.add_child(token);
                      continue;
                    },
                    Parsed::Fail(err) => {
                      return End::Error_In_Child_Of(result, err);
                    }
                  }
                }
                '>' => {
                  let tag_result = tag::Parser::Parse_Opt_At(cursor);
                  match tag_result {
                   Parsed::Pass(token) => {
                      result.add_child(token);
                      continue;
                    },
                    Parsed::Fail(tag_err) => {
                      let input_result = input::Parser::Parse_At(cursor);
                      match input_result {
                       Parsed::Pass(token) => {
                          result.add_child(token);
                          continue;
                        },
                        Parsed::Fail(input_err) => {
                          return Error::Missing_Choice_In(result, vec!["tag", "input"], vec![tag_err, input_err]);
                        }
                      }
                    }
                  }
                }
                _ => {
                  break;
                }
              }
            }
        }

        if result.children.is_none() {
          return End::None;
        } else {
          let prev_non_ws_pos = cursor.prev_non_ws_pos();
          return End::Match(result.end(prev_non_ws_pos));
        }
    }
}
