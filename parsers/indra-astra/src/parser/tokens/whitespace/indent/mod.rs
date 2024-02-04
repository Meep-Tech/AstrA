use crate::parser::{
    cursor::Cursor,
    results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
    tokens::token,
    Parser as _,
};

pub mod current;
pub mod decrease;
pub mod increase;

pub enum Indents {
    Increase(Token),
    Decrease(Token),
    Current(Token),
    Error(Error),
    None,
}

token! {
    indent => |cursor: &mut Cursor| {
        let start = cursor.curr_pos();
        if cursor.is_eof() {
            return End::None;
        }

        cursor.skip_ws();
        if start == cursor.curr_pos() || !cursor.indent().is_reading {
            return End::None;
        }

        if cursor.indent().curr > cursor.indent().prev() {
            End::New_Variant::<increase::Parser>(&KEY)
        } else if cursor.indent().curr < cursor.indent().prev() {
            End::New_Variant::<decrease::Parser>(&KEY)
        } else {
            End::New_Variant::<current::Parser>(&KEY)
        }
    }
}

#[allow(non_snake_case)]
pub fn Parse(input: &str) -> Indents {
    Match(Parser::Parse(input))
}

#[allow(non_snake_case)]
pub fn Parse_At(cursor: &mut Cursor) -> Indents {
    Match(Parser::Parse_At(cursor))
}

#[allow(non_snake_case)]
pub fn Parse_Opt(input: &str) -> Indents {
    Match(Parser::Parse_Opt(input))
}

#[allow(non_snake_case)]
pub fn Parse_Opt_At(cursor: &mut Cursor) -> Indents {
    Match(Parser::Parse_Opt_At(cursor))
}

#[allow(non_snake_case)]
pub fn Parse_Opt_Or_Skip(input: &str) -> Indents {
    Match(Parser::Get().parse_opt_or_skip(input))
}

#[allow(non_snake_case)]
pub fn Parse_Opt_Or_Skip_At(cursor: &mut Cursor) -> Indents {
    Match(Parser::Get().parse_opt_or_skip_at(cursor))
}

#[allow(non_snake_case)]
pub fn Try_Parse_At(cursor: &mut Cursor) -> Option<Indents> {
    match Match(Parser::Parse_At(cursor)) {
        Indents::Current(token) => Some(Indents::Current(token)),
        Indents::Increase(token) => Some(Indents::Increase(token)),
        Indents::Decrease(token) => Some(Indents::Decrease(token)),
        _ => None,
    }
}

#[allow(non_snake_case)]
pub fn Try_Parse(input: &str) -> Option<Indents> {
    match Match(Parser::Parse(input)) {
        Indents::Current(token) => Some(Indents::Current(token)),
        Indents::Increase(token) => Some(Indents::Increase(token)),
        Indents::Decrease(token) => Some(Indents::Decrease(token)),
        _ => None,
    }
}

#[allow(non_snake_case)]
pub fn Match(result: Parsed) -> Indents {
    match result {
        Parsed::Pass(token) => match token.name.as_str() {
            current::KEY => Indents::Current(token).into(),
            increase::KEY => Indents::Increase(token).into(),
            decrease::KEY => Indents::Decrease(token).into(),
            _ => Indents::Error(Error::New("unknown_indent_type").build_from(0, 0).unwrap()),
        },
        Parsed::Fail(error) => match error {
            Some(error) => Indents::Error(error),
            None => Indents::None,
        },
    }
}
