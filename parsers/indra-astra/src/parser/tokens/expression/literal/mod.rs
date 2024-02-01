use crate::parser::tokens::{
    expression::literal::structure::{closure, tree},
    splay_mods,
};
pub mod escape;

splay_mods! {
    literal: [primitive, closure, markup, tree]
    subs: [primitive, structure, markup]
}
