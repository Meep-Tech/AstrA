#![allow(incomplete_features)]
#![feature(lazy_cell)]
#![feature(const_hash)]
#![feature(type_name_of_val)]
#![feature(unsized_locals)]
#![feature(iter_intersperse)]

pub mod highlighter;
pub mod lexer;
pub mod tests;
pub mod utils;

//use lexer::parsers::statement::assignment::entry::named_entry;
//use lexer::parsers::statement::expression::invocation::identifier::key::name;
use lexer::parsers::statement::expression::literal::structure::tree;
use lexer::results::end::End;
use lexer::results::parsed::Parsed;
use lexer::results::token::Token;
use lexer::{cursor::Cursor, parsers};

use tests::lexer::parsers::test::{test_parsers, Testable};

fn init() {
    parsers::init_all();
}

fn main() {
    init();
    test_parsers(&[
        //named_entry::Parser::Tests(),
        //name::Parser::Tests(),
        tree::Parser::Tests(),
    ]);
}
