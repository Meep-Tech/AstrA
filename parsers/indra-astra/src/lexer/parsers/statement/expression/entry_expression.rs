use crate::lexer::parsers::{
    splay,
    statement::expression::{
        invocation::{self, identifier::lookup},
        literal::{
            self, markup, primitive,
            structure::{self, closure, tree},
        },
    },
};

splay! {
    entry_expression: [primitive, lookup, closure, markup, tree]
}
