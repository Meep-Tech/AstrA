use crate::parser::{cursor::Cursor, results::end::End, tokens::token};

pub fn is_allowed_symbol(c: char) -> bool {
    match c {
        '$' | '@' => true,
        _ => false,
    }
}

pub fn is_allowed_in_middle_without_repeating(c: char) -> bool {
    match c {
        '-' | '+' | '%' | '^' | '~' => true,
        _ => false,
    }
}

pub fn is_allowed_in_middle_with_repeating(c: char) -> bool {
    match c {
        '_' => true,
        _ => false,
    }
}

token! {
    name => |cursor: &mut Cursor| {
        let start = cursor.pos;
        let mut is_pure_numeric: bool;
        let mut curr: char = cursor.curr();

        if curr.is_numeric() {
            is_pure_numeric = true;
        } else if curr.is_alphabetic() || is_allowed_symbol(curr) {
            is_pure_numeric = false;
        } else {
            return End::Mismatch(
               "first_letter",
                "alphanumeric or allowed symbol: $, @",
                &cursor.curr_str(),
            );
        }

        cursor.read();

        let mut last_lone_char: Option<char> = None;
        loop {
            if cursor.is_eof() {
                cursor.read();
                return _check_end_is_valid(is_pure_numeric, cursor, start);
            }

            curr = cursor.curr();

            if curr.is_alphanumeric()
                || is_allowed_symbol(curr)
                || is_allowed_in_middle_with_repeating(curr)
            {
                if is_pure_numeric && !(curr.is_numeric() || curr == '_') {
                    is_pure_numeric = false;
                }
            } else if is_allowed_in_middle_without_repeating(curr) {
                if let Some(last) = last_lone_char {
                    if last == curr {
                        return End::Unexpected(
                          "repeat_lone_symbol",
                            &cursor.slice(cursor.pos - 1, cursor.pos),
                        );
                    }
                }

                // skip the last_lone_char = None call.
                last_lone_char = Some(curr);
                cursor.read();
                continue;
            } else {
                return _check_end_is_valid(is_pure_numeric, cursor, start);
            }

            last_lone_char = None;
            cursor.read();
        }
    },
    tests:
        unit!(["Alphabetic"]
            : "abc"
            => Parsed::Pass(Token::Of_Type::<Self>().partial().build(0, 2)))
}

fn _check_end_is_valid(is_pure_numeric: bool, cursor: &mut Cursor, start: usize) -> End {
    if !is_pure_numeric {
        if is_allowed_in_middle_with_repeating(cursor.prev())
            || is_allowed_in_middle_without_repeating(cursor.prev())
        {
            return End::Unexpected("last_letter", &cursor.slice(cursor.pos - 1, cursor.pos));
        }
        return End::Token();
    } else {
        return End::Unexpected("pure_numeric_key", &cursor.slice(start, cursor.pos));
    }
}
