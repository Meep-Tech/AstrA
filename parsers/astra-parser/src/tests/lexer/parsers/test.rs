use std::{collections::HashMap, rc::Rc};

use crate::{
    lexer::{
        parser::Parser,
        results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
    },
    utils::log::{self, Color, Styleable},
};

pub enum Outcome {
    Pass(Parsed, Parsed),
    Fail(Parsed, Parsed, String),
}

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
    pub expected: Parsed,
    pub enabled: bool,
}

impl<TParser> Test<TParser>
where
    TParser: Parser + 'static,
{
    pub fn new(name: &str, input: &str, expected: Parsed) -> Self {
        Self {
            parser: TParser::Instance(),
            name: name.to_string(),
            input: input.to_string(),
            expected,
            enabled: true,
        }
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}

pub trait Testable {
    fn tests() -> Vec<Test<Self>>
    where
        Self: 'static + Sized + Parser;

    fn run_tests() -> HashMap<String, Outcome>
    where
        Self: 'static + Sized + Parser,
    {
        log::add_color("FAIL", Color::BrightRed);
        log::add_color("PASS", Color::BrightGreen);
        log::add_color("TOKEN", Color::BrightBlue);

        log::ln!();
        log::push_key(&"TEST".color(Color::Yellow));
        log::push_unique_key("INIT");
        let key = Self::Instance().get_name();
        log::pop_unique_key("INIT");
        log::push_key(key);

        log::push_unique_key("INIT");
        let tests = Self::tests();
        log::pop_unique_key("INIT");

        log::ln!();
        log::info!(&[":START"], "Running tests");
        log::push_key_div("-", Color::Yellow);

        let mut results: HashMap<String, Outcome> = HashMap::new();
        for test in tests {
            if !test.enabled {
                continue;
            }

            log::push_key(&test.name.color(Color::BrightYellow));
            log::push_key_div("-", Color::Yellow);
            log::plain!(
                &[":START"],
                &format!(
                    "Running test on input: {}",
                    format!(
                        "\n\t┏{}\n\t┖",
                        format!("\n{}", test.input).replace("\n", "\n\t┣ ")
                    )
                ),
            );

            let result = test.parser.parse(&test.input);
            let outcome: Outcome = _verify_outcome(result, test.expected);

            match &outcome {
                Outcome::Pass(_, _) => {
                    log::info!(&[":END", "PASS"], &format!("Test passed"));
                }
                Outcome::Fail(expected, result, reason) => {
                    _log_failure(&test.name, expected, result, reason);
                }
            }

            log::pop_key();
            log::pop_key();
            log::ln!();

            results.insert(test.name, outcome);
        }

        log::pop_key();
        log::pop_key();
        log::info!(&[":END"], "Finished running tests.");
        log::ln!();

        _log_report(&results);

        log::ln!();
        results
    }
}

fn _log_report(results: &HashMap<String, Outcome>) {
    log::push_key(&"RESULTS".color(Color::Magenta));

    // log the percentage of tests that passed
    let mut passes = 0;
    for (_, outcome) in results.iter() {
        match outcome {
            Outcome::Pass(_, _) => {
                passes += 1;
            }
            Outcome::Fail(_, _, _) => {}
        }
    }

    let percentage = (passes as f32 / results.len() as f32) * 100.0;
    log::info!(
        &[":ALL", &"REPORT".color(Color::Blue)],
        &format!("{}% of tests passed", percentage)
    );

    log::push_key_div("-", Color::Magenta);

    let mut failures: Vec<(&str, &Outcome)> = Vec::new();
    for (name, outcome) in results.iter() {
        match outcome {
            Outcome::Pass(_, _) => {
                log::info!(&[&name.color(Color::Yellow), "PASS"], &format!("{}", name));
            }
            Outcome::Fail(_, _, _) => {
                log::error!(&[&name.color(Color::Yellow), "FAIL"], &format!("{}", name));
                failures.push((name, outcome));
            }
        }
    }

    log::pop_key();

    if failures.len() == 0 {
        log::info!(&[":ALL", "PASS"], "All tests passed");
    } else if failures.len() < results.len() {
        log::error!(&[":SOME", "FAIL"], "Some tests failed");
        _log_failures(failures);
    } else {
        log::error!(&[":ALL", "FAIL"], "All tests failed");
        _log_failures(failures);
    }

    log::pop_key();
    log::pop_key();
}

fn _log_failures(failures: Vec<(&str, &Outcome)>) {
    log::ln!();
    log::push_key(&"FAILURES".color(Color::BrightRed));
    log::push_key_div("-", Color::BrightRed);

    for (name, outcome) in failures {
        match outcome {
            Outcome::Pass(_, _) => {}
            Outcome::Fail(expected, result, reason) => {
                _log_failure(name, expected, result, reason);
            }
        }
    }

    log::pop_key();
    log::pop_key();
}

fn _log_failure(test_name: &str, expected: &Parsed, result: &Parsed, reason: &String) {
    log::error!(
        &[&test_name.color(Color::Yellow)],
        &format!(
            "{}: \n\t\t {} {}: \n\t\t{}, {}: \n\t\t{}",
            "\n\t:: Reason".color(Color::BrightYellow),
            reason.color(Color::Yellow),
            log::color(Color::BrightGreen, "\n\t?> Expected"),
            format!("{:#?}", expected).color(Color::Green).indent(2),
            log::color(Color::BrightRed, "\n\t!> Actual"),
            format!("{:#?}", result).color(Color::Red).indent(2)
        ),
    );
}

#[allow(non_snake_case)]
pub fn IsFrom<T>() -> Token
where
    T: Parser + 'static,
{
    Token::new()
        .name(T::Instance().get_name())
        .tag(_TEST_IS_FROM_PARSER_TAG)
        .build(0, 0)
}

const _TEST_IS_FROM_PARSER_TAG: &str = "__test__is_from_parser__";

fn _verify_outcome(result: Parsed, expected: Parsed) -> Outcome {
    match _compare_token_or_error(&result, &expected) {
        Comparison::AreEqual => Outcome::Pass(result, expected),
        Comparison::NotEqual(msg) => {
            log::warning!(&["!", "COMPARE", "FAIL"], &msg);
            Outcome::Fail(expected, result, msg)
        }
    }
}

fn _compare_token_or_error(result: &Parsed, expected: &Parsed) -> Comparison {
    match expected {
        Parsed::Token(expected) => match result {
            Parsed::Token(token) => _compare_token_result(&token, &expected),
            Parsed::Error(err) => Comparison::NotEqual(_mismatch(
                "error (Expected a token)",
                &format!("{:#?}", expected),
                &format!("{:#?}", err),
            )),
        },
        Parsed::Error(expected) => match result {
            Parsed::Token(result) => Comparison::NotEqual(_mismatch(
                "token (Expected an error)",
                &format!("{:#?}", expected),
                &format!("{:#?}", result),
            )),
            Parsed::Error(err) => _compare_error_result(&err, &expected),
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

        for i in 0..expected.children.len() {
            let expected_child = &expected.children[i];
            let result_child = &result.children[i];
            match _compare_token_result(result_child, expected_child) {
                Comparison::AreEqual => {}
                Comparison::NotEqual(msg) => {
                    return Comparison::NotEqual(_mismatch(
                        &format!("child at index: {}. {}", i, msg),
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

    if let Some(expected_props) = &expected.keys {
        if let Some(result_props) = &result.keys {
            if expected_props.len() != result_props.len() {
                return Comparison::NotEqual(_mismatch(
                    "prop count",
                    &expected_props.len().to_string(),
                    &result_props.len().to_string(),
                ));
            }

            for (key, index) in expected_props {
                if index >= &result.children.len() {
                    return Comparison::NotEqual(format!(
                        "Expected prop: {}, with type {} is missing.",
                        key, expected.children[*index].name,
                    ));
                }

                let expected_prop = &expected.children[*index];
                let result_prop = &result.children[*index];
                let expected_prop_name = &expected_prop.name;

                match _compare_token_result(result_prop, expected_prop) {
                    Comparison::AreEqual => {}
                    Comparison::NotEqual(msg) => {
                        return Comparison::NotEqual(_mismatch(
                            &format!("prop: {}. {}", expected_prop_name, msg),
                            &format!("{:#?}", expected_prop),
                            &format!("{:#?}", result_prop),
                        ));
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
    } else if result.keys.is_some() {
        return Comparison::NotEqual(_mismatch(
            "prop count",
            &0.to_string(),
            &result.keys.as_ref().unwrap().len().to_string(),
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

        for i in 0..expected.children.len() {
            let expected_child = &expected.children[i];
            let result_child = &result.children[i];
            match _compare_token_or_error(result_child, expected_child) {
                Comparison::AreEqual => {}
                Comparison::NotEqual(msg) => {
                    return Comparison::NotEqual(_mismatch(
                        &format!("child at index: {}. {}", i, msg),
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

    if let Some(expected_props) = &expected.keys {
        if let Some(result_props) = &result.keys {
            if expected_props.len() != result_props.len() {
                return Comparison::NotEqual(_mismatch(
                    "prop count",
                    &expected_props.len().to_string(),
                    &result_props.len().to_string(),
                ));
            }

            for (key, index) in expected_props {
                if index >= &result.children.len() {
                    return Comparison::NotEqual(format!(
                        "Expected prop: {}, with type {} is missing.",
                        key,
                        match &expected.children[*index] {
                            Parsed::Token(token) => token.name.clone(),
                            Parsed::Error(err) => err.name.clone(),
                        },
                    ));
                }

                let expected_prop = &expected.children[*index];
                let result_prop = &result.children[*index];

                let expected_prop_name = match expected_prop {
                    Parsed::Token(token) => token.name.clone(),
                    Parsed::Error(err) => err.name.clone(),
                };

                match result_prop {
                    Parsed::Token(result) => match expected_prop {
                        Parsed::Token(expected) => {
                            match _compare_token_result(&result, &expected) {
                                Comparison::AreEqual => {}
                                Comparison::NotEqual(msg) => {
                                    return Comparison::NotEqual(_mismatch(
                                        &format!("prop: {}. {}", expected_prop_name, msg),
                                        &format!("{:#?}", result),
                                        &format!("{:#?}", expected),
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
                                &format!("{:#?}", result),
                                &format!("{:#?}", err),
                            ));
                        }
                    },
                    Parsed::Error(result) => match expected_prop {
                        Parsed::Token(expected) => {
                            return Comparison::NotEqual(_mismatch(
                                &format!(
                                    "type. Expected error: {}, found token: {}.",
                                    expected_prop_name, expected.name
                                ),
                                &format!("{:#?}", result),
                                &format!("{:#?}", expected),
                            ));
                        }
                        Parsed::Error(existing_err) => {
                            match _compare_error_result(&existing_err, &result) {
                                Comparison::AreEqual => {}
                                Comparison::NotEqual(msg) => {
                                    return Comparison::NotEqual(_mismatch(
                                        &format!("prop: {}. {}", expected_prop_name, msg),
                                        &format!("{:#?}", result),
                                        &format!("{:#?}", existing_err),
                                    ));
                                }
                            }
                        }
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
    } else if result.keys.is_some() {
        return Comparison::NotEqual(_mismatch(
            "prop count",
            &0.to_string(),
            &result.keys.as_ref().unwrap().len().to_string(),
        ));
    }

    return Comparison::AreEqual;
}

fn _mismatch(prop: &str, expected: &str, actual: &str) -> String {
    return format!(
        "Mismatch in {}. {}: \n\t\t{}, {}: \n\t\t{}",
        prop,
        log::color(Color::BrightGreen, "\n\t?> Expected"),
        format!("{}", expected).color(Color::Green).indent(2),
        log::color(Color::BrightRed, "\n\t!> Actual"),
        format!("{}", actual).color(Color::Red).indent(2)
    );
}
