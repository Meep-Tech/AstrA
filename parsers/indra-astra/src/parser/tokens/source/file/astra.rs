use crate::parser::{
    results::span::Span,
    tokens::{
        attribute::attribute_group,
        expression::{
            assignment::{self, entry, func, var},
            literal::{
                markup,
                primitive::{self, number},
            },
            prefixed_expression,
        },
        source::file::data,
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
    let preceeding_attributes: Option<Token> = match attribute_group::Parser::Parse_Opt_At(cursor) {
        Parsed::Pass(attribute_group) => Some(attribute_group),
        Parsed::Fail(_) => None,
    };
    cursor.skip_ws();

    if cursor.curr_indent() > initial_indent {
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
        } else if let Some(prefixed_expression) = prefixed_expression::Parser::Try_Parse_At(cursor) {
            return End::ToDo("Check for prefixed expression (assume strux)");
        } else if let Some(first_assignment) = assignment::Parser::Try_Parse_At(cursor) {
            if first_assignment.tag(entry::KEY) {
                return End::ToDo("Check for entry strux expression");
            } else if first_assignment.tag(var::KEY) || first_assignment.tag(func::KEY) {
                return End::ToDo("Check for function prox expression");
            } else {
                return End::ToDo("Check for assignment prox expression");
            }
        } else if let Some(markup_view) = markup::Parser::Try_Parse_At(cursor) {
            let mut markup_file = Token::Of_Type::<markup::Parser>();
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
