use crate::parser::tokens::{
    attribute::{self, group},
    expression::{
        assignment::{self, entry, func, var},
        literal::primitive::{self, number},
    },
    splay_mods, token,
};

pub mod astra;
pub mod data;
pub mod markup;
pub mod mote;
pub mod prox;
pub mod r#trait;

token! {
  file => |cursor: &mut Cursor| {
        match cursor.file_type() {
            // ...dat
            fs::Type::Data(_) => End::As::<data::Parser>(&KEY, cursor),
            // ...mup
            fs::Type::Markup(_) => End::As::<markup::Parser>(&KEY, cursor),
            // ...mot
            fs::Type::Mote => End::As::<mote::Parser>(&KEY, cursor),
            // ...trt
            fs::Type::Trait(trait_file_type) => {
                match trait_file_type {
                    // .prx
                    fs::Trait::ProX => End::As::<prox::Parser>(&KEY, cursor),
                    // ...trt
                    _ => End::As::<r#trait::Parser>(&KEY, cursor),
                }
            }
            // ...
            fs::Type::AstrA | fs::Type::Unknown => {
                End::As::<astra::Parser>(&KEY, cursor)
            }
        }
    },
    subs: [data, markup, mote, prox, r#trait, astra]
}
