use crate::parser::tokens::{
    expression::{
        identifier, invocation,
        literal::{self, markup, primitive, structure::closure},
    },
    splay,
};

splay! {
    attribute_expression: [invocation, identifier, primitive, closure, markup]
}
