use std::fmt::Display;

use super::{term, token, Token};

#[derive(Debug, Clone)]
pub enum Type {
    Term(term::Type),
    Token(token::Type),
}

#[derive(Debug, Clone)]
pub struct Error {
    etype: String,
    ttype: Type,
    index: usize,
    data: Vec<Vec<String>>,
}

impl<'e> Error {
    pub type Type = Type;

    pub const INVALID_KEY: &'static str = "invalid_syntax";
    pub const UNEXPECTED_KEY: &'static str = "unexpected_syntax";
    pub const IN_CHILD_KEY: &'static str = "in_child";
    pub const IN_PROP_KEY: &'static str = "in_prop";

    #[allow(non_snake_case)]
    pub(in super::super) fn Unexpected(
        ttype: &Type,
        index: usize,
        found: impl Display,
        expected: &[&str],
    ) -> Error {
        Error {
            etype: Error::UNEXPECTED_KEY.to_string(),
            index,
            ttype: ttype.clone(),
            data: vec![
                expected.iter().map(|e| e.to_string()).collect(),
                vec![found.to_string()],
            ],
        }
    }

    #[allow(non_snake_case)]
    pub(in super::super) fn Invalid(
        ttype: &Type,
        index: usize,
        found: impl Display,
        reason: impl Display,
    ) -> Error {
        Error {
            etype: Error::INVALID_KEY.to_string(),
            index,
            ttype: ttype.clone(),
            data: vec![vec![reason.to_string()], vec![found.to_string()]],
        }
    }

    pub fn key(&self) -> &str {
        &self.etype
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn message(&self) -> String {
        let mut message = String::from(format!(
            "Error in {} @ {}: *{}*; ",
            format!("{:?}", self.ttype),
            self.index,
            self.etype,
        ));

        match self.etype.as_str() {
            Error::INVALID_KEY => {
                message.push_str(&self.data[0][0]);
            }
            Error::UNEXPECTED_KEY => {
                message.push_str(&format!(
                    "Found {}, Expected: {}",
                    self.data[1][0],
                    self.data[0].join(", "),
                ));
            }
            Error::INVALID_KEY => {
                if self.data.len() > 1 {
                    message.push_str(&format!(
                        "Invalid Syntax: {}. {}",
                        self.data[1][0], self.data[0][0],
                    ));
                } else {
                    message.push_str(&format!("Invalid Syntax. {}", self.data[0][0]));
                }
            }
            _ => panic!("unhandled error type"),
        }

        message
    }
}
