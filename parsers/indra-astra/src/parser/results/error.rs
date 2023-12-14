use crate::parser::results::{
    end::End, error_builder::ErrorBuilder, parsed::Parsed, token::Token,
    token_builder::TokenBuilder,
};
use std::collections::{HashMap, HashSet};

use crate::parser::results::node::{Node, _EMPTY_KEYS, _EMPTY_TAGS};

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
        End::Fail({
            let mut err = ErrorBuilder::new("no_match_for_{}");
            err.set_text("No match found.").add_tag("none");
            err
        })
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
        let mut err = Error::New(&["unexpected_", key, "_in_{}"].concat());
        err.set_text(&format!("Unexpected: `{}`.", value))
            .add_tag("unexpected");
        End::Fail(err)
    }

    pub fn mismatch(key: &str, expected: &str, found: &str) -> End {
        let mut err = Error::New(&["unexpected_", key, "_in_{}"].concat());
        err.set_text(&format!(
            "Expected: `{}`, but found: `{}`.",
            expected, found
        ))
        .add_tag("missing");
        End::Fail(err)
    }

    pub fn missing(key: &str, expected: &str, found: &str) -> End {
        let mut err = Error::New(&["missing_expected_", key, "_in_{}"].concat());
        err.set_text(&format!(
            "Expected: `{}`, but found: `{}`.",
            expected, found
        ))
        .add_tag("missing");
        End::Fail(err)
    }

    pub fn in_child(parent: TokenBuilder, err: Option<Error>) -> End {
        let mut parent_err = ErrorBuilder {
            name: "incomplete_{}".to_string(),
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

        parent_err.add_tag("incomplete");
        parent_err.add_child(Parsed::Fail(err));

        return End::Fail(parent_err);
    }

    pub fn in_prop(parent: TokenBuilder, key: &str, err: Option<Error>) -> End {
        let mut parent_err = ErrorBuilder {
            name: "incomplete_{}".to_string(),
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

        parent_err.add_tag("incomplete");
        parent_err.set_prop(key, Parsed::Fail(err));

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
