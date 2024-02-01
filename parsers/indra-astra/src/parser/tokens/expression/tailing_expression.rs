use crate::parser::tokens::{
    expression::{
        identifier, invocation,
        literal::{self, markup, primitive, structure::closure},
    },
    splay,
};

splay! {
    tailing_expression: [invocation, identifier]
}
