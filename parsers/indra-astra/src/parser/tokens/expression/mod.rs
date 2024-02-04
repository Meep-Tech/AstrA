use crate::parser::tokens::{
    expression::{
        self,
        literal::{markup::word, primitive},
    },
    splay_mods,
    whitespace::indent,
};

use super::token;

pub mod attribute_expression;
pub mod tailing_expression;
pub mod value_expression;

splay_mods! {
    expression: [assignment, invocation, identifier, literal]
}
