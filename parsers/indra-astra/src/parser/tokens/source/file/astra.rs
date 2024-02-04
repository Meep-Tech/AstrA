use crate::parser::{
    results::span::Span,
    tokens::{
        attribute::group,
        expression::{
            assignment::{self, entry, func, var},
            invocation,
            literal::{
                markup,
                primitive::{self, number},
                structure::tree,
            },
        },
        source::file::{self, data},
        token,
    },
};

token! {
  astra => |cursor: &mut Cursor| {
    match cursor.file_type() {
      fs::Type::AstrA => {},
      fs::Type::Unknown => {},
      _ => {
        return End::Mismatch("file-type",
          &format!("{:?}", fs::Type::AstrA),
          &format!("{:?}", cursor.file_type())
        )
      },
    }

    let initial_indent = cursor.curr_indent();
    let preceeding_attributes: Option<Token> = match group::Parser::Parse_Opt_At(cursor) {
        Parsed::Pass(attribute_group) => Some(attribute_group),
        Parsed::Fail(_) => None,
    };
    cursor.skip_ws();

    if cursor.curr_indent() < initial_indent {
        return End::Indent_Mismatch(
            "entry",
            initial_indent,
            cursor.curr_indent(),
        );
    } else {
        if let Some(prim_data) = primitive::Parser::Try_Parse_At(cursor) {
            if prim_data.tag(number::KEY) {
                return End::ToDo("Check for data math expression");
            } else {
                return End::ToDo("Check for data string expression");
            }
        } else if let Some(struct_data) = data::Parser::Try_Parse_At(cursor) {
            return End::ToDo("Check for data strut literal expression");
        } else if let Some(prefixed_expression) = invocation::prefixed::Parser::Try_Parse_At(cursor) {
            return End::ToDo("Check for prefixed expression (assume strux)");
        } else if let Some(tree) = tree::Parser::Try_Parse_At(cursor) {
            let mut file = Token::Of_Type::<data::Parser>();
            match preceeding_attributes {
                Some(attributes) => {
                  file.add_child(attributes);
                },
                None => {}
            };
            file.set_prop("value", tree);

            return End::Match(file);
        } /*else if let Some(first_assignment) = assignment::Parser::Try_Parse_At(cursor) {
            let mut first_entry = first_assignment.to_builder();
            if first_assignment.tag(entry::KEY) {
              // read rest as tree
              let mut file = Token::Of_Type::<data::Parser>();
              match preceeding_attributes {
                Some(attributes) => {
                  // add the first attributes to the first entry
                  first_entry.prepend_child(attributes);
                },
                None => {}
              };

              let tree_result = if !cursor.is_eof() {tree::Parser::Parse_Opt_At(cursor)} else {Parsed::Fail(None)};
              match tree_result {
                Parsed::Pass(tree) => {
                  // combine first assignment and tree
                  let mut tree_builder = tree.to_builder();
                  tree_builder.prepend_child(first_entry.build(first_assignment.start, first_assignment.end));
                  file.set_prop("value", tree_builder.build(first_assignment.start, cursor.prev_pos()));
                }
                Parsed::Fail(err) => {
                  if let Some(err) = err {
                    return End::Error_In_Child_Of(file, Some(err));
                  } else {
                    let tree = Token::Of_Type::<tree::Parser>().child(first_entry.build(first_assignment.start, first_assignment.end));
                    file.set_prop("value", tree.build(first_assignment.start, first_assignment.end));
                  }
                }
              }

              return file.end();
            } else if first_assignment.tag(var::KEY) || first_assignment.tag(func::KEY) {
                return End::ToDo("Check for function prox expression");
            } else if first_assignment.tag(entry::ordered::KEY) {
                return End::ToDo("Check for strux ordered list");
            } else {
                return End::ToDo("Check for assignment prox expression");
            }
        } */else if let Some(markup_view) = markup::Parser::Try_Parse_At(cursor) {
            let mut markup_file = Token::Of_Type::<file::markup::Parser>();
            match preceeding_attributes {
                Some(attributes) => {
                  markup_file.add_child(attributes);
                },
                None => {}
            };
            markup_file.set_prop("body", markup_view);

            return End::Match(markup_file);
        } else if cursor.is_eof() {
          if preceeding_attributes.is_some() {
            return End::ToDo("Check tags for filetype");
          }

          return End::ToDo("Check for empty file expression");
        }

        return End::Unexpected(
          &"start-of-file",
          &cursor.slice(
          match preceeding_attributes {
            Some(attributes) => attributes.start(),
              None => 0,
            },
            cursor.curr_pos()
          )
        );
    }
  }
}
