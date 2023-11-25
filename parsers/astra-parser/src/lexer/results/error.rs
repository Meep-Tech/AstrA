use crate::{
    lexer::results::error_builder::ErrorBuilder, lexer::results::token::Token,
    lexer::results::token_builder::TokenBuilder, End, Parsed,
};
use std::collections::{HashMap, HashSet};

pub struct ChildOrError {
    pub child: Option<Token>,
    pub err: Option<Error>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Error {
    pub name: String,
    pub text: Option<String>,
    pub tags: Option<HashSet<String>>,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Parsed>,
    pub props: Option<HashMap<String, Parsed>>,
}

impl Error {
    pub(crate) fn new(key: &str) -> ErrorBuilder {
        return ErrorBuilder::new(key);
    }

    pub fn in_child(parent: TokenBuilder, err: Error) -> Option<End> {
        let mut parent_err = ErrorBuilder {
            name: "incomplete".to_string(),
            text: None,
            tags: parent.tags,
            children: Some(
                parent
                    .children
                    .unwrap_or(Vec::new())
                    .into_iter()
                    .map(|c| Parsed::Token(c))
                    .collect(),
            ),
            props: None,
        };

        parent_err = parent_err.child(Parsed::Error(err));

        return Some(End::Fail(parent_err));
    }

    pub fn in_prop(parent: Token, key: &str, err: Error) -> Option<End> {
        let mut parent_err = ErrorBuilder {
            name: "incomplete".to_string(),
            text: None,
            tags: parent.tags,
            children: Some(
                parent
                    .children
                    .into_iter()
                    .map(|c| Parsed::Token(c))
                    .collect(),
            ),
            props: Some(
                parent
                    .props
                    .unwrap_or(HashMap::new())
                    .into_iter()
                    .map(|(k, v)| (k, Parsed::Token(v)))
                    .collect(),
            ),
        };

        parent_err = parent_err.prop(key, Parsed::Error(err));

        return Some(End::Fail(parent_err));
    }
}
