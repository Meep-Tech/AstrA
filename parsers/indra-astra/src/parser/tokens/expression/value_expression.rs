use crate::parser::{
    cursor::Cursor,
    results::token::Token,
    tokens::{
        expression::{
            assignment, attribute_expression, invocation,
            literal::{
                markup::{paragraph, sentence, word},
                primitive,
            },
        },
        token,
        whitespace::indent,
    },
    Parser as _,
};

token! {
    #expression
    value_expression => |cursor: &mut Cursor| {
        cursor.save();
        if let Some(assignment) = assignment::Parser::Try_Parse_At(cursor) {
            if assignment.prop("operator").is_some() {
                return End::As_Variant(KEY, Parsed::Pass(assignment));
            } else {
                cursor.restore();
            }
        } else {
            cursor.pop();
        }

        cursor.skip_spacing();
        let first_element = match _try_to_read_value_expression_element(cursor) {
            Some(token) => token,
            None => return End::None,
        };
        let mut indent_increase = None;

        cursor.save();
        match indent::Parse_Opt_At(cursor) {
            indent::Indents::Current(_) => {
                cursor.restore();
                return End::As_Variant(KEY, Parsed::Pass(first_element))
            },
            indent::Indents::Increase(ident) => {
                indent_increase = Some(ident);
            },
            indent::Indents::Decrease(_) => {
                cursor.restore();
                return End::As_Variant(KEY, Parsed::Pass(first_element))
            },
            indent::Indents::None => {
              cursor.pop();
              cursor.skip_spacing();
            },
            indent::Indents::Error(err) => {
              cursor.pop();
              return End::Error_In_Child(KEY, err);
            },
        }

        let second_element = _try_to_read_value_expression_element(cursor);

        match second_element {
          None => {
            if indent_increase.is_some() {
              cursor.restore();
            }

            return End::As_Variant(KEY, Parsed::Pass(first_element))
          },
          Some(second_element) => {
            if indent_increase.is_some() {
              cursor.pop();
            }

            match paragraph::Parser::Try_Parse_At(cursor) {
                Some(token) => {
                    let start = first_element.start;
                    let mut paragraph = token.to_builder();
                    let mut sentence = paragraph.children.as_ref().unwrap()[0].to_builder();
                    sentence.prepend_child(second_element);
                    if let Some(indent) = indent_increase {
                        sentence.prepend_child(indent);
                    }
                    sentence.prepend_child(first_element);
                    paragraph.children.as_mut().unwrap()[0] = sentence.build_at(start);
                    return End::As_Variant(KEY, Parsed::Pass(paragraph.build_at(start)));
                },
                None => {
                    let paragraph = Token::Of_Type::<paragraph::Parser>().start(first_element.start);
                    let mut sentence = Token::Of_Type::<sentence::Parser>().start(first_element.start);
                    sentence.add_child(first_element);
                    if let Some(indent) = indent_increase {
                        sentence.add_child(indent);
                    }
                    sentence.add_child(second_element);
                    return paragraph.child(sentence.end(
                        cursor.prev_non_ws_pos(),
                    ).build()).end(cursor.prev_non_ws_pos()).to_end();
                }
            };
          }
        }
    }
}

pub fn _try_to_read_value_expression_element(cursor: &mut Cursor) -> Option<Token> {
    match invocation::prefixed::Parser::Try_Parse_At(cursor) {
        None => match primitive::Parser::Try_Parse_At(cursor) {
            None => match word::Parser::Try_Parse_At(cursor) {
                None => None,
                Some(token) => Some(token),
            },
            Some(token) => Some(token),
        },
        Some(token) => Some(token),
    }
}
