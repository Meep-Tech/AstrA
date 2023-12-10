use crate::lexer::parsers::{
    splay,
    statement::expression::{invocation, literal},
};

splay! {
    attribute_expression: [invocation, literal]
}
