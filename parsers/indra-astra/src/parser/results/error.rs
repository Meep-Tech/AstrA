use crate::{
    parser::results::{
        end::End, error_builder::ErrorBuilder, parsed::Parsed, token::Token,
        token_builder::TokenBuilder,
    },
    utils::{
        ansi::{Color, Styleable},
        sexp::SExpressable,
    },
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

    #[allow(non_snake_case)]
    pub fn Unexpected(key: &str, value: &str) -> End {
        let mut err = Error::New(&["unexpected_", key, "_in_{}"].concat());
        err.set_text(&format!("Unexpected: `{}`.", value))
            .add_tag("unexpected");
        End::Fail(err)
    }

    #[allow(non_snake_case)]
    pub fn Mismatch(key: &str, expected: &str, found: &str) -> End {
        let mut err = Error::New(&["unexpected_", key, "_in_{}"].concat());
        err.set_text(&format!(
            "Expected: `{}`, but found: `{}`.",
            expected, found
        ))
        .add_tag("missing");
        End::Fail(err)
    }

    #[allow(non_snake_case)]
    pub fn Missing(key: &str, expected: &str, found: &str) -> End {
        let mut err = Error::New(&["missing_expected_", key, "_in_{}"].concat());
        err.set_text(&format!(
            "Expected: `{}`, but found: `{}`.",
            expected, found
        ))
        .add_tag("missing");
        End::Fail(err)
    }

    #[allow(non_snake_case)]
    pub fn In_Child(parent: TokenBuilder, err: Option<Error>) -> End {
        let mut parent_err = ErrorBuilder {
            name: "incomplete_{}".to_string(),
            text: None,
            tags: parent.tags,
            start: parent.start,
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

    #[allow(non_snake_case)]
    pub fn In_Prop(parent: TokenBuilder, key: &str, err: Option<Error>) -> End {
        let mut parent_err = ErrorBuilder {
            name: "incomplete_{}".to_string(),
            text: None,
            tags: parent.tags,
            start: parent.start,
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

    pub fn to_builder(self) -> ErrorBuilder {
        return ErrorBuilder {
            name: self.name,
            tags: self.tags,
            text: self.text,
            start: Some(self.start),
            children: if !self.children.is_empty() {
                Some(self.children)
            } else {
                None
            },
            keys: self.keys,
        };
    }

    pub fn get_message(&self) -> String {
        let current = &self;
        let mut result = self.text.clone().unwrap_or("".to_string());

        if self.text.is_none() {
            let error_children = current
                .children
                .iter()
                .filter_map(|c| match &c {
                    Parsed::Fail(f) => match f {
                        Some(f) => Some(f),
                        None => None,
                    },
                    _ => None,
                })
                .collect::<Vec<&Error>>();

            if !error_children.is_empty() {
                result.push_str("[");

                result.push_str(
                    &error_children
                        .iter()
                        .map(|c| format!("\n\t - {}: {}", &c.name, c.get_message()))
                        .collect::<String>(),
                );

                result.push_str("\n]");
            }
        }

        return result;
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

impl SExpressable<Parsed> for Error {
    fn get_children(&self) -> Vec<&Parsed> {
        self.children()
    }
    fn get_keys(&self) -> &HashMap<String, usize> {
        self.keys()
    }
    fn get_name(&self) -> &str {
        self.name()
    }
    fn get_tags(&self) -> &HashSet<String> {
        self.tags()
    }
    fn name_color() -> Color {
        Color::BrightRed
    }
    fn node_to_sexp_str(node: &Parsed, depth: usize, colors: &Option<Color::Loop>) -> String {
        match node {
            Parsed::Pass(token) => token.to_sexp_str(depth, colors),
            Parsed::Fail(err) => match err {
                Some(err) => err.to_sexp_str(depth, colors),
                None => {
                    if colors.is_some() {
                        "<None>".color(Color::Magenta)
                    } else {
                        "<None>".to_string()
                    }
                }
            },
        }
    }
}

impl Span for Error {
    fn start(&self) -> usize {
        return self.start;
    }

    fn end(&self) -> usize {
        return self.end;
    }
}
