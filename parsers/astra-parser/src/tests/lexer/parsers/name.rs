use crate::lexer::{
    parsers::name,
    results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
};

use super::test::{Test, Testable};

impl Testable for crate::lexer::parsers::name::Parser {
    fn tests() -> Vec<Test<Self>> {
        return vec![
            Test::<Self>::new(
                "Alphabetic",
                "abc",
                Parsed::Token(Token::new().name(name::KEY).build(0, 2)),
            ),
            Test::<Self>::new(
                "Alphabetic & Dash",
                "abc-def",
                Parsed::Token(Token::new().name(name::KEY).build(0, 6)),
            ),
            Test::<Self>::new(
                "Alphabetic & Underscore",
                "abc_",
                Parsed::Token(Token::new().name(name::KEY).build(0, 3)),
            ),
            Test::<Self>::new(
                "Alphabetic & Underscore & Dash",
                "abc_efg-hij",
                Parsed::Token(Token::new().name(name::KEY).build(0, 10)),
            ),
            Test::<Self>::new(
                "Alphanumeric & Numeric Start",
                "123abc",
                Parsed::Token(Token::new().name(name::KEY).build(0, 5)),
            ),
            Test::<Self>::new(
                "Alphanumeric & Underscore",
                "abc_123",
                Parsed::Token(Token::new().name(name::KEY).build(0, 6)),
            ),
            Test::<Self>::new(
                "Alphanumeric & Underscore & Dash",
                "abc_123-456",
                Parsed::Token(Token::new().name(name::KEY).build(0, 10)),
            ),
            Test::<Self>::new(
                "Alphabetic & Double Dash & Error",
                "abc--def",
                Parsed::Error(Error::new("unexpected-repeat-lone-symbol-in-name").build(0, 3)),
            ),
            Test::<Self>::new(
                "Alphabetic & Double Underscore",
                "abc__def",
                Parsed::Token(Token::new().name(name::KEY).build(0, 7)),
            ),
            Test::<Self>::new(
                "Alphabetic & Double Underscore & Dash",
                "abc__def-ghi",
                Parsed::Token(Token::new().name(name::KEY).build(0, 11)),
            ),
            Test::<Self>::new(
                "Alphanumeric & Numeric End",
                "abc123",
                Parsed::Token(Token::new().name(name::KEY).build(0, 5)),
            ),
            Test::<Self>::new(
                "Alphanumeric & Numeric Middle",
                "abc123def",
                Parsed::Token(Token::new().name(name::KEY).build(0, 8)),
            ),
            Test::<Self>::new(
                "Alphanumeric & Alphabetic Middle & Numeric End & Numeric Start",
                "123abc456",
                Parsed::Token(Token::new().name(name::KEY).build(0, 8)),
            ),
            Test::<Self>::new(
                "Alphanumeric & Alphabetic Middle & Numeric End & Numeric Start & Underscore",
                "123abc_456",
                Parsed::Token(Token::new().name(name::KEY).build(0, 9)),
            ),
            Test::<Self>::new(
                "Alphanumeric & Alphabetic Middle & Numeric End & Numeric Start & Underscore & Dash",
                "123abc_456-789",
                Parsed::Token(Token::new().name(name::KEY).build(0, 13)),
            ),
            Test::<Self>::new(
                "Alphanumeric & Alphabetic Middle & Numeric End & Numeric Start & Underscore & Double Dash & Error",
                "123abc_456--789",
                Parsed::Error(Error::new("unexpected-repeat-lone-symbol-in-name").build(0, 10)),
            ),
            Test::<Self>::new(
              "Numeric & Error",
              "123",
              Parsed::Error(Error::new("unexpected-pure-numeric-key-in-name").build(0, 2)),
            ),
            Test::<Self>::new(
              "Numeric Start & Numeric End & Underscore & Error",
              "123_123",
              Parsed::Error(Error::new("unexpected-pure-numeric-key-in-name").build(0, 6)),
            ),
            Test::<Self>::new(
              "Alphabetic & Underscore End & Ends Early",
              "abc_",
              Parsed::Token(Token::new().name(name::KEY).build(0, 3)),
            ),
            Test::<Self>::new(
              "Dash Start & Error",
              "-abc",
              Parsed::Error(Error::new("unexpected-first-letter-in-name").build(0, 0)),
            ),
            Test::<Self>::new(
              "Dash End & Error",
              "abc-",
              Parsed::Error(Error::new("unexpected-last-letter-in-name").build(3, 3)),
            ),
        ];
    }
}
