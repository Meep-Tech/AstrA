use crate::lexer::{
    parsers::{
        statement::assignment::entry::named_entry,
        statement::expression::{
            invocation::identifier::key::name, literal::markup::element::text,
        },
        symbol::operator::assigner::mutable_field_assigner,
        whitespace::indent,
    },
    results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
};

use super::test::{IsFrom, Test, Testable};

impl Testable for named_entry::Parser {
    fn get_tests(&self) -> Vec<Test> {
        return vec![
            Test::new::<Self>(
                "One Line & Spaced Right",
                "name: value",
                Parsed::Pass(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key", Token::new()
                            .name(name::KEY)
                            .build(0, 3))
                        .prop("operator", Token::new()
                            .name(mutable_field_assigner::KEY)
                            .build(4, 4))
                        .prop("value", Token::new()
                            .name(text::KEY)
                            .build(6, 9))
                        .build(0, 9),
                ),
            ),
            Test::new::<Self>(
                "One Line & Spaced Around",
                "name : value",
                Parsed::Pass(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key",
                            Token::new()
                                .name(name::KEY)
                                .build(0, 3))
                        .prop("operator",
                            Token::new()
                                .name(mutable_field_assigner::KEY)
                                .build(5, 5))
                        .prop("value",
                            Token::new()
                                .name(text::KEY)
                                .build(7, 10))
                        .build(0, 10),
                ),
            ),
            Test::new::<Self>(
                "One Line & Not-Spaced & Error",
                "name:value",
                Parsed::Fail(
                    Error::new("incomplete-named-entry")
                        .prop("key", Parsed::Pass(IsFrom::<name::Parser>()))
                        .prop(
                            "operator",
                            Parsed::Fail(
                                Error::new(
                                    "missing-expected-trailing-whitespace-in-mutable-field-assigner"
                                ).build(4, 4),
                            ),
                        )
                        .build(0, 4),
                ),
            ),
            Test::new::<Self>(
                "Two Lines & Spaced Indent Increase",
                "name:\n  value",
                Parsed::Pass(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key",
                            Token::new()
                                .name(name::KEY)
                                .build(0, 3))
                        .prop("operator",
                            Token::new()
                                .name(mutable_field_assigner::KEY)
                                .build(4, 4))
                        .child(
                            Token::new()
                                .name(indent::increase::KEY)
                                .build(5, 7))
                        .prop("value",
                            Token::new()
                                .name(text::KEY)
                                .build(8, 11))
                        .build(0, 11),
                )
            ),
            Test::new::<Self>(
                "Two Lines & Spaced Indent Increase & Operator on Newline & Not Spaced & Error",
                "name\n  :value",
                Parsed::Fail(
                    Error::new("incomplete-named-entry")
                        .prop("key",
                            Parsed::Pass(
                                Token::new()
                                    .name(name::KEY)
                                    .build(0, 3),
                            ))
                        .child(
                            Parsed::Pass(
                                Token::new()
                                    .name(indent::increase::KEY)
                                    .build(4, 6),
                            ))
                        .prop("operator",
                            Parsed::Fail(
                                Error::new(
                                    "missing-expected-trailing-whitespace-in-mutable-field-assigner"
                                ).build(7, 7),
                            ))
                        .build(0, 7),
                )
            ),
            Test::new::<Self>(
                "Two Lines & Spaced Indent Increase & Operator on Newline & Spaced",
                "name\n  : value",
                Parsed::Pass(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key",
                            Token::new()
                                .name(name::KEY)
                                .build(0, 3))
                        .child(
                            Token::new()
                                .name(indent::increase::KEY)
                                .build(4, 6))
                        .prop("operator",
                            Token::new()
                                .name(mutable_field_assigner::KEY)
                                .build(7, 7))
                        .prop("value",
                            Token::new()
                                .name(text::KEY)
                                .build(9, 12))
                        .build(0, 12),
                )
            ),
            Test::new::<Self>(
                "Three Lines & Spaced Indent Increase",
                "name\n  :\n  value",
                Parsed::Pass(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key",
                            Token::new()
                                .name(name::KEY)
                                .build(0, 3))
                        .child(
                            Token::new()
                                .name(indent::increase::KEY)
                                .build(4, 6))
                        .prop("operator",
                            Token::new()
                                .name(mutable_field_assigner::KEY)
                                .build(7, 7))
                        .child(
                            Token::new()
                                .name(indent::current::KEY)
                                .build(8, 10))
                        .prop("value",
                            Token::new()
                                .name(text::KEY)
                                .build(11, 14))
                        .build(0, 14),
                )
            ),
            Test::new::<Self>(
                "Three Lines & Multple Spaced Indent Increases",
                "name\n  :\n    value",
                Parsed::Pass(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key",
                            Token::new()
                                .name(name::KEY)
                                .build(0, 3))
                        .child(
                            Token::new()
                                .name(indent::increase::KEY)
                                .build(4, 6))
                        .prop("operator",
                            Token::new()
                                .name(mutable_field_assigner::KEY)
                                .build(7, 7))
                        .child(
                            Token::new()
                                .name(indent::increase::KEY)
                                .build(8, 12))
                        .prop("value",
                            Token::new()
                                .name(text::KEY)
                                .build(13, 16))
                        .build(0, 16),
                )
            ),
            Test::new::<Self>(
                "Three Lines & Spaced Indent Increase & Spaced Indent Decrease & Ends Early",
                "name\n  :\nvalue",
                Parsed::Pass(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key",
                          Token::new()
                                .name(name::KEY)
                                .build(0, 3))
                        .child(
                          Token::new()
                                .name(indent::increase::KEY)
                                .build(4, 6))
                        .prop("operator",
                          Token::new()
                                .name(mutable_field_assigner::KEY)
                                .build(7, 7))
                        .build(0, 8),
                )
            ),
        ];
    }
}
