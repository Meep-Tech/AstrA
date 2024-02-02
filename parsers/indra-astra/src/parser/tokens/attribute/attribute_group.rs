use crate::parser::tokens::{attribute::attribute_group, token};

token! {
    attribute_group => |cursor: &mut Cursor| {
        let initial_indent = cursor.curr_indent();
        let mut result = Token::Of_Type::<attribute_group::Parser>();
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
                  End::ToDo("read as tag");
                }
                '|' => {
                  End::ToDo("read as alias");
                }
                '>' => {
                  End::ToDo("read as input, input-tag, or output-tag");
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
          return result.end();
        }
    }
}
