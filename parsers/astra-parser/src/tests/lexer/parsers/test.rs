use std::rc::Rc;

use crate::lexer::{
    parser::Parser,
    results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
};

pub trait Testable {
    fn tests() -> Vec<Test<Self>>
    where
        Self: 'static + Sized + Parser;

    fn run_tests() -> Vec<Outcome>
    where
        Self: 'static + Sized + Parser,
    {
        println!(
            "== Running Parser tests for: {}",
            Self::instance().get_name()
        );
        let mut results = Vec::new();
        for test in Self::tests() {
            println!("TEST: {}", test.name);
            let result = test.parser.parse(&test.input);
            let outcome: Outcome = _compare_results(result, &test.expected);

            match &outcome {
                Outcome::Pass => {
                    println!("PASS: {}", test.name);
                }
                Outcome::Fail(result) => {
                    println!(
                        "FAIL: {}. \n\t ?> Expected: {:?}, \n\t !> Actual: {:?}",
                        test.name, test.expected, result
                    );
                }
            }

            results.push(outcome);
            println!();
        }

        println!();
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
                        println!("{}", msg);
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
            Parsed::Error(ref err) => Comparison::NotEqual(format!(
                "Expected token: {}, Actual error: {}",
                expected.name, err.name
            )),
        },
        Parsed::Error(expected) => match &result {
            Parsed::Token(result) => Comparison::NotEqual(format!(
                "Expected error: {}, Actual token: {}",
                expected.name, &result.name
            )),
            Parsed::Error(err) => _compare_error_result(err, expected),
        },
    }
}

fn _compare_token_result(result: &Token, expected: &Token) -> Comparison {
    if result.name == expected.name {
        return Comparison::NotEqual(format!(
            "Expected token name: {}, Actual token name: {}",
            expected.name, result.name
        ));
    }

    if let Some(tags) = &expected.tags {
        if tags.contains(_TEST_IS_FROM_PARSER_TAG) {
            return Comparison::AreEqual;
        }
    }

    if result.start != expected.start {
        return Comparison::NotEqual(format!(
            "Expected start: {}, Actual start: {}",
            expected.start, result.start
        ));
    }

    if result.end != expected.end {
        return Comparison::NotEqual(format!(
            "Expected end: {}, Actual end: {}",
            expected.end, result.end
        ));
    }

    if expected.children.len() > 0 {
        if expected.children.len() != result.children.len() {
            return Comparison::NotEqual(format!(
                "Expected child count: {}, Actual child count: {}",
                expected.children.len(),
                result.children.len()
            ));
        }

        for (i, expected_child) in expected.children.iter().enumerate() {
            let result_child = &result.children[i];
            match _compare_token_result(result_child, expected_child) {
                Comparison::AreEqual => {}
                Comparison::NotEqual(msg) => {
                    return Comparison::NotEqual(format!(
                        "Expected child: {}, Actual child: {}",
                        msg, result_child.name
                    ));
                }
            }
        }
    } else if result.children.len() > 0 {
        return Comparison::NotEqual(format!(
            "Expected child count: {}, Actual child count: {}",
            expected.children.len(),
            result.children.len()
        ));
    }

    if let Some(expected_props) = &expected.props {
        if let Some(result_props) = &result.props {
            if expected_props.len() != result_props.len() {
                return Comparison::NotEqual(format!(
                    "Expected prop count: {}, Actual prop count: {}",
                    expected_props.len(),
                    result_props.len()
                ));
            }

            for (key, expected_prop) in expected_props.iter() {
                let result_prop = result_props.get(key);
                let expected_prop_name = &expected_prop.name;

                match result_prop {
                    None => {
                        return Comparison::NotEqual(format!(
                            "Expected prop: {} is missing",
                            expected_prop_name
                        ));
                    }
                    Some(existing_result) => {
                        match _compare_token_result(&existing_result, expected_prop) {
                            Comparison::AreEqual => {}
                            Comparison::NotEqual(msg) => {
                                return Comparison::NotEqual(format!(
                                    "Expected prop: {}, Actual prop: {}",
                                    msg, existing_result.name
                                ));
                            }
                        }
                    }
                }
            }
        } else {
            return Comparison::NotEqual(format!(
                "Expected prop count: {}, Actual prop count: {}",
                expected_props.len(),
                0
            ));
        }
    } else if result.props.is_some() {
        return Comparison::NotEqual(format!(
            "Expected prop count: {}, Actual prop count: {}",
            0,
            result.props.as_ref().unwrap().len()
        ));
    }

    return Comparison::AreEqual;
}

fn _compare_error_result(result: &Error, expected: &Error) -> Comparison {
    if result.name != expected.name {
        return Comparison::NotEqual(format!(
            "Expected error name: {}, Actual error name: {}",
            expected.name, result.name
        ));
    }

    if result.start != expected.start {
        return Comparison::NotEqual(format!(
            "Expected start: {}, Actual start: {}",
            expected.start, result.start
        ));
    }

    if result.end != expected.end {
        return Comparison::NotEqual(format!(
            "Expected end: {}, Actual end: {}",
            expected.end, result.end
        ));
    }

    if expected.children.len() > 0 {
        if expected.children.len() != result.children.len() {
            return Comparison::NotEqual(format!(
                "Expected child count: {}, Actual child count: {}",
                expected.children.len(),
                result.children.len()
            ));
        }

        for (i, expected_child) in expected.children.iter().enumerate() {
            let result_child = &result.children[i];
            let result_child_name = match &result_child {
                Parsed::Token(token) => token.name.clone(),
                Parsed::Error(err) => err.name.clone(),
            };
            let expected_child_name = match &expected_child {
                Parsed::Token(token) => token.name.clone(),
                Parsed::Error(err) => err.name.clone(),
            };

            match _compare_token_or_error(&result_child, expected_child) {
                Comparison::AreEqual => {}
                Comparison::NotEqual(msg) => {
                    return Comparison::NotEqual(format!(
                        "Expected child: {}, Actual child: {}. \n\t {}",
                        expected_child_name, result_child_name, msg
                    ));
                }
            }
        }
    } else if result.children.len() > 0 {
        return Comparison::NotEqual(format!(
            "Expected child count: {}, Actual child count: {}",
            expected.children.len(),
            result.children.len()
        ));
    }

    if let Some(expected_props) = &expected.props {
        if let Some(result_props) = &result.props {
            if expected_props.len() != result_props.len() {
                return Comparison::NotEqual(format!(
                    "Expected prop count: {}, Actual prop count: {}",
                    expected_props.len(),
                    result_props.len()
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
                            "Expected prop: {} is missing",
                            expected_prop_name
                        ));
                    }
                    Some(existing_result) => match &expected_prop {
                        Parsed::Token(token) => match existing_result {
                            Parsed::Token(existing_token) => {
                                match _compare_token_result(&existing_token, token) {
                                    Comparison::AreEqual => {}
                                    Comparison::NotEqual(msg) => {
                                        return Comparison::NotEqual(format!(
                                            "Expected prop: {}, Actual prop: {}",
                                            msg, existing_token.name
                                        ));
                                    }
                                }
                            }
                            Parsed::Error(err) => {
                                return Comparison::NotEqual(format!(
                                    "Expected prop: {}, Actual prop: {}",
                                    expected_prop_name, err.name
                                ));
                            }
                        },
                        Parsed::Error(err) => match existing_result {
                            Parsed::Token(token) => {
                                return Comparison::NotEqual(format!(
                                    "Expected prop: {}, Actual prop: {}",
                                    expected_prop_name, token.name
                                ));
                            }
                            Parsed::Error(existing_err) => {
                                match _compare_error_result(&existing_err, err) {
                                    Comparison::AreEqual => {}
                                    Comparison::NotEqual(msg) => {
                                        return Comparison::NotEqual(format!(
                                            "Expected prop: {}, Actual prop: {}",
                                            msg, existing_err.name
                                        ));
                                    }
                                }
                            }
                        },
                    },
                }
            }
        } else {
            return Comparison::NotEqual(format!(
                "Expected prop count: {}, Actual prop count: {}",
                expected_props.len(),
                0
            ));
        }
    } else if result.props.is_some() {
        return Comparison::NotEqual(format!(
            "Expected prop count: {}, Actual prop count: {}",
            0,
            result.props.as_ref().unwrap().len()
        ));
    }

    return Comparison::AreEqual;
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
    TParser: Parser,
{
    pub parser: Rc<TParser>,
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
