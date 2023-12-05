use crate::{
    lexer::{
        parsers::{
            statement::{assignment::entry::named_entry, expression::literal::structure::tree},
            whitespace::indent,
        },
        results::{parsed::Parsed, token::Token},
    },
    tests::lexer::parsers::test::{IsFrom, Mockable, Test},
};

use super::test::Testable;

impl Testable for tree::Parser {
    fn get_tests(&self) -> Vec<super::test::Test>
    where
        Self: 'static + Sized + crate::lexer::parser::Parser,
    {
        vec![Test::pattern_with_tags::<Self>(
            &["Single Entry"],
            "{}",
            &[&named_entry::KEY],
            Parsed::Pass(
                Token::new()
                    .name(tree::KEY)
                    .child(IsFrom::<indent::current::Parser>())
                    .child(Token::new().name(named_entry::KEY).mock())
                    .mock(),
            ),
        )]
    }
}
