use crate::lexer::{
    parsers::{mutable_field_assigner, naked_text, name, named_entry},
    results::{builder::Builder, parsed::Parsed, token::Token},
};

use super::test::{IsFrom, Test, Testable};

impl Testable for crate::lexer::parsers::named_entry::Parser {
    fn tests() -> Vec<Test<Self>> {
        return vec![
            Test::<Self>::new(
                "Single Line Spaced Right",
                "name: value",
                Parsed::Token(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key", IsFrom::<name::Parser>())
                        .prop("operator", IsFrom::<mutable_field_assigner::Parser>())
                        .prop("value", IsFrom::<naked_text::Parser>())
                        .build(0, 11),
                ),
            ),
            Test::<Self>::new(
                "Single Line Spaced Around",
                "name : value",
                Parsed::Token(
                    Token::new()
                        .name(named_entry::KEY)
                        .prop("key", IsFrom::<name::Parser>())
                        .prop("operator", IsFrom::<mutable_field_assigner::Parser>())
                        .prop("value", IsFrom::<naked_text::Parser>())
                        .build(0, 12),
                ),
            ),
        ];
    }
}
