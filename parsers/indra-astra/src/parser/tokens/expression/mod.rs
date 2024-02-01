use crate::parser::tokens::splay_mods;

pub mod attribute_expression;
pub mod tailing_expression;

splay_mods! {
    expression: [assignment, invocation, identifier, literal]
}
