use crate::parser::tokens::splay_mods;
pub mod escape;

splay_mods! {
    literal: [primitive, structure, identifier, text]
}
