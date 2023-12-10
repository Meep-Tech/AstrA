use std::{collections::HashSet, rc::Rc};

use crate::{
    lexer::{
        parser::Parser,
        parsers,
        results::{
            builder::Builder, error::Error, error_builder::ErrorBuilder, node::Node,
            parsed::Parsed, token::Token, token_builder::TokenBuilder,
        },
    },
    utils::log::{self, Color, Styleable},
};

pub enum Outcome {
    Pass {
        test: Test,
        result: Parsed,
    },
    Fail {
        test: Test,
        result: Parsed,
        reason: String,
    },
}

enum Comparison {
    AreEqual,
    NotEqual(String),
}

pub struct Test {
    pub parser: &'static Rc<dyn Parser>,
    pub name: String,
    pub tags: Vec<String>,
    pub input: String,
    pub expected: Parsed,
    pub enabled: bool,
    pub(crate) subs: Option<Vec<String>>,
}

impl Test {
    pub fn new<TParser>(name: &str, input: &str, expected: Parsed) -> Self
    where
        TParser: Parser + 'static,
    {
        let mut test = Self {
            parser: TParser::Get(),
            name: name.to_string(),
            input: input.to_string(),
            expected,
            tags: vec![],
            enabled: true,
            subs: None,
        };

        _add_default_tags::<TParser>(&mut test);
        test.tags.push(name.to_string().to_lowercase());

        test
    }

    pub fn tags<TParser>(tags: &[&str], input: &str, expected: Parsed) -> Self
    where
        TParser: Parser + 'static,
    {
        let mut test = Self {
            parser: TParser::Get(),
            name: tags.join(" & "),
            input: input.to_string(),
            expected,
            tags: vec![],
            enabled: true,
            subs: None,
        };

        _add_default_tags::<TParser>(&mut test);
        let mut tags = tags.to_vec();
        tags.sort();
        test.tags.extend(
            tags.iter()
                .map(|tag| tag.to_string().to_lowercase())
                .into_iter(),
        );

        test
    }

    pub fn pattern<TParser>(name: &str, input: &str, subs: &[&str], expected: Parsed) -> Self
    where
        TParser: Parser + 'static,
    {
        let mut test = Self {
            parser: TParser::Get(),
            name: name.to_string(),
            input: input.to_string(),
            expected,
            enabled: true,
            tags: vec![],
            subs: Some(subs.iter().map(|part| part.to_string()).collect()),
        };

        _add_default_tags::<TParser>(&mut test);
        test.tags.push(name.to_string().to_lowercase());

        test
    }

    pub fn pattern_with_tags<TParser>(
        tags: &[&str],
        input: &str,
        subs: &[&str],
        expected: Parsed,
    ) -> Self
    where
        TParser: Parser + 'static,
    {
        let mut test = Self {
            parser: TParser::Get(),
            name: tags.join(" & "),
            input: input.to_string(),
            expected,
            enabled: true,
            tags: vec![],
            subs: Some(subs.iter().map(|part| part.to_string()).collect()),
        };

        _add_default_tags::<TParser>(&mut test);
        let mut tags = tags.to_vec();
        tags.sort();
        test.tags.extend(
            tags.iter()
                .map(|tag| tag.to_string().to_lowercase())
                .into_iter(),
        );

        test
    }

    pub fn partial(mut self) -> Self {
        self.tags.push("partial".to_string());
        self.name = format!("{} & Partial", self.name);
        self
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn expects_fail(&self) -> bool {
        match self.expected {
            Parsed::Pass(_) => false,
            Parsed::Fail(_) => true,
        }
    }

    pub fn expects_pass(&self) -> bool {
        match self.expected {
            Parsed::Pass(_) => true,
            Parsed::Fail(_) => false,
        }
    }

    pub fn expects_none(&self) -> bool {
        match &self.expected {
            Parsed::Pass(_) => false,
            Parsed::Fail(err) => match err {
                Some(_) => false,
                None => true,
            },
        }
    }

    pub fn expects_error(&self) -> bool {
        match &self.expected {
            Parsed::Pass(_) => false,
            Parsed::Fail(err) => match err {
                Some(_) => true,
                None => false,
            },
        }
    }
}

pub fn test_parsers(parsers: &[&'static dyn Testable]) -> Vec<Outcome> {
    log::push_unique!(&"TEST".color(Color::Yellow));
    let mut results: Vec<Outcome> = Vec::new();

    for parser in parsers {
        results.extend(parser.run_tests());
    }

    log::push_unique!(&"TEST".color(Color::Yellow));
    log::info!(
        &[&"RESULTS".color(Color::Magenta)],
        &"FINAL TEST REPORT ===============================".color(Color::BrightMagenta)
    );
    #[cfg(feature = "log")]
    log_results(&results);
    log::pop_unique!(&"TEST".color(Color::Yellow));

    return results;
}

pub trait Mockable<T> {
    fn mock(self) -> T;
    fn partial(self, start: Option<usize>, end: Option<usize>) -> T;
}

pub trait Mocked {
    fn is_mock(&self) -> bool;
}

impl Mockable<Token> for TokenBuilder {
    fn mock(self) -> Token {
        self.partial(None, None)
    }

    fn partial(self, start: Option<usize>, end: Option<usize>) -> Token {
        let start_val: usize;
        let end_val: usize;

        let mut builder = self.tag(_TEST_MOCK_TAG);
        builder.add_tag(_TEST_PARTIAL_CHILDREN_TAG);
        builder.add_tag(_TEST_PARTIAL_PROPS_TAG);
        builder.add_tag(_TEST_PARTIAL_TAGS_TAG);
        match start {
            Some(val) => {
                start_val = val;
            }
            None => {
                start_val = 0;
                builder.add_tag(_TEST_IGNORE_START_TAG);
            }
        }

        match end {
            Some(val) => {
                end_val = val;
            }
            None => {
                end_val = start_val + 1;
                builder.add_tag(_TEST_IGNORE_END_TAG);
            }
        }

        builder.build(start_val, end_val)
    }
}

impl Mocked for Token {
    fn is_mock(&self) -> bool {
        self.tags
            .as_ref()
            .unwrap()
            .contains(&_TEST_MOCK_TAG.to_string())
    }
}

impl Mockable<Error> for ErrorBuilder {
    fn mock(self) -> Error {
        self.partial(None, None)
    }

    fn partial(self, start: Option<usize>, end: Option<usize>) -> Error {
        let start_val: usize;
        let end_val: usize;

        let mut builder = self.tag(_TEST_MOCK_TAG);
        match start {
            Some(val) => {
                start_val = val;
            }
            None => {
                start_val = 0;
                builder.add_tag(_TEST_IGNORE_START_TAG);
            }
        }

        match end {
            Some(val) => {
                end_val = val;
            }
            None => {
                end_val = start_val + 1;
                builder.add_tag(_TEST_IGNORE_END_TAG);
            }
        }

        builder.build(start_val, end_val).unwrap()
    }
}

impl Mocked for Error {
    fn is_mock(&self) -> bool {
        self.tags
            .as_ref()
            .unwrap()
            .contains(&_TEST_MOCK_TAG.to_string())
    }
}

pub trait Testable: Parser {
    #[allow(non_snake_case)]
    fn Tests() -> &'static dyn Testable
    where
        Self: 'static + Sized + Parser + Testable,
    {
        Self::Instance().as_tests().unwrap()
    }

    #[allow(non_snake_case)]
    fn Get_Tests() -> Vec<Test>
    where
        Self: 'static + Sized + Parser,
    {
        Self::Instance().get_tests()
    }

    #[allow(non_snake_case)]
    fn Run_Tests() -> Vec<Outcome>
    where
        Self: 'static + Sized + Parser,
    {
        Self::Instance().run_tests()
    }

    // Tags that should always be on this token, given it's produced by other tag types usually.
    // These tags are added to the result before it's compared to the expected and colorized,
    // TODO: check for when these tags for this type when it's used in a patterns to ensure consistency.
    fn assure_tags(&self) -> Option<HashSet<String>> {
        None
    }

    fn get_tests(&self) -> Vec<Test>;

    fn run_tests(&self) -> Vec<Outcome> {
        log::color!("FAIL", Color::BrightRed);
        log::color!("PASS", Color::BrightGreen);
        log::color!("TOKEN", Color::BrightBlue);

        log::ln!();
        log::push_unique!(&"TEST".color(Color::Yellow));

        log::push_unique!("INIT");
        #[cfg(feature = "log")]
        let name = self.name();
        log::pop_unique!("INIT");
        log::push!(name);
        log::push_unique!("INIT");
        let tests = self.get_tests();
        log::pop_unique!("INIT");

        log::ln!();
        log::info!(&[":START"], "Running tests");
        log::push_div!("-", Color::Yellow);

        let mut results: Vec<Outcome> = Vec::new();
        for test in tests {
            if !test.enabled {
                continue;
            }

            log::push!(&test.name.color(Color::BrightYellow));
            log::push_div!("-", Color::Yellow);
            #[cfg(feature = "verbose")]
            log::plain!(
                &[":START"],
                &format!(
                    "Running test on input: {}",
                    _format_input(&test.input, "\t")
                ),
            );

            if let Some(subs) = &test.subs {
                results.extend(_run_test_via_pattern(&test, subs));
            } else {
                let result = test.parser.parse(&test.input);
                let outcome: Outcome = _verify_outcome(test, result);

                #[cfg(feature = "log")]
                match &outcome {
                    Outcome::Pass { test: _, result: _ } => {
                        log::info!(&[":END", "PASS"], &format!("Test passed"));
                    }
                    Outcome::Fail {
                        test,
                        result,
                        reason,
                    } => {
                        _log_failure(
                            &test.name,
                            &test.parser.name(),
                            &test.expected,
                            result,
                            reason,
                            &test.input,
                        );
                    }
                }

                results.push(outcome);
            }

            log::pop!();
            log::pop!();
            log::ln!();
        }

        log::pop!();
        log::pop!();
        log::info!(&[":END"], "Finished running tests.");
        #[cfg(feature = "vv")]
        log::ln!();

        #[cfg(feature = "vv")]
        log_results(&results);

        log::pop_unique!(&"TEST".color(Color::Yellow));
        log::ln!();
        results
    }
}

#[cfg(feature = "log")]
pub fn log_results(results: &Vec<Outcome>) {
    log::push!(&"RESULTS".color(Color::Magenta));

    // log the percentage of tests that passed
    let mut passes = 0;
    let mut failures: Vec<&Outcome> = Vec::new();
    for outcome in results {
        match outcome {
            Outcome::Pass { test: _, result: _ } => {
                passes += 1;
            }
            Outcome::Fail {
                test: _,
                result: _,
                reason: _,
            } => {
                failures.push(outcome);
            }
        }
    }

    log::log!(
        &[":ALL", &"REPORT".color(Color::Blue)],
        &format!(
            "{}% of tests passed",
            (passes as f32 / results.len() as f32) * 100.0
        )
    );
    log::ln!();

    log::push_div!("-", Color::Magenta);

    #[cfg(feature = "log")]
    let mut failures: Vec<&Outcome> = Vec::new();
    #[cfg(feature = "log")]
    for outcome in results {
        match outcome {
            Outcome::Pass { test, result } => {
                log::plain!(
                    &[
                        test.parser.name(),
                        "-",
                        &test.name.color(Color::Yellow),
                        "PASS"
                    ],
                    &format!(
                        "{}\n\t => {}",
                        &_format_input(&test.input, &"✔\t".color(Color::BrightGreen)),
                        match result {
                            Parsed::Pass(token) => format!(
                                "{} ({}, {})",
                                token.name.color(Color::Green),
                                token.start,
                                token.end
                            ),
                            Parsed::Fail(err) => match err {
                                Some(err) => format!(
                                    "{} ({}, {})",
                                    err.name.color(Color::Red),
                                    err.start,
                                    err.end
                                ),
                                None => "-none-".to_string(),
                            },
                        }
                    )
                );
            }
            Outcome::Fail {
                test,
                result,
                reason: _,
            } => {
                log::plain!(
                    &[
                        test.parser.name(),
                        "-",
                        &test.name.color(Color::Yellow),
                        &"FAIL".color(Color::BrightRed)
                    ],
                    &format!(
                        "{}\n\t => {}",
                        &_format_input(&test.input, &"✘\t".color(Color::BrightRed)),
                        match result {
                            Parsed::Pass(token) => format!(
                                "{} ({}, {})",
                                token.name.color(Color::Green),
                                token.start,
                                token.end
                            ),
                            Parsed::Fail(err) => match err {
                                Some(err) => format!(
                                    "{} ({}, {})",
                                    err.name.color(Color::Red),
                                    err.start,
                                    err.end
                                ),
                                None => "-none-".to_string(),
                            },
                        }
                    )
                );
                failures.push(outcome);
            }
        }
    }

    log::pop!();
    log::ln!();

    if failures.len() == 0 {
        log::log!(&[":ALL", "PASS"], "All tests passed");
        log::ln!();
    } else if failures.len() < results.len() {
        log::log!(&[":SOME", "FAIL"], "Some tests failed");
        log::ln!();
        _log_failures(failures);
    } else {
        log::log!(&[":ALL", "FAIL"], "All tests failed");
        log::ln!();
        _log_failures(failures);
    }

    log::pop!();
    log::pop!();
}

#[cfg(feature = "log")]
fn _log_failures(failures: Vec<&Outcome>) {
    log::ln!();
    log::push!(&"FAILURES".color(Color::BrightRed));
    log::push_div!("-", Color::BrightRed);

    for outcome in failures {
        match outcome {
            Outcome::Fail {
                test,
                result,
                reason,
            } => {
                _log_failure(
                    &test.name,
                    &test.parser.name(),
                    &test.expected,
                    result,
                    reason,
                    &test.input,
                );
            }
            _ => {}
        }
    }

    log::pop!();
    log::pop!();
}

#[cfg(feature = "log")]
fn _log_failure(
    test_name: &str,
    parser_name: &str,
    expected: &Parsed,
    result: &Parsed,
    reason: &String,
    input: &str,
) {
    log::error!(
        &[
            parser_name,
            &"-".color(Color::BrightRed),
            &test_name.color(Color::Yellow)
        ],
        &format!(
            "{}: {} {}: \n\t\t {} {}: \n\t\t{}, {}: \n\t\t{}",
            "\n\t:: Input".color(Color::White),
            _format_input(input, "\t").indent(1),
            "\n\t:: Reason".color(Color::BrightYellow),
            reason.color(Color::Yellow),
            "\n\t?> Expected".color(Color::BrightGreen),
            format!("{:#?}", expected).color(Color::Green).indent(2),
            "\n\t!> Actual".color(Color::BrightRed),
            format!("{:#?}", result).color(Color::Red).indent(2)
        ),
    );
}

pub trait TokenMocks {
    #[allow(non_snake_case)]
    fn Mock<T>() -> Token
    where
        T: Parser + 'static,
    {
        Token::New()
            .name(T::Instance().name())
            .tag(_TEST_MOCK_TAG)
            .tag(_TEST_NAME_ONLY)
            .mock()
    }

    #[allow(non_snake_case)]
    fn With_Tag<T>(tag: &str) -> Token
    where
        T: Parser + 'static,
    {
        Token::New()
            .name(T::Instance().name())
            .tag(_TEST_MOCK_TAG)
            .tag(_TEST_PARTIAL_TAGS_TAG)
            .tag(tag)
            .mock()
    }
}

impl TokenMocks for Token {}

const _TEST_MOCK_TAG: &str = "__test__mock__";
const _TEST_NAME_ONLY: &str = "__test__name_only__";
const _TEST_IGNORE_TAGS_TAG: &str = "__test__ignore_tags__";
const _TEST_IGNORE_START_TAG: &str = "__test__ignore_start__";
const _TEST_IGNORE_END_TAG: &str = "__test__ignore_end__";
const _TEST_PARTIAL_TAGS_TAG: &str = "__test__partial_tags__";
const _TEST_PARTIAL_CHILDREN_TAG: &str = "__test__partial_children__";
const _TEST_PARTIAL_PROPS_TAG: &str = "__test__partial_props__";

fn _verify_outcome(test: Test, mut result: Parsed) -> Outcome {
    match _compare_token_or_error(&mut result, &test.expected, &test) {
        Comparison::AreEqual => Outcome::Pass { test, result },
        Comparison::NotEqual(reason) => {
            log::warning!(&["!", "COMPARE", "FAIL"], &reason);
            Outcome::Fail {
                test,
                result,
                reason,
            }
        }
    }
}

fn _compare_token_or_error(result: &mut Parsed, expected: &Parsed, test: &Test) -> Comparison {
    match expected {
        Parsed::Pass(expected) => match result {
            Parsed::Pass(result) => _compare_token_result(result, &expected, test),
            Parsed::Fail(result) => Comparison::NotEqual(_mismatch(
                "error (Expected a token)",
                &format!("{:#?}", expected),
                &format!("{:#?}", result),
            )),
        },
        Parsed::Fail(expected) => match result {
            Parsed::Pass(result) => Comparison::NotEqual(_mismatch(
                "token (Expected an error)",
                &format!("{:#?}", expected),
                &format!("{:#?}", result),
            )),
            Parsed::Fail(result) => _compare_error_result(result, &expected, test),
        },
    }
}

fn _compare_token_result(result: &mut Token, expected: &Token, test: &Test) -> Comparison {
    if result.name != expected.name {
        return Comparison::NotEqual(_mismatch("name", &expected.name, &result.name));
    }

    let default_tags = test.parser.as_tests().unwrap().assure_tags();
    match default_tags {
        Some(assured_tags) => match &mut result.tags {
            Some(ref mut result_tags) => {
                result_tags.extend(assured_tags.clone());
            }
            None => {
                result.tags = Some(assured_tags.clone());
            }
        },
        None => {}
    }

    if let Some(value) = _compare_tags(&expected.tags, &result.tags) {
        return value;
    }

    if result.start != expected.start {
        if !expected.tag(_TEST_IGNORE_START_TAG) {
            return Comparison::NotEqual(_mismatch(
                "start",
                &expected.start.to_string(),
                &result.start.to_string(),
            ));
        }
    }

    if result.end != expected.end {
        if !expected.tag(_TEST_IGNORE_END_TAG) {
            return Comparison::NotEqual(_mismatch(
                "end",
                &expected.end.to_string(),
                &result.end.to_string(),
            ));
        }
    }

    if expected.children.len() > 0 {
        if !expected.tag(_TEST_PARTIAL_CHILDREN_TAG)
            && expected.children.len() != result.children.len()
        {
            return Comparison::NotEqual(_mismatch(
                "child count",
                &expected.children.len().to_string(),
                &result.children.len().to_string(),
            ));
        }

        for i in 0..expected.children.len() {
            if result.children.len() <= i {
                break;
            }

            let expected_child = &expected.children[i];
            let result_child = &mut result.children[i];

            match _compare_token_result(result_child, expected_child, test) {
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
    } else if !expected.tag(_TEST_PARTIAL_CHILDREN_TAG) && result.children.len() > 0 {
        return Comparison::NotEqual(_mismatch(
            "child count",
            &expected.children.len().to_string(),
            &result.children.len().to_string(),
        ));
    }

    if let Some(expected_props) = &expected.keys {
        if let Some(result_props) = &mut result.keys {
            if !expected.tag(_TEST_PARTIAL_PROPS_TAG) && expected_props.len() != result_props.len()
            {
                return Comparison::NotEqual(_mismatch(
                    "prop count",
                    &expected_props.len().to_string(),
                    &result_props.len().to_string(),
                ));
            }

            for (key, index) in expected_props {
                let expected_prop: &Token;
                let mut result_prop: Token;

                if !expected.tag(_TEST_PARTIAL_PROPS_TAG) {
                    if index >= &mut result.children.len() {
                        if !expected.tag(_TEST_PARTIAL_PROPS_TAG) {
                            return Comparison::NotEqual(format!(
                                "Expected prop: {}, with type {} is missing.",
                                key, expected.children[*index].name,
                            ));
                        } else {
                            break;
                        }
                    }

                    expected_prop = &expected.children[*index];
                    result_prop = result.children[*index].clone();
                } else {
                    expected_prop = &expected.children[0];
                    result_prop = result.prop(key).unwrap().clone();
                }

                let expected_prop_name = &expected_prop.name;
                match _compare_token_result(&mut result_prop, expected_prop, test) {
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
    } else if !expected.tag(_TEST_PARTIAL_PROPS_TAG) && result.keys.is_some() {
        return Comparison::NotEqual(_mismatch(
            "prop count",
            &0.to_string(),
            &result.keys.as_ref().unwrap().len().to_string(),
        ));
    }

    return Comparison::AreEqual;
}

fn _compare_error_result(
    result: &mut Option<Error>,
    expected: &Option<Error>,
    test: &Test,
) -> Comparison {
    match result {
        Some(result) => match expected {
            Some(expected) => {
                if result.name != expected.name {
                    return Comparison::NotEqual(_mismatch("name", &expected.name, &result.name));
                }

                if let Some(value) = _compare_tags(&expected.tags, &result.tags) {
                    return value;
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
                    if !expected.tag(_TEST_PARTIAL_CHILDREN_TAG)
                        && expected.children.len() != result.children.len()
                    {
                        return Comparison::NotEqual(_mismatch(
                            "child count",
                            &expected.children.len().to_string(),
                            &result.children.len().to_string(),
                        ));
                    }

                    for i in 0..expected.children.len() {
                        let expected_child = &expected.children[i];
                        let result_child = &mut result.children[i];
                        match _compare_token_or_error(result_child, expected_child, test) {
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
                    if let Some(result_props) = &mut result.keys {
                        if !expected.tag(_TEST_PARTIAL_PROPS_TAG)
                            && expected_props.len() != result_props.len()
                        {
                            return Comparison::NotEqual(_mismatch(
                                "prop count",
                                &expected_props.len().to_string(),
                                &result_props.len().to_string(),
                            ));
                        }

                        for (key, index) in expected_props {
                            let expected_prop: &Parsed;
                            let mut result_prop: Parsed;
                            if !expected.tag(_TEST_PARTIAL_PROPS_TAG) {
                                if index >= &mut result.children.len() {
                                    return Comparison::NotEqual(format!(
                                        "Expected prop: {}, with type {} is missing.",
                                        key,
                                        match &expected.children[*index] {
                                            Parsed::Pass(token) => token.name.clone(),
                                            Parsed::Fail(err) => match err {
                                                Some(err) => err.name.clone(),
                                                None => "-none-".to_string(),
                                            },
                                        },
                                    ));
                                }

                                expected_prop = &expected.children[*index];
                                result_prop = result.children[*index].clone();
                            } else {
                                expected_prop = &expected.children[0];
                                result_prop = result.prop(key).unwrap().clone();
                            }

                            let expected_prop_name = match expected_prop {
                                Parsed::Pass(token) => token.name.clone(),
                                Parsed::Fail(err) => match err {
                                    Some(err) => err.name.clone(),
                                    None => "-none-".to_string(),
                                },
                            };

                            match result_prop {
                                Parsed::Pass(ref mut result) => match expected_prop {
                                    Parsed::Pass(expected) => {
                                        match _compare_token_result(result, &expected, test) {
                                            Comparison::AreEqual => {}
                                            Comparison::NotEqual(msg) => {
                                                return Comparison::NotEqual(_mismatch(
                                                    &format!(
                                                        "prop: {} = {}. {}",
                                                        key, expected_prop_name, msg
                                                    ),
                                                    &format!("{:#?}", expected),
                                                    &format!("{:#?}", result),
                                                ));
                                            }
                                        }
                                    }
                                    Parsed::Fail(expected) => {
                                        return Comparison::NotEqual(_mismatch(
                                            &format!(
                                                "Expected prop: {}, Actual prop: {}",
                                                expected_prop_name,
                                                match expected {
                                                    Some(err) => err.name.clone(),
                                                    None => "-none-".to_string(),
                                                }
                                            ),
                                            &format!("{:#?}", expected),
                                            &format!("{:#?}", result),
                                        ));
                                    }
                                },
                                Parsed::Fail(ref mut result) => match expected_prop {
                                    Parsed::Pass(expected) => {
                                        return Comparison::NotEqual(_mismatch(
                                            &format!(
                                                "type. Expected error: {}, found token: {}.",
                                                expected_prop_name, expected.name
                                            ),
                                            &format!("{:#?}", expected),
                                            &format!("{:#?}", result),
                                        ));
                                    }
                                    Parsed::Fail(expected) => {
                                        match _compare_error_result(result, &expected, test) {
                                            Comparison::AreEqual => {}
                                            Comparison::NotEqual(msg) => {
                                                return Comparison::NotEqual(_mismatch(
                                                    &format!(
                                                        "prop: {} = {}. {}",
                                                        key, expected_prop_name, msg
                                                    ),
                                                    &format!("{:#?}", expected),
                                                    &format!("{:#?}", result),
                                                ));
                                            }
                                        }
                                    }
                                },
                            }

                            // add the potentially modified result prop back to the result
                            result.children[*index] = result_prop;
                        }
                    } else {
                        return Comparison::NotEqual(_mismatch(
                            "prop count",
                            &expected_props.len().to_string(),
                            &0.to_string(),
                        ));
                    }
                } else if !expected.tag(_TEST_PARTIAL_PROPS_TAG) && result.keys.is_some() {
                    return Comparison::NotEqual(_mismatch(
                        "prop count",
                        &0.to_string(),
                        &result.keys.as_ref().unwrap().len().to_string(),
                    ));
                }
            }
            None => {
                return Comparison::NotEqual(_mismatch(
                    "error (Expected a token)",
                    &format!("{:#?}", expected),
                    &format!("{:#?}", result),
                ));
            }
        },
        None => match expected {
            Some(expected) => {
                return Comparison::NotEqual(_mismatch(
                    "token (Expected an error)",
                    &format!("{:#?}", expected),
                    &format!("{:#?}", result),
                ));
            }
            None => {
                return Comparison::AreEqual;
            }
        },
    }

    return Comparison::AreEqual;
}

fn _compare_tags(
    expected_tags: &Option<HashSet<String>>,
    result_tags: &Option<HashSet<String>>,
) -> Option<Comparison> {
    if let Some(expected_tags) = expected_tags {
        if expected_tags.contains(_TEST_NAME_ONLY) {
            return Some(Comparison::AreEqual);
        } else if expected_tags.contains(_TEST_IGNORE_TAGS_TAG) {
            return None;
        } else {
            let countable_tags = expected_tags
                .iter()
                .filter(|t| !t.starts_with("__test__"))
                .count();

            if countable_tags > 0 {
                if let Some(result_tags) = result_tags {
                    if !expected_tags.contains(_TEST_PARTIAL_TAGS_TAG)
                        && (countable_tags != result_tags.len())
                    {
                        return Some(Comparison::NotEqual(_mismatch(
                            "tag count",
                            &countable_tags.to_string(),
                            &result_tags.len().to_string(),
                        )));
                    }

                    for tag in expected_tags {
                        if !tag.starts_with("__test__") && !result_tags.contains(tag) {
                            return Some(Comparison::NotEqual(format!(
                                "Missing expected tag: {}",
                                tag
                            )));
                        }
                    }

                    if expected_tags.contains(_TEST_PARTIAL_TAGS_TAG) {
                        return Some(Comparison::AreEqual);
                    }
                } else {
                    return Some(Comparison::NotEqual(_mismatch(
                        "tags",
                        &countable_tags.to_string(),
                        "None",
                    )));
                }
            }
        }
    }
    None
}

fn _mismatch(prop: &str, expected: &str, result: &str) -> String {
    return format!(
        "Mismatch in {}.{}: \n\t\t{}, {}: \n\t\t{}",
        prop,
        "\n\tExpected".color(Color::BrightGreen),
        format!("{}", expected).color(Color::Green).indent(2),
        "\n\tActual".color(Color::BrightRed),
        format!("{}", result).color(Color::Red).indent(2)
    );
}

fn _add_default_tags<TParser>(test: &mut Test)
where
    TParser: Parser + 'static,
{
    test.tags.push(TParser::Instance().name().to_string());
    if let Parsed::Pass(_) = &test.expected {
        test.tags.push("Pass".to_string());
    } else if let Parsed::Fail(err) = &test.expected {
        test.tags.push("Fail".to_string());
        if let Some(_) = err {
            test.tags.push("Error".to_string());
        } else {
            test.tags.push("None".to_string());
        }
    }
}

fn _format_input(input: &str, prefix: &str) -> String {
    format!(
        "\n{}┏{}\n\t┖",
        prefix,
        format!("\n{}", input).replace("\n", "\n\t┣ ")
    )
}

fn _run_test_via_pattern(test: &Test, subs: &Vec<String>) -> Vec<Outcome> {
    let patterns = _build_tests_from_pattern(test, subs);
    return _run_pattern_tests(test, patterns);
}

fn _run_pattern_tests(test: &Test, patterns: Vec<(String, Vec<usize>)>) -> Vec<Outcome> {
    let mut results: Vec<Outcome> = Vec::new();
    for (pattern, combo) in patterns {
        let merged_combo = combo
            .iter()
            .map(|i| i.to_string())
            .intersperse(", ".to_string())
            .collect::<String>();
        log::push_div!("-", Color::Yellow);
        #[cfg(feature = "verbose")]
        log::plain!(
            &["PATTERN", &"COMBO".color(Color::BrightYellow)],
            &format!("Running test on input: {}", _format_input(&pattern, "\t")),
        );

        let result = test.parser.parse(&pattern);
        let pattern_key = &format!("Pattern ({})", merged_combo);
        let outcome: Outcome = _verify_outcome(
            Test {
                input: pattern,
                parser: test.parser,
                name: test.name.clone() + &" & " + &pattern_key,
                expected: test.expected.clone(),
                enabled: test.enabled,
                subs: None,
                tags: test
                    .tags
                    .iter()
                    .chain(vec![&"pattern".to_string()])
                    .cloned()
                    .collect(),
            },
            result,
        );

        #[cfg(feature = "log")]
        match &outcome {
            Outcome::Pass { test: _, result: _ } => {
                log::info!(&[":END", "PASS"], &format!("Test passed"));
            }
            Outcome::Fail {
                test,
                result,
                reason,
            } => {
                _log_failure(
                    &test.name,
                    &test.parser.name(),
                    &test.expected,
                    result,
                    reason,
                    &test.input,
                );
            }
        }

        log::pop!();
        log::ln!();

        results.push(outcome);
    }

    results
}

fn _build_tests_from_pattern(test: &Test, subs: &Vec<String>) -> Vec<(String, Vec<usize>)> {
    log::push!("PATTERN");
    let sub_pattern_count = &test.input.matches("{}").count();

    if subs.len() != *sub_pattern_count {
        log::error!(
            &["COUNT-MISMATCH"],
            &format!(
                "Expected {} sub-patterns, found {}",
                sub_pattern_count,
                subs.len()
            ),
        );
        panic!("Sub-pattern count mismatch in Test")
    } else {
        log::info!(
            &["INIT"],
            &format!("Found {} sub-patterns for test", sub_pattern_count),
        );
    }

    let mut substitutions: Vec<(String, Vec<String>)> = Vec::new();
    let mut total_combinations = 1;

    for sub_type_key in subs {
        let sub_type_testable = parsers::get_tests_for(&sub_type_key);

        log::info!(
            &["INIT"],
            &format!(
                "Found {} tests for sub-type: {}",
                sub_type_testable.get_tests().len(),
                sub_type_key
            ),
        );

        let sub_type_input_options: Vec<String> = sub_type_testable
            .get_tests()
            .iter()
            .filter(|test| test.expects_pass() && !test.tags.contains(&"partial".to_string()))
            .map(|test| test.input.to_string())
            .collect();

        if sub_type_input_options.len() == 0 {
            log::error!(
                &["INIT"],
                &format!(
                    "Found {} valid inputs for sub-type: {}",
                    sub_type_input_options.len(),
                    sub_type_key
                ),
            );
            panic!(
                "No valid test inputs found for parser type: {}",
                sub_type_key
            );
        } else {
            log::info!(
                &["INIT"],
                &format!(
                    "Found {} valid inputs for sub-type: {}",
                    sub_type_input_options.len(),
                    sub_type_key
                ),
            );
        }

        total_combinations *= sub_type_input_options.len();
        substitutions.push((sub_type_key.to_string(), sub_type_input_options));
    }

    let mut options_combinations: Vec<Vec<usize>> = Vec::new();
    for i in 0..total_combinations {
        let mut options_combination: Vec<usize> = Vec::new();
        let mut remainder = i;
        for j in 0..substitutions.len() {
            let sub_type_options = &substitutions[j].1;
            let sub_type_option_index = remainder % sub_type_options.len();
            options_combination.push(sub_type_option_index);
            remainder = remainder / sub_type_options.len();
        }

        options_combinations.push(options_combination);
    }

    log::info!(
        &["INIT"],
        &format!(
            "Found {} total combinations for test",
            options_combinations.len()
        ),
    );

    let mut results: Vec<(String, Vec<usize>)> = Vec::new();
    for combo in options_combinations {
        let result = _sub_pattern_with_options(&test.input, &substitutions, &combo);
        results.push((result, combo));
    }

    log::pop!();
    results
}

fn _sub_pattern_with_options(
    pattern: &str,
    sub_type_options: &Vec<(String, Vec<String>)>,
    options_combination: &Vec<usize>,
) -> String {
    let mut result = pattern.to_string();
    for i in 0..options_combination.len() {
        let option = &sub_type_options[i].1[options_combination[i]];
        result = result.replacen("{}", &option, 1);
    }

    return result;
}
