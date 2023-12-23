use crate::parser::tokens::{
    statement::expression::{
        invocation::lookup::{dot_lookup, slash_lookup},
        literal::escape::escape_sequence,
    },
    token,
    whitespace::indent,
};

token! {
    text => |cursor: &mut Cursor| {
        let mut result = End::New();
        loop {
            if cursor.is_eof() {
                cursor.read();
                break;
            }
            if let Some(escape) = escape_sequence::Parser::Try_Parse_At(cursor) {
                result.add_child(escape);
            } else {
                match cursor.curr() {
                    '\n' => match indent::increase::Parser::Try_Parse_At(cursor) {
                        Some(token) => {
                            result.add_child(token);
                        }
                        None => {
                            return End::Token();
                        }
                    },
                    '.' => {
                        if cursor.curr().is_whitespace() && !cursor.next().is_whitespace() {
                            match dot_lookup::Parser::Parse_At(cursor) {
                                Parsed::Pass(child) => {
                                    result.add_child(child);
                                }
                                Parsed::Fail(error) => {
                                    return End::Unexpected_Child_Of(result, error)
                                }
                            }
                        }
                    }
                    '/' => {
                        if cursor.curr().is_whitespace() && !cursor.next().is_whitespace() {
                            match slash_lookup::Parser::Parse_At(cursor) {
                                Parsed::Pass(child) => {
                                    result.add_child(child);
                                }
                                Parsed::Fail(error) => {
                                    return End::Unexpected_Child_Of(result, error)
                                }
                            }
                        }
                    }
                    '{' => {
                        End::TODO();
                    }
                    '#' => {
                        End::TODO();
                    }
                    '|' => {
                        End::TODO();
                    }
                    _ => {
                        cursor.read();
                    }
                }
            }
        }

        return result.end();
    }
}
