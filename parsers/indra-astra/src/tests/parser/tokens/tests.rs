use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    parser::{
        self,
        results::{
            builder::Builder, error::Error, error_builder::ErrorBuilder, node::Node,
            parsed::Parsed, r#match::Token, span::Span, token_builder::TokenBuilder,
        },
    },
    utils::log::{self, Styleable},
};

#[cfg(feature = "log")]
use crate::utils::log::Color;

pub struct Test {
    parser: &'static Rc<dyn parser::Type>,
    tags: Vec<String>,
    input: String,
    expected: Parsed,
    is_partial: bool,
    sub_types: Vec<String>,
    is_disabled: bool,
}

lazy_static! {
    static ref PATTERN_SUB_REGEX: Regex = Regex::new(r"\{([a-z\-]+[\?\*\+]?)\}").unwrap();
}

impl Test {
    #[allow(non_snake_case)]
    pub fn Unit<TParser>(tags: &[&str], input: &str, expected: Parsed) -> Test
    where
        TParser: parser::Type + 'static,
    {
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: input.to_string(),
            expected,
            is_partial: false,
            sub_types: Vec::new(),
            is_disabled: false,
        }
    }

    #[allow(non_snake_case)]
    pub fn Error<TParser>(tags: &[&str], input: &str, expected: Error) -> Test
    where
        TParser: parser::Type + 'static,
    {
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: input.to_string(),
            expected: Parsed::Fail(Some(expected)),
            is_partial: false,
            sub_types: Vec::new(),
            is_disabled: false,
        }
    }

    #[allow(non_snake_case)]
    pub fn Partial<TParser>(tags: &[&str], input: &str, expected: Parsed) -> Test
    where
        TParser: parser::Type + 'static,
    {
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: input.to_string(),
            expected,
            is_partial: true,
            sub_types: Vec::new(),
            is_disabled: false,
        }
    }

    #[allow(non_snake_case)]
    pub fn Pattern<TParser>(tags: &[&str], template: &str, expected: Parsed) -> Test
    where
        TParser: parser::Type + 'static,
    {
        let mut sub_types: Vec<String> = Vec::new();
        for capture in PATTERN_SUB_REGEX.captures_iter(template) {
            sub_types.push(capture[1].to_string());
        }

        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: template.to_string(),
            expected,
            is_partial: false,
            sub_types,
            is_disabled: false,
        }
    }

    #[allow(non_snake_case)]
    pub fn Error_Pattern<TParser>(tags: &[&str], template: &str, expected: Error) -> Test
    where
        TParser: parser::Type + 'static,
    {
        let mut sub_types: Vec<String> = Vec::new();
        for capture in PATTERN_SUB_REGEX.captures_iter(template) {
            sub_types.push(capture[1].to_string());
        }
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: template.to_string(),
            expected: Parsed::Fail(Some(expected)),
            is_partial: false,
            sub_types,
            is_disabled: false,
        }
    }

    #[allow(non_snake_case)]
    pub fn Partial_Pattern<TParser>(tags: &[&str], template: &str, expected: Parsed) -> Test
    where
        TParser: parser::Type + 'static,
    {
        let mut sub_types: Vec<String> = Vec::new();
        for capture in PATTERN_SUB_REGEX.captures_iter(template) {
            sub_types.push(capture[1].to_string());
        }
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: template.to_string(),
            expected,
            is_partial: true,
            sub_types,
            is_disabled: false,
        }
    }

    pub fn disable(&mut self) {
        self.is_disabled = true;
    }

    pub fn is_disabled(&self) -> bool {
        self.is_disabled
            || match self.expected {
                Parsed::Pass(ref token) => token.tags().contains(_DISABLED_TAG),
                Parsed::Fail(ref error) => error
                    .as_ref()
                    .map(|error| error.tags().contains(_DISABLED_TAG))
                    .unwrap_or(false),
            }
    }

    pub fn get_name(&self) -> String {
        self.tags.join(" & ")
    }

    pub fn format_input(&self, decorator: Option<InputDecoration>) -> String {
        _format_input(&self.input, decorator)
    }

    pub fn get_formatted_input(&self) -> String {
        self.format_input(None)
    }

    pub fn run(self) -> Vec<Outcome> {
        let parsers = parser::get_all();
        self.run_with_context(
            &parsers,
            &Settings {
                panic_on_fail: false,
            },
        )
    }

    pub fn run_with_context(self, parsers: ParserMap, settings: &Settings) -> Vec<Outcome> {
        let outcomes: Vec<Outcome>;

        log::color!("TESTS", Color::Yellow);
        log::push_unique!("TESTS");
        log::push!("-");
        log::push!(&self.parser.name());
        log::push!("-");

        if !self.is_disabled() {
            if self.sub_types.len() == 0 {
                outcomes = vec![_run_unit_test(self)];
                _check_for_panic_on_fail(&outcomes.first().unwrap(), settings);
            } else {
                outcomes = _run_tests_for_pattern(self, parsers, settings);
            }
        } else {
            outcomes = vec![];
        }

        log::pop!();
        log::pop!();
        log::pop!();
        log::pop_unique!("TESTS");

        return outcomes;
    }

    pub fn get_all_combinations(self, parsers: ParserMap) -> Vec<Test> {
        if self.is_partial {
            return vec![];
        } else {
            if self.sub_types.len() == 0 {
                return vec![self];
            } else {
                let mut tests: Vec<Test> = Vec::new();
                let pattern = self.input;
                let mut total_combos = 1;
                let subs = PATTERN_SUB_REGEX
                    .captures_iter(&pattern)
                    .map(|c| {
                        let capture = c.get(1).unwrap();
                        let value = capture.as_str();

                        let modifier: Option<char> = if value.ends_with("?") {
                            Some('?')
                        } else if value.ends_with("+") {
                            Some('+')
                        } else if value.ends_with("*") {
                            Some('*')
                        } else {
                            None
                        };

                        let key = if modifier.is_some() {
                            &value[..value.len() - 1]
                        } else {
                            value
                        };

                        let parser = parsers.get(value).unwrap_or_else(|| {
                            panic!(
                                "Unknown parser type: {}, in pattern: \n\t{}",
                                value, pattern,
                            )
                        });

                        let mut tests: Vec<Test> = vec![];
                        for test in parser.get_tests() {
                            let combos = test.get_all_combinations(parsers);
                            tests.extend(combos);
                        }

                        total_combos *= tests.len();
                        ((capture.start(), capture.end()), key, tests, modifier)
                    })
                    .collect::<Vec<((usize, usize), &str, Vec<Test>, Option<char>)>>();

                let mut combos: Vec<Vec<usize>> = Vec::new();
                for combo_index in 0..total_combos {
                    let mut combo: Vec<usize> = Vec::new();
                    let mut remainder = combo_index;
                    for sub_index in 0..subs.len() {
                        let sub = &subs[sub_index];
                        let sub_options_count = sub.2.len();
                        let sub_option_index = remainder % sub_options_count;
                        remainder = remainder / sub_options_count;
                        combo.push(sub_option_index);
                    }

                    combos.push(combo);
                }

                let mut result_patterns: Vec<(String, usize)> = Vec::new();
                for combo in combos {
                    let curr_patterns = &result_patterns.clone();
                    result_patterns = Vec::new();
                    let mut result_patterns: Vec<(String, usize)> = Vec::new();
                    result_patterns.push((pattern.clone(), 0));

                    for (index, sub_index) in combo.iter().enumerate() {
                        let sub = &subs[index];
                        let sub_input = &sub.2[*sub_index].input;

                        for (pattern, offset) in curr_patterns {
                            let start = sub.0 .0 + offset;
                            let end = sub.0 .1 + offset;

                            let mut updated = pattern.clone();
                            updated.replace_range(start..end, sub_input);
                            result_patterns.push((pattern.clone(), *offset));

                            let sub_modifier = sub.3;
                            match sub_modifier {
                                Some('?') => {
                                    _build_and_append_optional_pattern(
                                        sub,
                                        &pattern,
                                        start,
                                        end,
                                        &mut result_patterns,
                                    );
                                }
                                Some('+') => _build_and_append_repeat_patterns(
                                    sub,
                                    &updated,
                                    start,
                                    end,
                                    &mut result_patterns,
                                ),
                                Some('*') => {
                                    _build_and_append_optional_pattern(
                                        sub,
                                        &pattern,
                                        start,
                                        end,
                                        &mut result_patterns,
                                    );
                                    _build_and_append_repeat_patterns(
                                        sub,
                                        &updated,
                                        start,
                                        end,
                                        &mut result_patterns,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }

                    for pattern in curr_patterns {
                        let case = Test {
                            parser: self.parser,
                            tags: self.tags.clone(),
                            input: pattern.0.clone(),
                            expected: self.expected.clone(),
                            is_partial: self.is_partial,
                            sub_types: self.sub_types.clone(),
                            is_disabled: self.is_disabled,
                        };

                        tests.push(case);
                    }
                }

                return tests;
            }
        }
    }
}

fn _build_and_append_optional_pattern(
    sub: &((usize, usize), &str, Vec<Test>, Option<char>),
    pattern: &String,
    start: usize,
    end: usize,
    result_patterns: &mut Vec<(String, usize)>,
) {
    let offset = 0 - (sub.1.len() + 3);
    let optional_pattern = pattern.clone();
    pattern.clone().replace_range(start..end, "");
    result_patterns.push((optional_pattern, offset));
}

fn _build_and_append_repeat_patterns(
    sub: &((usize, usize), &str, Vec<Test>, Option<char>),
    updated_pattern: &String,
    start: usize,
    end: usize,
    result_patterns: &mut Vec<(String, usize)>,
) {
    for i in [1, 2, 3, 5, 8, 13] {
        let offset = (sub.1.len() + 3) * i;
        let mut consistent_repeat_pattern = updated_pattern[0..start].to_owned();
        let mut random_repeat_pattern = updated_pattern[0..start].to_owned();
        let mut index = 0;
        for sub_input in HashSet::<&String>::from_iter(sub.2.iter().map(|t| &t.input)) {
            if index >= i {
                break;
            } else {
                index += 1;
            }

            random_repeat_pattern.push_str(sub_input);
        }

        for sub_input in sub.2.iter().map(|t| &t.input) {
            for _ in 0..i {
                consistent_repeat_pattern.push_str(&sub_input);
            }
        }

        consistent_repeat_pattern.push_str(&updated_pattern[end..]);

        result_patterns.push((random_repeat_pattern, offset));
        result_patterns.push((consistent_repeat_pattern, offset));
    }
}

macro_rules! unit {
    ([$($tag:literal $(&)?)*]: $input:literal => $expected:expr) => {
        Test::Unit::<Self>(&[$($tag,)*], $input, $expected)
    };
}
pub(crate) use unit;

macro_rules! pattern {
    ([$($tag:literal $(&)?)*]: $template:literal => $expected:expr) => {
        {
          let builder: TokenBuilder = $expected;
          Test::Pattern::<Self>(&[$($tag,)*], $template, Parsed::Pass(builder.pattern()))
      }
    };
}
pub(crate) use pattern;

pub(crate) const _PARTIAL_TAG: &str = "_!__PARTIAL__!_";
pub(crate) const _PATTERN_TAG: &str = "_!__PATTERN__!_";
pub(crate) const _DISABLED_TAG: &str = "_!__DISABLED__!_";
pub(crate) const _MOCK_TAG: &str = "_!__MOCK__!_";
pub(crate) const _SUB_TAG: &str = "_!__SUB__!_";

pub trait Mockable<B, R>
where
    B: Builder<R>,
{
    fn partial(self) -> B; // makes it partial (tags, children, and props only check what you provide)
    fn mock(self) -> R; // makes it partial and not check the ends.
    fn sub(self) -> R; // makes it a mock and ends it
    fn pattern(self) -> R; // makes it check the end as the end of the last sub
    fn disable(self) -> R; // disables the test when used on the root builder.
}

impl Mockable<TokenBuilder, Token> for TokenBuilder {
    fn partial(self) -> TokenBuilder {
        self.tag(_PARTIAL_TAG)
    }

    fn mock(self) -> Token {
        self.partial().tag(_MOCK_TAG).partial().build(0, 0)
    }

    fn sub(self) -> Token {
        self.tag(_SUB_TAG).mock()
    }

    fn pattern(self) -> Token {
        self.tag(_PATTERN_TAG).mock()
    }

    fn disable(self) -> Token {
        self.tag(_DISABLED_TAG).mock()
    }
}

impl Mockable<ErrorBuilder, Option<Error>> for ErrorBuilder {
    fn partial(self) -> ErrorBuilder {
        self.tag(_PARTIAL_TAG)
    }

    fn mock(self) -> Option<Error> {
        self.partial().tag(_MOCK_TAG).partial().build(0, 0)
    }

    fn sub(self) -> Option<Error> {
        self.tag(_SUB_TAG).mock()
    }

    fn pattern(self) -> Option<Error> {
        self.tag(_PATTERN_TAG).mock()
    }

    fn disable(self) -> Option<Error> {
        self.tag(_DISABLED_TAG).mock()
    }
}
pub struct Mock;
impl Mock {
    #[allow(non_snake_case)]
    pub fn Token<T>(start: usize, end: usize) -> Token
    where
        T: parser::Type + 'static,
    {
        Token::Of_Type::<T>().partial().build(start, end)
    }

    #[allow(non_snake_case)]
    pub fn Error(name: &str, start: usize, end: usize) -> Option<Error> {
        Error::New(name).partial().build(start, end)
    }

    #[allow(non_snake_case)]
    pub fn Sub<T>() -> Token
    where
        T: parser::Type + 'static,
    {
        Token::Of_Type::<T>().sub()
    }
}

pub enum Outcome {
    Pass(Test),
    Fail(Test, Parsed, String),
}

enum Comparison {
    Pass,
    Fail(String),
}

pub type ParserMap<'p> = &'p HashMap<&'p str, Rc<dyn parser::Type>>;

#[derive(Debug)]
pub struct Settings {
    pub panic_on_fail: bool,
}

pub fn run_all() -> Vec<Outcome> {
    run_all_with_settings(&Settings {
        panic_on_fail: false,
    })
}

pub fn run_all_with_settings(settings: &Settings) -> Vec<Outcome> {
    let parsers = parser::get_all();
    run_all_with_context(parsers, settings)
}

pub fn run_all_with_context(parsers: ParserMap, settings: &Settings) -> Vec<Outcome> {
    let mut outcomes: Vec<Outcome> = Vec::new();
    for parser in parsers.values() {
        let tests = parser.get_tests();
        for test in tests {
            let mut tags = test.tags.clone();
            tags.push(parser.name().to_string());
            let mut test = test;
            test.tags = tags;
            let outcome = run_with_context(test, parsers, settings);
            outcomes.extend(outcome);
        }
    }
    return outcomes;
}

pub fn run(test: Test) -> Vec<Outcome> {
    run_with_context(
        test,
        &parser::get_all(),
        &Settings {
            panic_on_fail: false,
        },
    )
}

pub fn run_with_context(test: Test, parsers: ParserMap, settings: &Settings) -> Vec<Outcome> {
    test.run_with_context(parsers, settings)
}

fn _run_unit_test(test: Test) -> Outcome {
    let parser = test.parser;
    let input = &test.input;
    let expected = &test.expected;

    log::push!(&test.get_name());
    log::plain!(&["START"], &test.get_formatted_input());

    let result = parser.parse(&input);
    let comparison = _validate_outcome(&expected, &result);
    let result = match comparison {
        Comparison::Pass => Outcome::Pass(test),
        Comparison::Fail(message) => Outcome::Fail(test, result, message),
    };

    #[cfg(feature = "log")]
    match &result {
        Outcome::Pass(_test) => {
            log::log!(
                &["END"],
                &format!(
                    "{}{} \n => {}",
                    "✔".color(log::Color::Green),
                    " PASS".color(log::Color::Green),
                    match _test.expected {
                        Parsed::Pass(ref token) => token.name(),
                        Parsed::Fail(ref error) => match error {
                            Some(error) => error.name(),
                            None => "None",
                        },
                    }
                )
            );
        }
        Outcome::Fail(_test, _result, _message) => {
            log::error!(
                &["END"],
                &format!(
                    "{}\n{}{} \t=> {}\n\n{}\n{}\n{}",
                    &_format_input(&_test.input, Some(InputDecoration::XMark)),
                    "✘".color(log::Color::Red),
                    " FAIL".color(log::Color::Red),
                    match _result {
                        Parsed::Pass(ref token) => token.name.clone(),
                        Parsed::Fail(ref error) => match error {
                            Some(error) => error.name.clone(),
                            None => "None".to_string(),
                        },
                    },
                    &format!("\t- Reason:\n{}", _message)
                        .indent(2)
                        .color(log::Color::Yellow),
                    &format!(
                        "\t- Expected:\n{}",
                        match _test.expected {
                            Parsed::Pass(ref token) => format!("{:#?}", token),
                            Parsed::Fail(ref error) => match error {
                                Some(error) => format!("{:#?}", error),
                                None => "None".to_string(),
                            },
                        }
                    )
                    .indent(2)
                    .color(log::Color::Green),
                    &format!(
                        "\t- Actual:\n{}",
                        match _result {
                            Parsed::Pass(ref token) => format!("{:#?}", token),
                            Parsed::Fail(ref error) => match error {
                                Some(error) => format!("{:#?}", error),
                                None => "None".to_string(),
                            },
                        }
                    )
                    .indent(2)
                    .color(log::Color::Red),
                )
            );
        }
    }

    log::pop!();

    return result;
}

fn _run_tests_for_pattern(base: Test, parsers: ParserMap, settings: &Settings) -> Vec<Outcome> {
    let combos = base.get_all_combinations(parsers);
    let mut outcomes: Vec<Outcome> = Vec::new();
    for combo in combos {
        let outcome = _run_unit_test(combo);

        _check_for_panic_on_fail(&outcome, settings);

        outcomes.push(outcome);
    }

    return outcomes;
}

fn _check_for_panic_on_fail(outcome: &Outcome, settings: &Settings) {
    if let Outcome::Fail(test, _, _) = outcome {
        if settings.panic_on_fail {
            panic!("Test failed: {}", test.get_name(),);
        }
    }
}

fn _validate_outcome(expected: &Parsed, result: &Parsed) -> Comparison {
    match result {
        Parsed::Pass(resulting_pass) => match expected {
            Parsed::Pass(expected_pass) => _compare_tokens(expected_pass, resulting_pass),
            Parsed::Fail(expected_failure) => Comparison::Fail(format!(
                "Expected failure of type: {}, but found pass of type: {}.",
                match &expected_failure {
                    Some(err) => err.name.clone(),
                    None => "None".to_string(),
                },
                resulting_pass.name
            )),
        },
        Parsed::Fail(resulting_failure) => match expected {
            Parsed::Pass(expected_pass) => Comparison::Fail(format!(
                "Expected pass of type: {}, but found {}",
                expected_pass.name,
                match resulting_failure {
                    Some(err) => format!("error: {}; {}", err.name.clone(), err.get_message()),
                    None => "None".to_string(),
                }
            )),
            Parsed::Fail(expected_failure) => {
                _compare_errors(expected_failure.clone(), resulting_failure.clone())
            }
        },
    }
}

fn _compare_tokens(expected: &Token, result: &Token) -> Comparison {
    match _compare_name(&expected.name, &result.name, &expected.tags) {
        Comparison::Pass => match _compare_sizes(expected, result, &expected.tags) {
            Comparison::Pass => match _compare_tags(&expected.tags, &result.tags) {
                Comparison::Pass => {
                    match _compare_children(
                        if expected.children.len() == 0 {
                            None
                        } else {
                            Some(
                                expected
                                    .children
                                    .iter()
                                    .map(|c| Parsed::Pass(c.clone()))
                                    .collect(),
                            )
                        },
                        if result.children.len() == 0 {
                            None
                        } else {
                            Some(
                                result
                                    .children
                                    .iter()
                                    .map(|c| Parsed::Pass(c.clone()))
                                    .collect(),
                            )
                        },
                        &expected.tags,
                    ) {
                        Comparison::Pass => {
                            match _compare_props(&expected.keys, &result.keys, &expected.tags) {
                                Comparison::Pass => Comparison::Pass,
                                Comparison::Fail(message) => Comparison::Fail(message),
                            }
                        }
                        Comparison::Fail(message) => Comparison::Fail(message),
                    }
                }
                Comparison::Fail(message) => Comparison::Fail(message),
            },
            Comparison::Fail(message) => Comparison::Fail(message),
        },
        Comparison::Fail(message) => Comparison::Fail(message),
    }
}

fn _compare_errors(expected: Option<Error>, result: Option<Error>) -> Comparison {
    match (expected, result) {
        (Some(expected), Some(result)) => {
            match _compare_name(&expected.name, &result.name, &expected.tags) {
                Comparison::Pass => match _compare_sizes(&expected, &result, &expected.tags) {
                    Comparison::Pass => match _compare_tags(&expected.tags, &result.tags) {
                        Comparison::Pass => match _compare_children(
                            if &expected.children.len() == &0 {
                                None
                            } else {
                                Some(expected.children.clone())
                            },
                            if result.children.len() == 0 {
                                None
                            } else {
                                Some(result.children.clone())
                            },
                            &expected.tags,
                        ) {
                            Comparison::Pass => {
                                match _compare_props(&expected.keys, &result.keys, &expected.tags) {
                                    Comparison::Pass => Comparison::Pass,
                                    Comparison::Fail(message) => Comparison::Fail(message),
                                }
                            }
                            Comparison::Fail(message) => Comparison::Fail(message),
                        },
                        Comparison::Fail(message) => Comparison::Fail(message),
                    },
                    Comparison::Fail(message) => Comparison::Fail(message),
                },
                Comparison::Fail(message) => Comparison::Fail(message),
            }
        }
        (Some(expected), None) => Comparison::Fail(format!(
            "Expected error of type: {}, but found None.",
            expected.name,
        )),
        (None, Some(result)) => Comparison::Fail(format!(
            "Expected None, but found error: {}; {}.",
            result.name,
            result.get_message()
        )),
        (None, None) => Comparison::Pass,
    }
}

fn _compare_name(expected: &str, result: &str, tags: &Option<HashSet<String>>) -> Comparison {
    if let Some(tags) = tags
        && tags.contains(_PARTIAL_TAG)
        && expected == ""
    {
        return Comparison::Pass;
    }

    if expected == result {
        return Comparison::Pass;
    }

    Comparison::Fail(format!(
        "Expected name to be: {}, but found: {}.",
        expected, result
    ))
}

fn _compare_sizes(
    expected: &dyn Span,
    result: &dyn Span,
    tags: &Option<HashSet<String>>,
) -> Comparison {
    if let Some(tags) = tags
        && tags.contains(_MOCK_TAG)
    {
        return Comparison::Pass;
    }

    if expected.start() != result.start() {
        return Comparison::Fail(format!(
            "Expected first character to be at index: {}, but found: {}.",
            expected.start(),
            result.start()
        ));
    }

    if expected.end() != result.end() {
        return Comparison::Fail(format!(
            "Expected last character to be at index: {}, but found: {}.",
            expected.end() - expected.start(),
            result.end() - result.start()
        ));
    }

    Comparison::Pass
}

fn _compare_tags(
    expected: &Option<HashSet<String>>,
    result: &Option<HashSet<String>>,
) -> Comparison {
    if let Some(expected) = expected {
        if let Some(result) = result {
            if expected.contains(_PARTIAL_TAG) {
                if expected.is_subset(result) {
                    return Comparison::Pass;
                } else {
                    return Comparison::Fail(format!(
                        "Expected tags to be subset of: {:?}, but found: {:?}.",
                        expected, result
                    ));
                }
            } else {
                if expected == result {
                    return Comparison::Pass;
                } else {
                    return Comparison::Fail(format!(
                        "Expected tags to be exactly: {:?}, but found: {:?}.",
                        expected, result
                    ));
                }
            }
        } else {
            if expected.contains(_PARTIAL_TAG) && expected.len() == 0 {
                return Comparison::Pass;
            } else {
                return Comparison::Fail(format!(
                    "Expected tags to be exactly: {:?}, but found: None.",
                    result
                ));
            }
        }
    } else {
        if let Some(result) = result {
            return Comparison::Fail(format!("Expected no tags, but found: {:?}.", result));
        } else {
            return Comparison::Pass;
        }
    }
}

fn _compare_children(
    expected: Option<Vec<Parsed>>,
    result: Option<Vec<Parsed>>,
    tags: &Option<HashSet<String>>,
) -> Comparison {
    if let Some(tags) = tags
        && tags.contains(_PARTIAL_TAG)
    {
        if let Some(expected) = expected {
            if let Some(result) = result {
                let mut result_index = 0;
                let mut num_found = 0;
                let mut last_e: Option<&Parsed> = None;
                for e in &expected {
                    last_e = Some(e);
                    if result_index == result.len() {
                        break;
                    }

                    loop {
                        if result_index == result.len() {
                            break;
                        }

                        let r = &result[result_index];
                        result_index += 1;

                        if let Comparison::Pass = _validate_outcome(&e, r) {
                            num_found += 1;
                            break;
                        }
                    }
                }

                if num_found == expected.len() {
                    return Comparison::Pass;
                } else {
                    return Comparison::Fail(format!(
                        "Expected to find child token of type: {}, but found end of parent instead.",
                         match last_e {
                            Some(e) => e.get_name().to_string(),
                            None => "none".to_string(),
                        }
                    ));
                }
            } else {
                return Comparison::Fail(format!(
                    "Expected to find at least {} children, but found None.",
                    expected.len()
                ));
            }
        } else {
            return Comparison::Pass;
        }
    } else {
        if let Some(expected) = expected {
            if let Some(result) = result {
                if expected.len() != result.len() {
                    return Comparison::Fail(format!(
                        "Expected {} children, but found {}.",
                        expected.len(),
                        result.len()
                    ));
                }

                for (index, expected_child) in expected.iter().enumerate() {
                    let result_child = &result[index];
                    match _validate_outcome(expected_child, result_child) {
                        Comparison::Pass => {}
                        Comparison::Fail(message) => {
                            return Comparison::Fail(format!(
                                "Child of type {}, at index {}, failed: {}",
                                result_child.get_name(),
                                index,
                                message
                            ))
                        }
                    }
                }

                return Comparison::Pass;
            } else {
                return Comparison::Fail(format!(
                    "Expected {} children, but found None.",
                    expected.len()
                ));
            }
        } else {
            if let Some(result) = result {
                return Comparison::Fail(format!(
                    "Expected None, but found {} children.",
                    result.len()
                ));
            } else {
                return Comparison::Pass;
            }
        }
    }
}

fn _compare_props(
    expected_keys: &Option<HashMap<String, usize>>,
    result_keys: &Option<HashMap<String, usize>>,
    tags: &Option<HashSet<String>>,
) -> Comparison {
    if let Some(tags) = tags
        && tags.contains(_PARTIAL_TAG)
    {
        if let Some(expected) = expected_keys {
            if let Some(result) = result_keys {
                for (key, expected_value) in expected {
                    if let Some(result_value) = result.get(key) {
                        if expected_value == result_value {
                            continue;
                        } else {
                            return Comparison::Fail(format!(
                                "Expected prop: {} to be: {}, but found: {}.",
                                key, expected_value, result_value
                            ));
                        }
                    } else {
                        return Comparison::Fail(format!(
                            "Expected prop: {} to be: {}, but found: None.",
                            key, expected_value
                        ));
                    }
                }

                return Comparison::Pass;
            } else {
                return Comparison::Fail(format!(
                    "Expected props: {:?}, but found: None.",
                    expected.keys()
                ));
            }
        } else {
            return Comparison::Pass;
        }
    } else {
        if let Some(expected) = expected_keys {
            if let Some(result) = result_keys {
                if expected.len() != result.len() {
                    return Comparison::Fail(format!(
                        "Expected props: {:?}, but found: {:?}.",
                        expected.keys(),
                        result.keys()
                    ));
                } else {
                    for (key, expected_value) in expected {
                        if let Some(result_value) = result.get(key) {
                            if expected_value == result_value {
                                continue;
                            } else {
                                return Comparison::Fail(format!(
                                    "Expected prop: {} to be: {}, but found: {}.",
                                    key, expected_value, result_value
                                ));
                            }
                        } else {
                            return Comparison::Fail(format!(
                                "Expected prop: {} to be: {}, but found: None.",
                                key, expected_value
                            ));
                        }
                    }

                    return Comparison::Pass;
                }
            } else {
                return Comparison::Fail(format!(
                    "Expected props: {:?}, but found: None.",
                    expected.keys()
                ));
            }
        } else {
            if let Some(result) = result_keys {
                return Comparison::Fail(format!(
                    "Expected None, but found props: {:?}.",
                    result.keys()
                ));
            } else {
                return Comparison::Pass;
            }
        }
    }
}

pub enum InputDecoration {
    CheckMark,
    XMark,
}

fn _format_input(input: &str, decorator: Option<InputDecoration>) -> String {
    format!(
        "\n{}┏{}\n\t┖",
        match decorator {
            Some(InputDecoration::CheckMark) => "✔\t".color(log::Color::Green),
            Some(InputDecoration::XMark) => "✘\t".color(log::Color::Red),
            None => "\t".to_string(),
        },
        format!("\n{}", input).replace("\n", "\n\t┣ "),
    )
}
