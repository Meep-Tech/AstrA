use crate::parser::tokens::{
    expression::{
        identifier, invocation,
        literal::{self, markup, primitive, structure::closure},
    },
    splay,
};

splay! {
    #expression
    tailing_expression: [invocation, identifier]
}
