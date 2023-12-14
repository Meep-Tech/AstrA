use super::lib::parser::{
    parsers::{
        self,
        attribute::tag,
        statement::{
            assignment::entry::named_entry,
            expression::{invocation::identifier::key::name, literal::structure::tree},
        },
    },
    results::{end::End, parsed::Parsed, token::Token},
};
use tests::parser::parsers::test::{test_parsers, Testable};

fn init() {
    parsers::init_all();
}

fn main() {
    init();
    // test_parsers(&[
    //     named_entry::Parser::Tests(),
    //     name::Parser::Tests(),
    //     tree::Parser::Tests(),
    //     tag::Parser::Tests(),
    // ]);
}
