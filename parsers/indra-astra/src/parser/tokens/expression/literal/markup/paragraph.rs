use std::f32::consts::E;

use crate::parser::tokens::{
    expression::{
        invocation::lookup::{dot_lookup, slash_lookup},
        literal::{
            escape::escape_sequence,
            markup::{paragraph, sentence},
        },
    },
    token,
    whitespace::indent,
};

token! {
    paragraph => |cursor: &mut Cursor| {
        let mut result = Token::Of_Type::<paragraph::Parser>();
        let mut start = cursor.curr_pos();
        let initial_indent = cursor.curr_indent();
        let mut ws_only = true;

        'pg: loop {
            if cursor.is_eof() {
                break;
            }

            if let Some(escape) = escape_sequence::Parser::Try_Parse_At(cursor) {
                result.add_child(escape);
                ws_only = false;
            } else {
                if cursor.curr_is_ws() {
                    let mut found_nl = false;
                    let mut prev_was_nl = false;

                    cursor.save();
                    while cursor.curr_is_ws() {
                        if prev_was_nl {
                            if cursor.curr_is('\n') {
                                cursor.restore();
                                break 'pg;
                            } else {
                                prev_was_nl = false;
                            }
                        } else {
                            if cursor.curr_is('\n') {
                                prev_was_nl = true;
                                found_nl = true;
                            }
                        }

                        cursor.skip();
                    }

                    if ws_only {
                        start = cursor.curr_pos();
                    }

                    if found_nl && cursor.curr_indent() <= initial_indent  {
                        cursor.restore();
                        break;
                    } else {
                        cursor.pop();
                        continue;
                    }
                } else {
                    if let Some(sentence) = sentence::Parser::Try_Parse_At(cursor) {
                        result.add_child(sentence);
                        ws_only = false;
                    } else {
                        End::Unexpected("in-paragraph", &cursor.slice(start, cursor.prev_pos()));
                    }
                }
            }
        }

        if result.children.is_none() {
            return End::None;
        } else {
            return result.end(cursor.prev_non_ws_pos()).to_end();
        }
    }
}
