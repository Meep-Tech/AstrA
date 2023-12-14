use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::parser::{
    parser,
    results::{
        builder::Builder, error::Error, error_builder::ErrorBuilder, node::Node, parsed::Parsed,
        token::Token, token_builder::TokenBuilder,
    },
};

pub struct Test {
    parser: &'static Rc<dyn parser::Parser>,
    tags: Vec<String>,
    input: String,
    expected: Parsed,
    is_partial: bool,
    sub_types: Vec<String>,
}

impl Test {
    #[allow(non_snake_case)]
    pub fn Unit<TParser>(tags: &[&str], input: &str, expected: Parsed) -> Test
    where
        TParser: parser::Parser,
    {
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: input.to_string(),
            expected,
            is_partial: false,
            sub_types: Vec::new(),
        }
    }

    #[allow(non_snake_case)]
    pub fn Error<TParser>(tags: &[&str], input: &str, expected: Error) -> Test
    where
        TParser: parser::Parser,
    {
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: input.to_string(),
            expected: Parsed::Fail(Some(expected)),
            is_partial: false,
            sub_types: Vec::new(),
        }
    }

    #[allow(non_snake_case)]
    pub fn Partial<TParser>(tags: &[&str], input: &str, expected: Parsed) -> Test
    where
        TParser: parser::Parser,
    {
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: input.to_string(),
            expected,
            is_partial: true,
            sub_types: Vec::new(),
        }
    }

    #[allow(non_snake_case)]
    pub fn Pattern<TParser>(tags: &[&str], template: &str, expected: Parsed) -> Test
    where
        TParser: parser::Parser,
    {
        let replacement_pattern = Regex::new(r"\{%([a-z_]+)%}").unwrap();
        let mut sub_types: Vec<String> = Vec::new();
        for capture in replacement_pattern.captures_iter(template) {
            sub_types.push(capture[1].to_string());
        }

        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: template.to_string(),
            expected,
            is_partial: false,
            sub_types,
        }
    }

    #[allow(non_snake_case)]
    pub fn Error_Pattern<TParser>(tags: &[&str], template: &str, expected: Error) -> Test
    where
        TParser: parser::Parser,
    {
        let replacement_pattern = Regex::new(r"\{%([a-z_]+)%}").unwrap();
        let mut sub_types: Vec<String> = Vec::new();
        for capture in replacement_pattern.captures_iter(template) {
            sub_types.push(capture[1].to_string());
        }
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: template.to_string(),
            expected: Parsed::Fail(Some(expected)),
            is_partial: false,
            sub_types,
        }
    }

    #[allow(non_snake_case)]
    pub fn Partial_Pattern<TParser>(tags: &[&str], template: &str, expected: Parsed) -> Test
    where
        TParser: parser::Parser,
    {
        let replacement_pattern = Regex::new(r"\{%([a-z_]+%)}").unwrap();
        let mut sub_types: Vec<String> = Vec::new();
        for capture in replacement_pattern.captures_iter(template) {
            sub_types.push(capture[1].to_string());
        }
        Test {
            parser: TParser::Get(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            input: template.to_string(),
            expected,
            is_partial: true,
            sub_types,
        }
    }
}

macro_rules! unit {
    ([$($tag:literal $(&)?)*]: $input:literal => $expected:expr) => {
        Test::Unit::<Parser>(&[$($tag,)*], $input, $expected)
    };
}
pub(crate) use unit;

macro_rules! pattern {
    ([$($tag:literal $(&)?)*]: $template:literal => $expected:expr) => {
        {
          let builder: TokenBuilder = $expected;
          Test::Pattern::<Parser>(&[$($tag,)*], $template, Parsed::Pass(builder.pattern()))
      }
    };
}
pub(crate) use pattern;

pub(crate) const _PARTIAL_TAG: &str = "_!__PARTIAL__!_";
pub(crate) const _PATTERN_TAG: &str = "_!__PATTERN__!_";
pub(crate) const _DISABLED_TAG: &str = "_!__DISABLED__!_";
pub(crate) const _MOCK_TAG: &str = "_!__MOCK__!_";
pub(crate) const _SUB_TAG: &str = "_!__SUB__!_";

pub struct Mock;
impl Mock {
    #[allow(non_snake_case)]
    pub fn Token<T>(start: usize, end: usize) -> TokenBuilder
    where
        T: parser::Parser,
    {
        Token::Of_Type::<T>()
    }

    #[allow(non_snake_case)]
    pub fn Error<T>(start: usize, end: usize) -> TokenBuilder
    where
        T: parser::Parser,
    {
        Token::Of_Type::<T>()
    }

    #[allow(non_snake_case)]
    pub fn Sub<T>() -> Token
    where
        T: parser::Parser,
    {
        Token::Of_Type::<T>().sub()
    }
}

pub trait Mockable<B, R>
where
    B: Builder<R>,
{
    fn partial(&self) -> B; // makes it partial
    fn mock(&self) -> R; // makes it partial and not check the ends.
    fn sub(&self) -> R; // makes it a mock and ends it
    fn pattern(&self) -> R; // makes it check the end as the end of the last sub
    fn disable(&self) -> R; // disables the test when used on the root builder.
}

impl Mockable<TokenBuilder, Token> for TokenBuilder {
    fn partial(&self) -> TokenBuilder {
        self.tag(_PARTIAL_TAG)
    }

    fn mock(&self) -> Token {
        self.partial().tag(_MOCK_TAG).partial().build(0, 0)
    }

    fn sub(&self) -> Token {
        self.tag(_SUB_TAG).mock()
    }

    fn pattern(&self) -> Token {
        self.tag(_PATTERN_TAG).mock()
    }

    fn disable(&self) -> Token {
        self.tag(_DISABLED_TAG).mock()
    }
}

impl Mockable<ErrorBuilder, Option<Error>> for ErrorBuilder {
    fn partial(&self) -> ErrorBuilder {
        self.tag(_PARTIAL_TAG)
    }

    fn mock(&self) -> Option<Error> {
        self.partial().tag(_MOCK_TAG).partial().build(0, 0)
    }

    fn sub(&self) -> Option<Error> {
        self.tag(_SUB_TAG).mock()
    }

    fn pattern(&self) -> Option<Error> {
        self.tag(_PATTERN_TAG).mock()
    }

    fn disable(&self) -> Option<Error> {
        self.tag(_DISABLED_TAG).mock()
    }
}

enum Outcome {
    Pass(Test),
    Fail(Test, Parsed, String),
}

enum Comparison {
    Pass,
    Fail(String),
}

pub fn run_for(parsers: &'static HashMap<&'static str, &Rc<dyn parser::Parser>>) -> Vec<Outcome> {
    let mut outcomes: Vec<Outcome> = Vec::new();
    for parser in parsers.values() {
        let tests = parser.get_tests();
        for test in tests {
            let mut tags = test.tags.clone();
            tags.push(parser.name().to_string());
            let mut test = test;
            test.tags = tags;
            let outcome = run(test, parsers);
            outcomes.extend(outcome);
        }
    }
    return outcomes;
}

fn run(
    test: Test,
    parsers: &'static HashMap<&'static str, &Rc<dyn parser::Parser>>,
) -> Vec<Outcome> {
    if test.sub_types.len() == 0 {
        vec![_run_unit_test(test, parsers)]
    } else {
        _run_tests_for_pattern(test, parsers)
    }
}

fn _run_unit_test(
    test: Test,
    parsers: &'static HashMap<&'static str, &Rc<dyn parser::Parser>>,
) -> Outcome {
    let parser = test.parser;
    let input = test.input;
    let expected = test.expected;

    let result = parser.parse(&input);
    let comparison = _validate_outcome(expected, result);
    match comparison {
        Comparison::Pass => Outcome::Pass(test),
        Comparison::Fail(message) => Outcome::Fail(test, result, message),
    }
}

fn _run_tests_for_pattern(
    base: Test,
    parsers: &'static HashMap<&'static str, &Rc<dyn parser::Parser>>,
) -> Vec<Outcome> {
}

fn _validate_outcome(expected: Parsed, result: Parsed) -> Comparison {
    match result {
        Parsed::Pass(resulting_pass) => match expected {
            Parsed::Pass(expected_pass) => _compare_tokens(expected_pass, resulting_pass),
            Parsed::Fail(expected_failure) => {}
        },
        Parsed::Fail(resulting_failure) => match expected {
            Parsed::Pass(expected_pass) => {}
            Parsed::Fail(expected_failure) => {}
        },
    }
}

macro_rules! try_validation {
    ($method:ident, $status:ident, $tags:ident, $expected:ident, $result:ident) => {
        $status = $method($expected, $result, $tags);
        if (!$status.0) {
            return $status;
        }
    };
}
pub(crate) use try_validation;

fn _compare_tokens(expected: Token, result: Token) -> Comparison {
    let mut status: Comparison = Comparison::Pass;
    try_validation!(
        _compare_name,
        status,
        expected.tags,
        expected.name,
        result.name
    );

    status
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

fn _validate_token_size(expected: Token, result: Token) -> Comparison {
    if expected.tag(_MOCK_TAG) {
        return Comparison::Pass;
    }

    if expected.start != result.start {
        return Comparison::Fail(format!(
            "Expected first character to be at index: {}, but found: {}.",
            expected.start, result.start
        ));
    }

    if expected.end != result.end {
        return Comparison::Fail(format!(
            "Expected last character to be at index: {}, but found: {}.",
            expected.end - expected.start,
            result.end - result.start
        ));
    }

    Comparison::Pass
}

fn _compare_token_tags(expected: Token, result: Token) -> Comparison {
    todo!();
}

fn _compare_token_children(expected: Token, result: Token) -> Comparison {
    todo!();
}

fn _compare_token_props(expected: Token, result: Token) -> Comparison {
    todo!();
}
