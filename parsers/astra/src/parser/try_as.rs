use crate::token::{Attribute, Code, Source, Token, TokenBuilder, Type};

use super::cursor::Cursor;

pub enum Result {
    Match(Token),
    None,
}

pub fn source(cursor: Cursor) -> Result {
    match cursor.config().source_type() {
        Source::Code(src_code) => match src_code {
            Code::Axa => axa_code(cursor),
            Code::Stx => stx_code(cursor),
            Code::Prx => prx_code(cursor),
            Code::Blx => blx_code(cursor),
            Code::Arc => arc_file(cursor),
            Code::Mot => mot_file(cursor),
            Code::Cmd => cmd_file(cursor),
        },
        Source::Command => command(cursor),
    }
}

pub fn axa_code(cursor: Cursor) -> Result {
    // look for a starting entry.
    // check first for preceding attributes.
    let mut initial_entry = Token::new(Type::Unknown, cursor.index());
    let preceding_attributes = attribute_group(cursor);
    if let Result::Match(attributes) = preceding_attributes {
        initial_entry.add_child(attributes);
    }

    // check for a line prefix.
    if cursor.indent().is_reading() {
        //todo!("line prefix")
        // also check for ordered/hybrid line prefix in here.
    }

    // check for a key
    if let Result::Match(key) = entry_key(cursor) {
        initial_entry.set_prop("key", key);
    }

    return Result::Match(initial_entry);
}

pub fn attribute_group(cursor: Cursor) -> Result {
    let container = None;
    loop {
        cursor.skip_ws_with_same_indent();
        let start = cursor.index() + 1;
        if let Result::Match(tag) = tag(cursor) {
            if container.is_none() {
                container = Some(Token::new(Type::Attribute(Attribute::Group), start));
            }

            container.unwrap().add_child(tag);
        } else if let Result::Match(alias) = alias(cursor) {
            if container.is_none() {
                container = Some(Token::new(Type::Attribute(Attribute::Group), start));
            }

            container.unwrap().add_child(alias);
        } else {
            break;
        }
    }

    if !container.is_some() {
        Result::None
    } else {
        Result::Match(container.unwrap())
    }
}

pub fn stx_code(cursor: Cursor) -> Result {
    todo!("stx_code")
}

pub fn prx_code(cursor: Cursor) -> Result {
    todo!("prx_code")
}

pub fn blx_code(cursor: Cursor) -> Result {
    todo!("blx_code")
}

pub fn arc_file(cursor: Cursor) -> Result {
    todo!("arc_file")
}

pub fn mot_file(cursor: Cursor) -> Result {
    todo!("mot_file")
}

pub fn cmd_file(cursor: Cursor) -> Result {
    todo!("cmd_file")
}

pub fn command(cursor: Cursor) -> Result {
    todo!("command")
}
