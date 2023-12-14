pub mod attribute_expression;
pub mod entry_expression;

use crate::parser::tokens::splay_mods;

use self::invocation::identifier::lookup;

splay_mods! {
    expression: [literal, invocation]
}
