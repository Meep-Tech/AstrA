use crate::lexer::{
    parsers::{indent, mutable_field_assigner, naked_text, name, named_entry},
    results::{builder::Builder, error::Error, parsed::Parsed, token::Token},
};

use super::test::{IsFrom, Test, Testable};

impl Testable for crate::lexer::parsers::named_entry::Parser {
    fn tests() -> Vec<Test<Self>> {
        return vec![
            Test::<Self>::new(
                "One Line & Spaced Right",
                "name: value",
                Parsed::Token(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key", Token::new()
                            .name(name::KEY)
                            .build(0, 3))
                        .prop("operator", Token::new()
                            .name(mutable_field_assigner::KEY)
                            .build(4, 4))
                        .prop("value", Token::new()
                            .name(naked_text::KEY)
                            .build(6, 9))
                        .build(0, 9),
                ),
            ),
            Test::<Self>::new(
                "One Line & Spaced Around",
                "name : value",
                Parsed::Token(
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
                                .name(naked_text::KEY)
                                .build(7, 10))
                        .build(0, 10),
                ),
            ),
            Test::<Self>::new(
                "One Line & Not-Spaced & Error",
                "name:value",
                Parsed::Error(
                    Error::new("incomplete-named-entry")
                        .prop("key", Parsed::Token(IsFrom::<name::Parser>()))
                        .prop(
                            "operator",
                            Parsed::Error(
                                Error::new(
                                    "missing-expected-trailing-whitespace-in-mutable-field-assigner"
                                ).build(4, 4),
                            ),
                        )
                        .build(0, 4),
                ),
            ),
            Test::<Self>::new(
                "Two Lines & Spaced Indent Increase",
                "name:\n  value",
                Parsed::Token(
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
                                .name(naked_text::KEY)
                                .build(8, 11))
                        .build(0, 11),
                )
            ),
            Test::<Self>::new(
                "Two Lines & Spaced Indent Increase & Operator on Newline & Not Spaced & Error",
                "name\n  :value",
                Parsed::Error(
                    Error::new("incomplete-named-entry")
                        .prop("key",
                            Parsed::Token(
                                Token::new()
                                    .name(name::KEY)
                                    .build(0, 3),
                            ))
                        .child(
                            Parsed::Token(
                                Token::new()
                                    .name(indent::increase::KEY)
                                    .build(4, 6),
                            ))
                        .prop("operator",
                            Parsed::Error(
                                Error::new(
                                    "missing-expected-trailing-whitespace-in-mutable-field-assigner"
                                ).build(7, 7),
                            ))
                        .build(0, 7),
                )
            ),
            Test::<Self>::new(
                "Two Lines & Spaced Indent Increase & Operator on Newline & Spaced",
                "name\n  : value",
                Parsed::Token(
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
                                .name(naked_text::KEY)
                                .build(9, 12))
                        .build(0, 12),
                )
            ),
            Test::<Self>::new(
                "Three Lines & Spaced Indent Increase",
                "name\n  :\n  value",
                Parsed::Token(
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
                                .name(naked_text::KEY)
                                .build(11, 14))
                        .build(0, 14),
                )
            ),
            Test::<Self>::new(
                "Three Lines & Multple Spaced Indent Increases",
                "name\n  :\n    value",
                Parsed::Token(
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
                                .name(naked_text::KEY)
                                .build(13, 16))
                        .build(0, 16),
                )
            ),
            Test::<Self>::new(
                "Three Lines & Spaced Indent Increase & Spaced Indent Decrease & Ends Early",
                "name\n  :\nvalue",
                Parsed::Token(
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
