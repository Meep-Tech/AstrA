use crate::{
    lexer::{
        parsers::{
            statement::{assignment::entry::named_entry, expression::literal::structure::tree},
            whitespace::indent,
        },
        results::{parsed::Parsed, token::Token},
    },
    tests::lexer::parsers::test::{Mockable, Test, TokenMocks},
};

use super::test::Testable;

impl Testable for tree::Parser {
    fn get_tests(&self) -> Vec<super::test::Test>
    where
        Self: 'static + Sized + crate::lexer::parser::Parser,
    {
        vec![
            Test::pattern_with_tags::<Self>(
                &["One Entry"],
                "{}",
                &[&named_entry::KEY],
                Parsed::Pass(
                    Token::new()
                        .name(tree::KEY)
                        .child(Token::Mock::<indent::current::Parser>())
                        .child(Token::Mock::<named_entry::Parser>())
                        .mock(),
                ),
            ),
            Test::pattern_with_tags::<Self>(
                &["Two Entries"],
                "{}\n{}",
                &[&named_entry::KEY, &named_entry::KEY],
                Parsed::Pass(
                    Token::new()
                        .name(tree::KEY)
                        .child(Token::Mock::<indent::current::Parser>())
                        .child(Token::Mock::<named_entry::Parser>())
                        .child(Token::Mock::<indent::current::Parser>())
                        .child(Token::Mock::<named_entry::Parser>())
                        .mock(),
                ),
            ),
        ]
    }
}
