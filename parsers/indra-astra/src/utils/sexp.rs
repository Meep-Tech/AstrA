use super::ansi::{Color, ColorLoop, Styleable};
use std::collections::{HashMap, HashSet};

pub trait SExpressable<TNode> {
    fn get_name(&self) -> &str;
    fn get_keys(&self) -> &HashMap<String, usize>;
    fn get_children(&self) -> Vec<&TNode>;
    fn get_tags(&self) -> &HashSet<String>;
    fn name_color() -> Color;
    fn node_to_sexp_str(node: &TNode, depth: usize, colors: &mut Option<Color::Loop>) -> String;

    fn to_sexp_str(&self, depth: usize, colors: &mut Option<ColorLoop>) -> String {
        let depth = depth + 1;
        let mut result = String::new();

        macro_rules! nl {
            () => {
                result.push_str(" \n");
                for _ in 0..depth {
                    result.push_str("\t");
                }
            };
        }

        match colors {
            Some(colors) => {
                colors.next();
                result.push_str("(".color(colors.curr()).as_str());
            }
            None => {
                result.push_str("(");
            }
        }
        if colors.is_some() {
            result.push_str(self.get_name().color(Self::name_color()).as_str());
        } else {
            result.push_str(self.get_name());
        }

        for tag in self.get_tags().iter() {
            if colors.is_some() {
                result.push_str("#".color(Color::Yellow).as_str());
                result.push_str(tag.color(Color::Yellow).as_str());
            } else {
                result.push_str("#");
                result.push_str(tag);
            }
        }

        if self.get_children().is_empty() && self.get_keys().is_empty() {
            match colors {
                Some(colors) => {
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
                if colors.is_some() {
                    result.push_str(prop_key.unwrap().color(Color::Cyan).as_str());
                    result.push_str(": ".color(Color::BrightCyan).as_str());
                } else {
                    result.push_str(prop_key.unwrap().as_str());
                    result.push_str(": ");
                }
            }

            result.push_str(Self::node_to_sexp_str(child, depth, colors).as_str());
            nl!();
        }

        match colors {
            Some(colors) => {
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
