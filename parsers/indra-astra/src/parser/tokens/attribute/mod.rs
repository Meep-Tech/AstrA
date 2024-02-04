use crate::{parser::tokens::attribute, runtime::nodes::Key};

use super::{splay_mods, token};

pub mod alias;
pub mod group;
pub mod input;
pub mod tag;
pub mod trailing;

token! {
    attribute => |cursor: &mut Cursor| {
        match cursor.curr() {
            '#' => {
                return End::As::<tag::Parser>(KEY, cursor);
            }
            '|' => {
                return End::As::<alias::Parser>(KEY, cursor);
            }
            '>' => {
                match tag::Parser::Try_Parse_At(cursor) {
                    Some(token) => {
                        return End::As_Variant(KEY, Parsed::Pass(token));
                    }
                    None => {
                        return End::As::<input::Parser>(KEY, cursor);
                    }
                }
            }
            _ => {
                return End::None;
            }
        }
    },
    subs: [tag, alias, input]
}
