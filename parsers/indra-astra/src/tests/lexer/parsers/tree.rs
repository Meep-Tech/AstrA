use crate::{
    lexer::{
        parsers::statement::{
            assignment::entry::named_entry, expression::literal::structure::tree,
        },
        results::parsed::Parsed,
    },
    tests::lexer::parsers::test::{IsFrom, Test},
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
            Parsed::Pass(IsFrom::<tree::Parser>()),
        )]
    }
}
