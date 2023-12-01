use std::{collections::HashMap, rc::Rc};

use crate::{
    lexer::{
        parser::Parser,
        results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
    },
    utils::log::{self, Color, Colorable, Indentable},
};

pub trait Testable {
    fn tests() -> Vec<Test<Self>>
    where
        Self: 'static + Sized + Parser;

    fn run_tests() -> HashMap<String, Outcome>
    where
        Self: 'static + Sized + Parser,
    {
        log::set_color("FAIL", Color::BrightRed);
        log::set_color("PASS", Color::BrightGreen);
        log::set_color("TOKEN", Color::BrightBlue);

        log::ln();
        log::push_key(&"TEST".color(Color::Yellow));
        log::push_unique_key("INIT");
        let key = Self::instance().get_name();
        log::pop_unique_key("INIT");
        log::push_key(key);

        log::push_unique_key("INIT");
        let tests = Self::tests();
        log::pop_unique_key("INIT");

        log::ln();
        log::info(&[":START"], "Running tests");
        log::push_key_div("-", Color::Yellow);

        let mut results: HashMap<String, Outcome> = HashMap::new();
        for test in tests {
            log::push_key(&test.name.color(Color::BrightYellow));
            log::push_key_div("-", Color::Yellow);
            log::info(
                &[":START"],
                &format!(
                    "Running test on input: {:}",
                    format!(
                        "\n\t┏{:}\n\t┖",
                        format!("\n{:}", test.input).replace("\n", "\n\t┣ ")
                    )
                ),
            );

            let result = test.parser.parse(&test.input);
            let outcome: Outcome = _compare_results(result, &test.expected);

            match &outcome {
                Outcome::Pass => {
                    log::info(&[":END", "PASS"], &format!("Test passed"));
                }
                Outcome::Fail(result) => {
                    log::warn(
                        &[":END", "FAIL"],
                        &format!(
                            "Test failed. \n\t ?> {:}: \n\t\t{:}, \n\t !> {:}: \n\t\t{:}",
                            log::color(Color::BrightGreen, "Expected"),
                            format!("{:#?}", test.expected)
                                .color(Color::Green)
                                .indent(2),
                            log::color(Color::BrightRed, "Actual"),
                            format!("{:#?}", result).color(Color::Red).indent(2),
                        ),
                    );
                }
            }

            log::pop_key();
            log::pop_key();

            results.insert(test.name, outcome);
        }

        log::pop_key();
        log::pop_key();
        log::info(&[":END"], "Finished running tests.");

        log::ln();
        log::push_key(&"RESULTS".color(Color::Magenta));
        log::push_key_div("-", Color::Magenta);
        let mut all_passed = true;
        for (name, outcome) in results.iter() {
            match outcome {
                Outcome::Pass => {
                    log::info(&[&name.color(Color::Yellow), "PASS"], &format!("{:}", name));
                }
                Outcome::Fail(_) => {
                    log::warn(&[&name.color(Color::Yellow), "FAIL"], &format!("{:}", name));
                    all_passed = false;
                }
            }
        }

        log::pop_key();

        if all_passed {
            log::info(&[":ALL", "PASS"], "All tests passed");
        } else {
            log::error(&[":SOME", "FAIL"], "Some tests failed");
        }

        log::pop_key();
        log::pop_key();

        log::ln();
        results
    }
}

const _TEST_IS_FROM_PARSER_TAG: &str = "__test__is_from_parser__";

fn _compare_results(result: Option<Parsed>, expected: &Option<Parsed>) -> Outcome {
    match &expected {
        Some(expected_some) => match result {
            Some(resulting_some) => {
                match _compare_token_or_error(&resulting_some, &expected_some) {
                    Comparison::AreEqual => Outcome::Pass,
                    Comparison::NotEqual(msg) => {
                        log::warn(&["!", "COMPARE", "FAIL"], &msg);
                        Outcome::Fail(Some(resulting_some))
                    }
                }
            }
            None => Outcome::Fail(None),
        },
        None => match result {
            Some(resulting_some) => Outcome::Fail(Some(resulting_some)),
            None => Outcome::Pass,
        },
    }
}

fn _compare_token_or_error(result: &Parsed, expected: &Parsed) -> Comparison {
    match &expected {
        Parsed::Token(ref expected) => match &result {
            Parsed::Token(ref token) => _compare_token_result(token, expected),
            Parsed::Error(ref err) => Comparison::NotEqual(_mismatch(
                "token",
                &format!("{:#?}", expected),
                &format!("{:#?}", err),
            )),
        },
        Parsed::Error(expected) => match &result {
            Parsed::Token(result) => Comparison::NotEqual(_mismatch(
                "error",
                &format!("{:#?}", expected),
                &format!("{:#?}", result),
            )),
            Parsed::Error(err) => _compare_error_result(err, expected),
        },
    }
}

fn _compare_token_result(result: &Token, expected: &Token) -> Comparison {
    if result.name != expected.name {
        return Comparison::NotEqual(_mismatch("name", &expected.name, &result.name));
    }

    if let Some(tags) = &expected.tags {
        if tags.contains(_TEST_IS_FROM_PARSER_TAG) {
            return Comparison::AreEqual;
        }
    }

    if result.start != expected.start {
        return Comparison::NotEqual(_mismatch(
            "start",
            &expected.start.to_string(),
            &result.start.to_string(),
        ));
    }

    if result.end != expected.end {
        return Comparison::NotEqual(_mismatch(
            "end",
            &expected.end.to_string(),
            &result.end.to_string(),
        ));
    }

    if expected.children.len() > 0 {
        if expected.children.len() != result.children.len() {
            return Comparison::NotEqual(_mismatch(
                "child count",
                &expected.children.len().to_string(),
                &result.children.len().to_string(),
            ));
        }

        for (i, expected_child) in expected.children.iter().enumerate() {
            let result_child = &result.children[i];
            match _compare_token_result(result_child, expected_child) {
                Comparison::AreEqual => {}
                Comparison::NotEqual(msg) => {
                    return Comparison::NotEqual(_mismatch(
                        &format!("child at index: {:}. {:}", i, msg.indent(1)),
                        &format!("{:#?}", expected_child),
                        &format!("{:#?}", result_child),
                    ));
                }
            }
        }
    } else if result.children.len() > 0 {
        return Comparison::NotEqual(_mismatch(
            "child count",
            &expected.children.len().to_string(),
            &result.children.len().to_string(),
        ));
    }

    if let Some(expected_props) = &expected.props {
        if let Some(result_props) = &result.props {
            if expected_props.len() != result_props.len() {
                return Comparison::NotEqual(_mismatch(
                    "prop count",
                    &expected_props.len().to_string(),
                    &result_props.len().to_string(),
                ));
            }

            for (key, expected_prop) in expected_props.iter() {
                let result_prop = result_props.get(key);
                let expected_prop_name = &expected_prop.name;

                match result_prop {
                    None => {
                        return Comparison::NotEqual(format!(
                            "Expected prop: {}, with type {} is missing.",
                            key, expected_prop_name,
                        ));
                    }
                    Some(existing_result) => {
                        match _compare_token_result(&existing_result, expected_prop) {
                            Comparison::AreEqual => {}
                            Comparison::NotEqual(msg) => {
                                return Comparison::NotEqual(_mismatch(
                                    &format!("prop: {:}. {:}", expected_prop_name, msg.indent(1)),
                                    &format!("{:#?}", expected_prop),
                                    &format!("{:#?}", existing_result),
                                ));
                            }
                        }
                    }
                }
            }
        } else {
            return Comparison::NotEqual(_mismatch(
                "prop count",
                &expected_props.len().to_string(),
                &0.to_string(),
            ));
        }
    } else if result.props.is_some() {
        return Comparison::NotEqual(_mismatch(
            "prop count",
            &0.to_string(),
            &result.props.as_ref().unwrap().len().to_string(),
        ));
    }

    return Comparison::AreEqual;
}

fn _compare_error_result(result: &Error, expected: &Error) -> Comparison {
    if result.name != expected.name {
        return Comparison::NotEqual(_mismatch("name", &expected.name, &result.name));
    }

    if result.start != expected.start {
        return Comparison::NotEqual(_mismatch(
            "start",
            &expected.start.to_string(),
            &result.start.to_string(),
        ));
    }

    if result.end != expected.end {
        return Comparison::NotEqual(_mismatch(
            "end",
            &expected.end.to_string(),
            &result.end.to_string(),
        ));
    }

    if expected.children.len() > 0 {
        if expected.children.len() != result.children.len() {
            return Comparison::NotEqual(_mismatch(
                "child count",
                &expected.children.len().to_string(),
                &result.children.len().to_string(),
            ));
        }

        for (i, expected_child) in expected.children.iter().enumerate() {
            let result_child = &result.children[i];
            match _compare_token_or_error(&result_child, expected_child) {
                Comparison::NotEqual(msg) => {
                    return Comparison::NotEqual(_mismatch(
                        &format!("child at index: {:}. {:}", i, msg.indent(1)),
                        &format!("{:#?}", expected_child),
                        &format!("{:#?}", result_child),
                    ));
                }
                Comparison::AreEqual => {}
            }
        }
    } else if result.children.len() > 0 {
        return Comparison::NotEqual(_mismatch(
            "child count",
            &expected.children.len().to_string(),
            &result.children.len().to_string(),
        ));
    }

    if let Some(expected_props) = &expected.props {
        if let Some(result_props) = &result.props {
            if expected_props.len() != result_props.len() {
                return Comparison::NotEqual(_mismatch(
                    "prop count",
                    &expected_props.len().to_string(),
                    &result_props.len().to_string(),
                ));
            }

            for (key, expected_prop) in expected_props.iter() {
                let result_prop = result_props.get(key);
                let expected_prop_name = match &expected_prop {
                    Parsed::Token(token) => token.name.clone(),
                    Parsed::Error(err) => err.name.clone(),
                };

                match result_prop {
                    None => {
                        return Comparison::NotEqual(format!(
                            "Expected prop: {}, with type {} is missing",
                            key, expected_prop_name
                        ));
                    }
                    Some(existing_result) => match &expected_prop {
                        Parsed::Token(token) => match existing_result {
                            Parsed::Token(existing_token) => {
                                match _compare_token_result(&existing_token, token) {
                                    Comparison::AreEqual => {}
                                    Comparison::NotEqual(msg) => {
                                        return Comparison::NotEqual(_mismatch(
                                            &format!(
                                                "Expected prop: {}, Actual prop: {}",
                                                expected_prop_name, msg
                                            ),
                                            &format!("{:#?}", token),
                                            &format!("{:#?}", existing_token),
                                        ));
                                    }
                                }
                            }
                            Parsed::Error(err) => {
                                return Comparison::NotEqual(_mismatch(
                                    &format!(
                                        "Expected prop: {}, Actual prop: {}",
                                        expected_prop_name, err.name
                                    ),
                                    &format!("{:#?}", token),
                                    &format!("{:#?}", err),
                                ));
                            }
                        },
                        Parsed::Error(err) => match existing_result {
                            Parsed::Token(token) => {
                                return Comparison::NotEqual(_mismatch(
                                    &format!(
                                        "Expected prop: {}, Actual prop: {}",
                                        expected_prop_name, token.name
                                    ),
                                    &format!("{:#?}", err),
                                    &format!("{:#?}", token),
                                ));
                            }
                            Parsed::Error(existing_err) => {
                                match _compare_error_result(&existing_err, err) {
                                    Comparison::AreEqual => {}
                                    Comparison::NotEqual(msg) => {
                                        return Comparison::NotEqual(_mismatch(
                                            &format!(
                                                "Expected prop: {}, Actual prop: {}",
                                                expected_prop_name, msg
                                            ),
                                            &format!("{:#?}", err),
                                            &format!("{:#?}", existing_err),
                                        ));
                                    }
                                }
                            }
                        },
                    },
                }
            }
        } else {
            return Comparison::NotEqual(_mismatch(
                "prop count",
                &expected_props.len().to_string(),
                &0.to_string(),
            ));
        }
    } else if result.props.is_some() {
        return Comparison::NotEqual(_mismatch(
            "prop count",
            &0.to_string(),
            &result.props.as_ref().unwrap().len().to_string(),
        ));
    }

    return Comparison::AreEqual;
}

fn _mismatch(prop: &str, expected: &str, actual: &str) -> String {
    return format!(
        "Mismatch in {:}. \n\t ?> {:}: \n\t\t{:}, \n\t !> {:}: \n\t\t{:}",
        prop,
        log::color(Color::BrightGreen, "Expected"),
        expected.color(Color::Green).indent(2),
        log::color(Color::BrightRed, "Actual"),
        actual.color(Color::Red).indent(2),
    );
}

#[allow(non_snake_case)]
pub fn IsFrom<T>() -> Token
where
    T: Parser + 'static,
{
    Token::new()
        .name(T::instance().get_name())
        .tag(_TEST_IS_FROM_PARSER_TAG)
        .build(0, 0)
}

#[allow(dead_code)]
pub enum Outcome {
    Pass,
    Fail(Option<Parsed>),
}

#[allow(dead_code)]
enum Comparison {
    AreEqual,
    NotEqual(String),
}

pub struct Test<TParser>
where
    TParser: Parser + 'static,
{
    pub parser: &'static Rc<TParser>,
    pub name: String,
    pub input: String,
    pub expected: Option<Parsed>,
}

impl<TParser> Test<TParser>
where
    TParser: Parser + 'static,
{
    pub fn new(name: &str, input: &str, expected: Option<Parsed>) -> Self {
        Self {
            parser: TParser::instance(),
            name: name.to_string(),
            input: input.to_string(),
            expected,
        }
    }
}
