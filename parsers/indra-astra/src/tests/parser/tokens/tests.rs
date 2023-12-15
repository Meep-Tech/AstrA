use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::parser::{
    self,
    results::{
        builder::Builder, error::Error, error_builder::ErrorBuilder, node::Node, parsed::Parsed,
        r#match::Match, span::Span, token_builder::TokenBuilder,
    },
};

pub struct Test {
    parser: &'static Rc<dyn parser::Type>,
    tags: Vec<String>,
    input: String,
    expected: Parsed,
    is_partial: bool,
    sub_types: Vec<String>,
    is_disabled: bool,
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
            is_disabled: false,
        }
    }

    #[allow(non_snake_case)]
    pub fn Error_Pattern<TParser>(tags: &[&str], template: &str, expected: Error) -> Test
    where
        TParser: parser::Type + 'static,
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
            is_disabled: false,
        }
    }

    #[allow(non_snake_case)]
    pub fn Partial_Pattern<TParser>(tags: &[&str], template: &str, expected: Parsed) -> Test
    where
        TParser: parser::Type + 'static,
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

impl Mockable<TokenBuilder, Match> for TokenBuilder {
    fn partial(self) -> TokenBuilder {
        self.tag(_PARTIAL_TAG)
    }

    fn mock(self) -> Match {
        self.partial().tag(_MOCK_TAG).partial().build(0, 0)
    }

    fn sub(self) -> Match {
        self.tag(_SUB_TAG).mock()
    }

    fn pattern(self) -> Match {
        self.tag(_PATTERN_TAG).mock()
    }

    fn disable(self) -> Match {
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
    pub fn Token<T>(start: usize, end: usize) -> Match
    where
        T: parser::Type + 'static,
    {
        Match::Of_Type::<T>().partial().build(start, end)
    }

    #[allow(non_snake_case)]
    pub fn Error(name: &str, start: usize, end: usize) -> Option<Error> {
        Error::New(name).partial().build(start, end)
    }

    #[allow(non_snake_case)]
    pub fn Sub<T>() -> Match
    where
        T: parser::Type + 'static,
    {
        Match::Of_Type::<T>().sub()
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

pub fn run_all() -> Vec<Outcome> {
    let parsers = parser::get_all();
    run_for(parsers)
}

pub fn run_for(parsers: ParserMap) -> Vec<Outcome> {
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

pub fn run(test: Test, parsers: ParserMap) -> Vec<Outcome> {
    if test.is_disabled() {
        return Vec::new();
    }

    if test.sub_types.len() == 0 {
        return vec![_run_unit_test(test, parsers)];
    } else {
        return _run_tests_for_pattern(test, parsers);
    }
}

fn _run_unit_test(test: Test, parsers: ParserMap) -> Outcome {
    let parser = test.parser;
    let input = &test.input;
    let expected = &test.expected;

    let result = parser.parse(&input);
    let comparison = _validate_outcome(&expected, &result);
    match comparison {
        Comparison::Pass => Outcome::Pass(test),
        Comparison::Fail(message) => Outcome::Fail(test, result, message),
    }
}

fn _run_tests_for_pattern(base: Test, parsers: ParserMap) -> Vec<Outcome> {
    todo!()
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
                "Expected pass of type: {}, but found failure of type: {}.",
                expected_pass.name,
                match resulting_failure {
                    Some(err) => err.name.clone(),
                    None => "None".to_string(),
                }
            )),
            Parsed::Fail(expected_failure) => {
                _compare_errors(expected_failure.clone(), resulting_failure.clone())
            }
        },
    }
}

fn _compare_tokens(expected: &Match, result: &Match) -> Comparison {
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
                            match _compare_token_props(&expected, &result, &expected.tags) {
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
                                match _compare_error_props(&expected, &result, &expected.tags) {
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
            "Expected None, but found error of type: {}.",
            result.name
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
            return Comparison::Fail(format!("Expected no tags, but found: {:?}.", expected));
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
                            Some(e) => match e {
                                Parsed::Pass(e) => e.name.clone(),
                                Parsed::Fail(e) => match e {
                                    Some(e) => e.name.clone(),
                                    None => "None".to_string(),
                                },
                            },
                            None => "None".to_string(),
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
                            return Comparison::Fail(format!("Child {} failed: {}", index, message))
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

fn _compare_token_props(
    expected: &Match,
    result: &Match,
    tags: &Option<HashSet<String>>,
) -> Comparison {
    todo!();
}

fn _compare_error_props(
    expected: &Error,
    result: &Error,
    tags: &Option<HashSet<String>>,
) -> Comparison {
    todo!();
}
