use crate::parser::results::span::Span;

use super::ansi::{Color, ColorLoop, Styleable};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SFormat<'s> {
    pub colors: Option<ColorLoop>,
    pub include_token_length: bool,
    pub text_source: Option<&'s str>,
    pub current_depth: usize,
}

pub trait SExpressable<TNode>: Span {
    fn get_name(&self) -> String;
    fn get_keys(&self) -> &HashMap<String, usize>;
    fn get_children(&self) -> Vec<&TNode>;
    fn get_tags(&self) -> &HashSet<String>;
    fn name_color() -> Color;
    fn node_to_sexp_str(node: &TNode, config: &mut SFormat) -> String;
    fn extra_subs(&self, _config: &mut SFormat) -> Vec<String> {
        vec![]
    }

    fn to_sexp_str(&self, config: Option<SFormat>) -> String {
        let mut result = String::new();
        let mut config = match config {
            Some(config) => config,
            None => SFormat {
                colors: None,
                include_token_length: false,
                text_source: None,
                current_depth: 0,
            },
        };
        config.current_depth += 1;

        macro_rules! nl {
            () => {
                result.push_str(" \n");
                for _ in 0..config.current_depth {
                    result.push_str("\t");
                }
            };
        }

        match config.colors {
            Some(ref mut colors) => {
                colors.next();
                result.push_str("(".color(colors.curr()).as_str());
            }
            None => {
                result.push_str("(");
            }
        }
        if config.colors.is_some() {
            result.push_str(self.get_name().color(Self::name_color()).as_str());
        } else {
            result.push_str(self.get_name().as_str());
        }

        let mut span_text = format!(" [{}..{}]", self.start(), self.end());
        if config.colors.is_some() {
            span_text = span_text.color(Color::BrightBlack);
        }
        result.push_str(span_text.as_str());

        if config.include_token_length {
            let length = self.end() - self.start() + 1;
            let length_text = format!(" <{}>", length);
            if config.colors.is_some() {
                result.push_str(length_text.color(Color::BrightBlack).as_str());
            } else {
                result.push_str(length_text.as_str());
            }
        }

        for tag in self.get_tags().iter() {
            if config.colors.is_some() {
                result.push_str(" #".color(Color::Yellow).as_str());
                result.push_str(tag.color(Color::Yellow).as_str());
            } else {
                result.push_str(" #");
                result.push_str(tag);
            }
        }

        if let Some(src) = config.text_source {
            nl!();
            let text = format!("`{}`", src[self.start()..=self.end()].to_string());
            if config.colors.is_some() {
                result.push_str(text.color(Color::BrightWhite).as_str());
            } else {
                result.push_str(text.as_str());
            }
        }

        let extra_subs: Vec<String> = self.extra_subs(&mut config);
        if self.get_children().is_empty() && self.get_keys().is_empty() && extra_subs.is_empty() {
            match config.colors {
                Some(mut colors) => {
                    result.push_str(")".color(colors.curr()).as_str());
                    colors.prev();
                }
                None => {
                    result.push_str(")");
                }
            }

            return result;
        } else {
            nl!();
        }

        let keys = &self.get_keys();
        for (index, child) in self.get_children().iter().enumerate() {
            let prop_key = match keys.iter().find(|(_, i)| **i == index) {
                Some((key, _)) => Some(key),
                None => None,
            };
            if prop_key.is_some() {
                if config.colors.is_some() {
                    result.push_str(prop_key.unwrap().color(Color::Cyan).as_str());
                    result.push_str(": ".color(Color::BrightCyan).as_str());
                } else {
                    result.push_str(prop_key.unwrap().as_str());
                    result.push_str(": ");
                }
            }

            result.push_str(Self::node_to_sexp_str(child, &mut config).as_str());

            let is_last_child = index == self.get_children().len() - 1;
            if !is_last_child || !extra_subs.is_empty() {
                nl!();
            }
        }

        for (index, extra_sub) in extra_subs.iter().enumerate() {
            result.push_str(extra_sub.as_str());
            if index != extra_subs.len() - 1 {
                nl!();
            }
        }

        match config.colors {
            Some(mut colors) => {
                result.push_str(")".color(colors.curr()).as_str());
                colors.prev();
            }
            None => {
                result.push_str(")");
            }
        }

        return result;
    }
}