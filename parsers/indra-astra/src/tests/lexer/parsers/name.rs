use crate::lexer::{
    parsers::statement::expression::invocation::identifier::key::name,
    results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
};

use super::test::{Test, Testable};

impl Testable for name::Parser {
    fn get_tests(&self) -> Vec<Test> {
        return vec![
            Test::tags::<Self>(
                &["Alphabetic"],
                "abc",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 2)),
            ),
            Test::tags::<Self>(
                &["Alphabetic", "Single Dash in Middle"],
                "abc-def",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 6)),
            ),
            Test::tags::<Self>(
                &["Alphabetic", "Underscore", "Dash"],
                "abc_efg-hij",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 10)),
            ),
            Test::tags::<Self>(
                &["Alphanumeric", "Numeric Start"],
                "123abc",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 5)),
            ),
            Test::tags::<Self>(
                &["Alphanumeric", "Underscore"],
                "abc_123",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 6)),
            ),
            Test::tags::<Self>(
                &["Alphanumeric", "Underscore", "Dash"],
                "abc_123-456",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 10)),
            ),
            Test::tags::<Self>(
                &["Alphabetic", "Double Dash", "Error"],
                "abc--def",
                Parsed::Fail(Error::new("unexpected-repeat-lone-symbol-in-name").build(0, 3)),
            ),
            Test::tags::<Self>(
                &["Alphabetic", "Double Underscore"],
                "abc__def",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 7)),
            ),
            Test::tags::<Self>(
                &["Alphabetic", "Double Underscore", "Dash"],
                "abc__def-ghi",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 11)),
            ),
            Test::tags::<Self>(
                &["Alphanumeric", "Numeric End"],
                "abc123",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 5)),
            ),
            Test::tags::<Self>(
                &["Alphanumeric", "Numeric Middle"],
                "abc123def",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 8)),
            ),
            Test::tags::<Self>(
                &[
                    "Alphanumeric",
                    "Alphabetic Middle",
                    "Numeric End",
                    "Numeric Start",
                ],
                "123abc456",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 8)),
            ),
            Test::tags::<Self>(
                &[
                    "Alphanumeric",
                    "Alphabetic Middle",
                    "Numeric End",
                    "Numeric Start",
                    "Underscore",
                ],
                "123abc_456",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 9)),
            ),
            Test::tags::<Self>(
                &[
                    "Alphanumeric",
                    "Alphabetic Middle",
                    "Numeric End",
                    "Numeric Start",
                    "Underscore",
                    "Dash",
                ],
                "123abc_456-789",
                Parsed::Pass(Token::new().name(name::KEY).build(0, 13)),
            ),
            Test::tags::<Self>(
                &[
                    "Alphanumeric",
                    "Alphabetic Middle",
                    "Numeric End",
                    "Numeric Start",
                    "Underscore",
                    "Double Dash",
                    "Error",
                ],
                "123abc_456--789",
                Parsed::Fail(Error::new("unexpected-repeat-lone-symbol-in-name").build(0, 10)),
            ),
            Test::tags::<Self>(
                &["Numeric", "Error"],
                "123",
                Parsed::Fail(Error::new("unexpected-pure-numeric-key-in-name").build(0, 2)),
            ),
            Test::tags::<Self>(
                &["Numeric Start", "Numeric End", "Underscore", "Error"],
                "123_123",
                Parsed::Fail(Error::new("unexpected-pure-numeric-key-in-name").build(0, 6)),
            ),
            Test::tags::<Self>(
                &["Numeric", "Underscore End", "Error"],
                "123_",
                Parsed::Fail(Error::new("unexpected-pure-numeric-key-in-name").build(0, 3)),
            ),
            Test::tags::<Self>(
                &["Alphabetic", "Underscore End", "Error"],
                "abc_",
                Parsed::Fail(Error::new("unexpected-last-letter-in-name").build(0, 3)),
            ),
            Test::tags::<Self>(
                &["Alphabetic", "Underscore Start", "Error"],
                "_abc",
                Parsed::Fail(Error::new("unexpected-first-letter-in-name").build(0, 0)),
            ),
            Test::tags::<Self>(
                &["Dash Start", "Error"],
                "-abc",
                Parsed::Fail(Error::new("unexpected-first-letter-in-name").build(0, 0)),
            ),
            Test::tags::<Self>(
                &["Dash End", "Error"],
                "abc-",
                Parsed::Fail(Error::new("unexpected-last-letter-in-name").build(0, 3)),
            ),
        ];
    }
}
