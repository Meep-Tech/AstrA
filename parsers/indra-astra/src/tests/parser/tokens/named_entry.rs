use std::collections::HashSet;

use crate::parser::{
    results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
    tokens::{
        statement::assignment::entry::named_entry,
        statement::{
            assignment::entry,
            expression::{invocation::identifier::key::name, literal::markup::element::text},
        },
        symbol::operator::assigner::mutable_field_assigner,
        whitespace::indent,
    },
};

use super::test::{Test, Testable, TokenMocks};

impl Testable for named_entry::Parser {
    fn assure_tags(&self) -> Option<HashSet<String>> {
        let mut tags = HashSet::new();
        tags.insert(entry::KEY.to_string());

        return Some(tags);
    }

    fn get_tests(&self) -> Vec<Test> {
        return vec![
            Test::tags::<Self>(
                &["One Line", "Not-Spaced", "Error"],
                "name:value",
                Parsed::Fail(
                    Error::New("incomplete_named_entry")
                        .prop("key", Parsed::Pass(Token::Mock::<name::Parser>()))
                        .prop(
                            "operator",
                            Parsed::Fail(
                                Error::New(
                             "missing_expected_trailing_whitespace_in_mutable_field_assigner"
                                ).build(4, 4),
                            ),
                        )
                        .build(0, 4),
                ),
            ),
            Test::tags::<Self>(
                &["Two Lines", "Spaced Indent Increase"],
                "name:\n  value",
                Parsed::Pass(
                    Token::New()
                        .name(named_entry::KEY)
                        .prop("key",
                            Token::New()
                                .name(name::KEY)
                                .build(0, 3))
                        .prop("operator",
                            Token::New()
                                .name(mutable_field_assigner::KEY)
                                .build(4, 4))
                        .child(
                            Token::New()
                                .name(indent::increase::KEY)
                                .build(5, 7))
                        .prop("value",
                            Token::New()
                                .name(text::KEY)
                                .build(8, 12))
                        .build(0, 12),
                )
            ),
            Test::tags::<Self>(
                &["Two Lines", "Spaced Indent Increase", "Operator on Newline", "Not Spaced", "Error"],
                "name\n  :value",
                Parsed::Fail(
                    Error::New("incomplete_named_entry")
                        .prop("key",
                            Parsed::Pass(
                                Token::New()
                                    .name(name::KEY)
                                    .build(0, 3),
                            ))
                        .child(
                            Parsed::Pass(
                                Token::New()
                                    .name(indent::increase::KEY)
                                    .build(4, 6),
                            ))
                        .prop("operator",
                            Parsed::Fail(
                                Error::New(
                             "missing_expected_trailing_whitespace_in_mutable_field_assigner"
                                ).build(7, 7),
                            ))
                        .build(0, 7),
                )
            ),
            Test::tags::<Self>(
                &["Two Lines", "Spaced Indent Increase", "Operator on Newline", "Spaced"],
                "name\n  : value",
                Parsed::Pass(
                    Token::New()
                        .name(named_entry::KEY)
                        .prop("key",
                            Token::New()
                                .name(name::KEY)
                                .build(0, 3))
                        .child(
                            Token::New()
                                .name(indent::increase::KEY)
                                .build(4, 6))
                        .prop("operator",
                            Token::New()
                                .name(mutable_field_assigner::KEY)
                                .build(7, 7))
                        .prop("value",
                            Token::New()
                                .name(text::KEY)
                                .build(9, 13))
                        .build(0, 13),
                )
            ),
            Test::tags::<Self>(
                &["Three Lines", "Spaced Indent Increase"],
                "name\n  :\n  value",
                Parsed::Pass(
                    Token::New()
                        .name(named_entry::KEY)
                        .prop("key",
                            Token::New()
                                .name(name::KEY)
                                .build(0, 3))
                        .child(
                            Token::New()
                                .name(indent::increase::KEY)
                                .build(4, 6))
                        .prop("operator",
                            Token::New()
                                .name(mutable_field_assigner::KEY)
                                .build(7, 7))
                        .child(
                            Token::New()
                                .name(indent::current::KEY)
                                .build(8, 10))
                        .prop("value",
                            Token::New()
                                .name(text::KEY)
                                .build(11, 15))
                        .build(0, 15),
                )
            ),
            Test::tags::<Self>(
                &["Three Lines", "Multple Spaced Indent Increases"],
                "name\n  :\n    value",
                Parsed::Pass(
                    Token::New()
                        .name(named_entry::KEY)
                        .prop("key",
                            Token::New()
                                .name(name::KEY)
                                .build(0, 3))
                        .child(
                            Token::New()
                                .name(indent::increase::KEY)
                                .build(4, 6))
                        .prop("operator",
                            Token::New()
                                .name(mutable_field_assigner::KEY)
                                .build(7, 7))
                        .child(
                            Token::New()
                                .name(indent::increase::KEY)
                                .build(8, 12))
                        .prop("value",
                            Token::New()
                                .name(text::KEY)
                                .build(13, 17))
                        .build(0, 17),
                )
            ),
            Test::tags::<Self>(
                &["Three Lines", "Spaced Indent Increase", "Spaced Indent Decrease"],
                "name\n  :\nvalue",
                Parsed::Pass(
                    Token::New()
                        .name(named_entry::KEY)
                        .prop("key",
                          Token::New()
                                .name(name::KEY)
                                .build(0, 3))
                        .child(
                          Token::New()
                                .name(indent::increase::KEY)
                                .build(4, 6))
                        .prop("operator",
                          Token::New()
                                .name(mutable_field_assigner::KEY)
                                .build(7, 7))
                        .build(0, 8),
                )
            ).partial(),
        ];
    }
}
