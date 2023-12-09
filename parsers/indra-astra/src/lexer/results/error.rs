use crate::{
    lexer::results::error_builder::ErrorBuilder, lexer::results::token::Token,
    lexer::results::token_builder::TokenBuilder, End, Parsed,
};
use std::collections::{HashMap, HashSet};

use crate::node::{Node, _EMPTY_KEYS, _EMPTY_TAGS};

use super::span::Span;

pub struct ChildOrError {
    pub child: Option<Token>,
    pub err: Option<Error>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Error {
    pub name: String,
    pub text: Option<String>,
    pub tags: Option<HashSet<String>>,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Parsed>,
    pub keys: Option<HashMap<String, usize>>,
}

impl Error {
    #[allow(non_snake_case)]
    pub(crate) fn New(key: &str) -> ErrorBuilder {
        return ErrorBuilder::new(key);
    }

    #[allow(non_snake_case)]
    pub fn None() -> End {
        End::Fail(
            ErrorBuilder::new("no-match-for-{}")
                .text("No match found.")
                .tag("none"),
        )
    }

    pub fn to_builder(self) -> ErrorBuilder {
        return ErrorBuilder {
            name: self.name,
            tags: self.tags,
            text: self.text,
            children: if !self.children.is_empty() {
                Some(self.children)
            } else {
                None
            },
            keys: self.keys,
        };
    }

    pub fn unexpected(key: &str, value: &str) -> End {
        End::Fail(
            Error::New(&["unexpected-", key, "-in-{}"].concat())
                .text(&format!("Unexpected: `{}`.", value))
                .tag("unexpected"),
        )
    }

    pub fn mismatch(key: &str, expected: &str, found: &str) -> End {
        End::Fail(
            Error::New(&["unexpected-", key, "-in-{}"].concat())
                .text(&format!(
                    "Expected: `{}`, but found: `{}`.",
                    expected, found
                ))
                .tag("missing"),
        )
    }

    pub fn missing(key: &str, expected: &str, found: &str) -> End {
        End::Fail(
            Error::New(&["missing-expected-", key, "-in-{}"].concat())
                .text(&format!(
                    "Expected: `{}`, but found: `{}`.",
                    expected, found
                ))
                .tag("missing"),
        )
    }

    pub fn in_child(parent: TokenBuilder, err: Option<Error>) -> End {
        let mut parent_err = ErrorBuilder {
            name: "incomplete-{}".to_string(),
            text: None,
            tags: parent.tags,
            children: Some(
                parent
                    .children
                    .unwrap_or(Vec::new())
                    .into_iter()
                    .map(|c| Parsed::Pass(c))
                    .collect(),
            ),
            keys: Some(parent.keys.unwrap_or(HashMap::new())),
        };

        parent_err = parent_err.tag("incomplete");
        parent_err = parent_err.child(Parsed::Fail(err));

        return End::Fail(parent_err);
    }

    pub fn in_prop(parent: TokenBuilder, key: &str, err: Option<Error>) -> End {
        let mut parent_err = ErrorBuilder {
            name: "incomplete-{}".to_string(),
            text: None,
            tags: parent.tags,
            children: Some(
                parent
                    .children
                    .unwrap_or(Vec::new())
                    .into_iter()
                    .map(|c| Parsed::Pass(c))
                    .collect(),
            ),
            keys: Some(parent.keys.unwrap_or(HashMap::new())),
        };

        parent_err = parent_err.tag("incomplete");
        parent_err = parent_err.prop(key, Parsed::Fail(err));

        return End::Fail(parent_err);
    }
}

impl Node<Parsed> for Error {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn tags(&self) -> &HashSet<String> {
        let hash_set = self.tags.as_ref();
        hash_set.unwrap_or(&_EMPTY_TAGS)
    }

    fn children(&self) -> Vec<&Parsed> {
        return self.children.iter().collect();
    }

    fn keys(&self) -> &HashMap<String, usize> {
        let hash_map = self.keys.as_ref();
        hash_map.unwrap_or(&_EMPTY_KEYS)
    }
}

impl Span<Parsed> for Error {
    fn start(&self) -> usize {
        return self.start;
    }

    fn end(&self) -> usize {
        return self.end;
    }
}
