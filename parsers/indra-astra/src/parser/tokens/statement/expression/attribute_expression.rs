use crate::parser::tokens::{
    splay,
    statement::expression::{invocation, literal},
};

splay! {
    attribute_expression: [invocation, literal]
}
