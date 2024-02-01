use std::{collections::HashSet, fmt::Display};

use super::{
    error::Error,
    node::{Node, _EMPTY_TAGS},
    span::Span,
    token::Token,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Parsed {
    Pass(Token),
    Fail(Option<Error>),
}

impl Parsed {
    pub fn get_name(&self) -> &str {
        match self {
            Parsed::Pass(node) => node.name(),
            Parsed::Fail(error) => match error {
                Some(error) => error.name(),
                None => "none",
            },
        }
    }

    pub fn get_tags(&self) -> &HashSet<String> {
        match self {
            Parsed::Pass(node) => node.tags(),
            Parsed::Fail(error) => match error {
                Some(error) => error.tags(),
                None => &_EMPTY_TAGS,
            },
        }
    }

    pub fn get_start(&self) -> usize {
        match self {
            Parsed::Pass(node) => node.start(),
            Parsed::Fail(error) => match error {
                Some(error) => error.start(),
                None => 0,
            },
        }
    }

    pub fn get_end(&self) -> usize {
        match self {
            Parsed::Pass(node) => node.end(),
            Parsed::Fail(error) => match error {
                Some(error) => error.end(),
                None => 0,
            },
        }
    }
}

pub trait Formatable: Display {
    fn format_as_html(&self, parsed: Parsed) -> String {
        self.format_part_as_html(0, parsed)
    }

    fn format_part_as_html(&self, offset: usize, parsed: Parsed) -> String {
        let mut result = String::new();
        let text = self.to_string();
        let start: usize;
        let end: usize;

        match &parsed {
            Parsed::Pass(span) => {
                start = span.start() - offset;
                end = span.end() - offset;
            }
            Parsed::Fail(span) => match span {
                Some(span) => {
                    start = span.start() - offset;
                    end = span.end() - offset;
                }
                None => {
                    start = 0;
                    end = text.len();
                }
            },
        }

        result.push_str(&text[..start]);
        if let Parsed::Pass(token) = &parsed
            && token.children().len() != 0
        {
            let mut last_end = start;
            let mut child_text = String::new();
            for child in token.children() {
                child_text.push_str(&text[last_end..child.start()]);
                let child_span = (&text[child.start()..child.end()])
                    .format_part_as_html(child.start(), Parsed::Pass(child.clone()));
                child_text.push_str(&child_span);
                last_end = child.end();
            }
            child_text.push_str(&text[last_end..end]);

            result.push_str(&child_text);
        } else {
            result.push_str(&text);
        }
        result.push_str(&text[end..]);

        wrap_in_html_span(
            text,
            match &parsed {
                Parsed::Pass(node) => node.name(),
                Parsed::Fail(error) => match error {
                    Some(error) => error.name(),
                    None => "--none",
                },
            },
            match &parsed {
                Parsed::Pass(node) => node.tags(),
                Parsed::Fail(error) => match error {
                    Some(error) => error.tags(),
                    None => &_EMPTY_TAGS,
                },
            },
        )
    }
}

impl Formatable for String {}
impl Formatable for &str {}

fn wrap_in_html_span(text: String, name: &str, tags: &HashSet<String>) -> String {
    "<span class=\"".to_owned()
        + name
        + " "
        + &tags.iter().fold(String::new(), |acc, tag| acc + tag + " ")
        + "\">"
        + &text
        + "</span>"
}
