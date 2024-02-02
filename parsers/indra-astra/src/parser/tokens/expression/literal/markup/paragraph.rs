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
        let mut ws_only = true;
        loop {
            if cursor.is_eof() {
                break;
            }

            if let Some(escape) = escape_sequence::Parser::Try_Parse_At(cursor) {
                result.add_child(escape);
                ws_only = false;
            } else {
                match cursor.curr() {
                    '\n' => {
                        cursor.read();
                        match cursor.curr() {
                            '\n' => {
                                break;
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    _ => {
                        if cursor.curr_is_ws() {
                            cursor.skip_ws();
                            if ws_only {
                                start = cursor.curr_pos();
                            }
                            continue;
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
            }
        }

        return result.end();
    }
}
